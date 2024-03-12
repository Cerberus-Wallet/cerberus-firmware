from typing import TYPE_CHECKING

import cerberusproto

decode = cerberusproto.decode
encode = cerberusproto.encode
encoded_length = cerberusproto.encoded_length
type_for_name = cerberusproto.type_for_name
type_for_wire = cerberusproto.type_for_wire

if TYPE_CHECKING:
    MessageType = cerberusproto.MessageType


def load_message_buffer(
    buffer: bytes,
    msg_wire_type: int,
    experimental_enabled: bool = True,
) -> MessageType:
    msg_type = type_for_wire(msg_wire_type)
    return decode(buffer, msg_type, experimental_enabled)


def dump_message_buffer(msg: MessageType) -> bytearray:
    buffer = bytearray(encoded_length(msg))
    encode(buffer, msg)
    return buffer
