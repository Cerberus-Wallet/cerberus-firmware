from typing import TYPE_CHECKING

from trezor.crypto import base58
from trezor.utils import BufferReader
from trezor.wire import DataError

from .instruction import Instruction
from .instructions import get_instruction, get_instruction_id_length
from .parse import (
    parse_address_lookup_tables,
    parse_addresses,
    parse_block_hash,
    parse_header,
    parse_instructions,
)

if TYPE_CHECKING:
    from ..types import Account, Address, AddressReference, RawInstruction


class Transaction:
    blind_signing = False
    required_signers_count = 0

    version: int | None = None

    addresses: list[Address]

    blockhash: bytes

    raw_instructions: list[RawInstruction]
    instructions: list[Instruction]

    address_lookup_tables_rw_addresses: list[AddressReference]
    address_lookup_tables_ro_addresses: list[AddressReference]

    def __init__(self, serialized_tx: bytes) -> None:
        self.instructions = []
        self.address_lookup_tables_rw_addresses = []
        self.address_lookup_tables_ro_addresses = []
        self._parse_transaction(serialized_tx)
        self._create_instructions()
        self._determine_if_blind_signing()

    def _parse_transaction(self, serialized_tx: bytes) -> None:
        serialized_tx_reader = BufferReader(serialized_tx)
        (
            self.version,
            num_required_signatures,
            num_signature_read_only_addresses,
            num_read_only_addresses,
        ) = parse_header(serialized_tx_reader)

        self.required_signers_count = num_required_signatures

        self.addresses = parse_addresses(
            serialized_tx_reader,
            num_required_signatures,
            num_signature_read_only_addresses,
            num_read_only_addresses,
        )

        self.blockhash = parse_block_hash(serialized_tx_reader)

        self.raw_instructions = parse_instructions(
            self.addresses, get_instruction_id_length, serialized_tx_reader
        )

        if self.version is not None:
            (
                self.address_lookup_tables_rw_addresses,
                self.address_lookup_tables_ro_addresses,
            ) = parse_address_lookup_tables(serialized_tx_reader)

        if serialized_tx_reader.remaining_count() != 0:
            raise DataError("Invalid transaction")

    def _get_combined_accounts(self) -> list[Account]:
        """
        Combine accounts from transaction's accounts field with accounts from address lookup tables.
        Instructions reference accounts by index in this combined list.
        """
        accounts: list[Account] = []
        for address in self.addresses:
            accounts.append(address)

        for rw_address in self.address_lookup_tables_rw_addresses:
            accounts.append(rw_address)
        for ro_address in self.address_lookup_tables_ro_addresses:
            accounts.append(ro_address)

        return accounts

    def _create_instructions(self) -> None:
        combined_accounts = self._get_combined_accounts()

        for (
            program_index,
            instruction_id,
            accounts,
            instruction_data,
        ) in self.raw_instructions:
            program_id = base58.encode(self.addresses[program_index][0])
            instruction_accounts = [
                combined_accounts[account_index] for account_index in accounts
            ]
            instruction = get_instruction(
                program_id,
                instruction_id,
                instruction_accounts,
                instruction_data,
            )

            self.instructions.append(instruction)

    def _determine_if_blind_signing(self) -> None:
        for instruction in self.instructions:
            if (
                not instruction.is_program_supported
                or not instruction.is_instruction_supported
            ):
                self.blind_signing = True
                break
