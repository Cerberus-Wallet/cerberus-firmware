from typing import TYPE_CHECKING

from apps.common.readers import read_uint64_le

if TYPE_CHECKING:
    from trezor.utils import BufferReader


def parse_var_int(serialized_tx: BufferReader) -> int:
    value = 0
    shift = 0
    while serialized_tx.remaining_count():
        B = serialized_tx.get()
        value += (B & 0b01111111) << shift
        shift += 7
        if B & 0b10000000 == 0:
            return value
    return value


def parse_block_hash(serialized_tx: BufferReader) -> bytes:
    return bytes(serialized_tx.read_memoryview(32))


def parse_pubkey(serialized_tx: BufferReader) -> bytes:
    return bytes(serialized_tx.read_memoryview(32))


def parse_optional_pubkey(serialized_tx: BufferReader) -> bytes | None:
    is_included = serialized_tx.get()
    if is_included == 0:
        return None

    return parse_pubkey(serialized_tx)


def parse_enum(serialized_tx: BufferReader) -> int:
    return serialized_tx.get()


def parse_string(serialized_tx: BufferReader) -> str:
    # TODO SOL: validation shall be checked (length is less than 2^32 or even less)
    length = read_uint64_le(serialized_tx)
    return bytes(serialized_tx.read_memoryview(length)).decode("utf-8")


def parse_memo(serialized_tx: BufferReader) -> str:
    return bytes(serialized_tx.read_memoryview(serialized_tx.remaining_count())).decode(
        "utf-8"
    )


def parse_byte(serialized_tx: BufferReader) -> int:
    return serialized_tx.get()
