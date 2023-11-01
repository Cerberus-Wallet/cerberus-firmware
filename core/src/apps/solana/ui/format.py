from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from ..transaction.instructions import Instruction


def format_pubkey(_: Instruction, value: bytes | None) -> str:
    from trezor.crypto import base58

    if value is None:
        raise ValueError  # should not be called with optional pubkey

    return base58.encode(value)


def format_lamports(_: Instruction, value: int) -> str:
    from trezor.strings import format_amount

    formatted = format_amount(value, decimals=9)
    return f"{formatted} SOL"


def format_token_amount(instruction: Instruction, value: int) -> str:
    from trezor.strings import format_amount

    decimals = instruction.decimals if instruction.decimals is not None else 0
    formatted = format_amount(value, decimals=decimals)
    return f"{formatted}"


def format_unix_timestamp(_: Instruction, value: int) -> str:
    from trezor.strings import format_timestamp

    return format_timestamp(value)


def format_int(_: Instruction, value: int) -> str:
    return str(value)


def format_identity(_: Instruction, value: str) -> str:
    return value
