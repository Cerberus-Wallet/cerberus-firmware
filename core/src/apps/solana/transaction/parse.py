from typing import TYPE_CHECKING

from trezor.wire import DataError

from apps.common.readers import read_uint32_le, read_uint64_le

if TYPE_CHECKING:
    from trezor.utils import BufferReader


def parse_var_int(serialized_tx: BufferReader) -> int:
    value = 0
    shift = 0
    while(serialized_tx.remaining_count()):
        B = serialized_tx.get()
        value += (B & 0b01111111) << shift
        shift += 7
        if B & 0b10000000 == 0:
            return value
    

def parse_block_hash(serialized_tx: BufferReader) -> bytes:
    return bytes(serialized_tx.read_memoryview(32))


def parse_pubkey(serialized_tx: BufferReader) -> bytes:
    return bytes(serialized_tx.read_memoryview(32))


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


def parse_property(
    reader: BufferReader, type: str, is_optional: bool = False
) -> str | int | bytes:
    if type == "u8":
        return reader.get()
    elif type == "u32":
        return read_uint32_le(reader)
    elif type == "u64":
        return read_uint64_le(reader)
    elif type == "i32":
        return read_uint32_le(reader)
    elif type in ("i64", "unix_timestamp", "lamports", "token_amount"):
        return read_uint64_le(reader)
    elif type in ("pubkey", "authority"):
        if is_optional:
            is_included = reader.get()
            if is_included == 0:
                return None

        return parse_pubkey(reader)
    elif type == "enum":
        return parse_enum(reader)
    elif type == "string":
        return parse_string(reader)
    elif type == "memo":
        return parse_memo(reader)
    else:
        from .instructions import enum_type_to_class

        int_value = parse_property(reader, enum_type_to_class(type).type())
        assert isinstance(int_value, int)
        return enum_type_to_class(type).from_int(int_value)
