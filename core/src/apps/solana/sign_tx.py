from typing import TYPE_CHECKING

from apps.common.keychain import with_slip44_keychain

from . import CURVE, PATTERNS, SLIP44_ID
from .transaction import Transaction

if TYPE_CHECKING:
    from trezor.messages import SolanaSignTx, SolanaTxSignature

    from apps.common.keychain import Keychain


@with_slip44_keychain(*PATTERNS, slip44_id=SLIP44_ID, curve=CURVE)
async def sign_tx(
    msg: SolanaSignTx,
    keychain: Keychain,
) -> SolanaTxSignature:
    from trezor.crypto import base58
    from trezor.crypto.curve import ed25519
    from trezor.enums import ButtonRequestType
    from trezor.messages import SolanaTxSignature
    from trezor.ui.layouts import confirm_metadata, show_warning

    from apps.common import seed

    from .ui import show_final_confirmation

    address_n = msg.address_n  # local_cache_attribute
    serialized_tx = msg.serialized_tx  # local_cache_attribute

    node = keychain.derive(address_n)
    signer_public_key = seed.remove_ed25519_prefix(node.public_key())

    transaction: Transaction = Transaction(serialized_tx)

    if transaction.blind_signing:
        await show_warning(
            "warning_blind_signing", "Transaction contains unknown instructions."
        )

    if transaction.required_signers_count > 1:
        await confirm_metadata(
            "multiple_signers",
            "Multiple signers",
            f"Transaction requires {transaction.required_signers_count} signers which increases the fee.",
            br_code=ButtonRequestType.Other,
        )

    await show_instructions(address_n, signer_public_key, transaction)

    signer_address = base58.encode(seed.remove_ed25519_prefix(node.public_key()))

    await show_final_confirmation(
        address_n,
        signer_address,
        transaction.blockhash,
        calculate_fee(transaction),
    )

    signature = ed25519.sign(node.private_key(), serialized_tx)

    return SolanaTxSignature(signature=signature)


async def show_instructions(
    signer_path: list[int], signer_public_key: bytes, transaction: Transaction
) -> None:
    instructions_count = len(transaction.instructions)
    for instruction_index, instruction in enumerate(transaction.instructions, 1):
        if not instruction.is_program_supported:
            from .ui import show_unsupported_program_confirm

            await show_unsupported_program_confirm(
                instruction,
                instructions_count,
                instruction_index,
                signer_path,
                signer_public_key,
            )
        elif not instruction.is_instruction_supported:
            from .ui import show_unsupported_instruction_confirm

            await show_unsupported_instruction_confirm(
                instruction,
                instructions_count,
                instruction_index,
                signer_path,
                signer_public_key,
            )
        else:
            from .ui import show_confirm

            await show_confirm(
                instruction,
                instructions_count,
                instruction_index,
                signer_path,
                signer_public_key,
            )


def calculate_fee(transaction: Transaction) -> int:
    from .types import AddressType
    from .constants import (
        SOLANA_BASE_FEE_LAMPORTS,
        SOLANA_COMPUTE_UNIT_LIMIT,
    )
    from .transaction.instructions import (
        COMPUTE_BUDGET_PROGRAM_ID,
        COMPUTE_BUDGET_PROGRAM_ID_INS_SET_COMPUTE_UNIT_LIMIT,
        COMPUTE_BUDGET_PROGRAM_ID_INS_SET_COMPUTE_UNIT_PRICE,
    )

    number_of_signers = 0
    for address in transaction.addresses:
        if address[1] == AddressType.AddressSig:
            number_of_signers += 1

    base_fee = SOLANA_BASE_FEE_LAMPORTS * number_of_signers

    unit_price = 0
    is_unit_price_set = False
    unit_limit = SOLANA_COMPUTE_UNIT_LIMIT
    is_unit_limit_set = False

    for instruction in transaction.instructions[:3]:
        if instruction.program_id == COMPUTE_BUDGET_PROGRAM_ID:
            if (
                instruction.instruction_id
                == COMPUTE_BUDGET_PROGRAM_ID_INS_SET_COMPUTE_UNIT_LIMIT
                and not is_unit_limit_set
            ):
                unit_limit = instruction.units
                is_unit_limit_set = True
            elif (
                instruction.instruction_id
                == COMPUTE_BUDGET_PROGRAM_ID_INS_SET_COMPUTE_UNIT_PRICE
                and not is_unit_price_set
            ):
                unit_price = instruction.lamports
                is_unit_price_set = True

    return int(base_fee + unit_price * unit_limit / 1000000)
