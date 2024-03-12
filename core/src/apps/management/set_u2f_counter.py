from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from cerberus.messages import SetU2FCounter, Success


async def set_u2f_counter(msg: SetU2FCounter) -> Success:
    import storage.device as storage_device
    from cerberus import TR, wire
    from cerberus.enums import ButtonRequestType
    from cerberus.messages import Success
    from cerberus.ui.layouts import confirm_action

    if not storage_device.is_initialized():
        raise wire.NotInitialized("Device is not initialized")
    if msg.u2f_counter is None:
        raise wire.ProcessError("No value provided")

    await confirm_action(
        "set_u2f_counter",
        TR.u2f__title_set,
        description=TR.u2f__set_template,
        description_param=str(msg.u2f_counter),
        verb=TR.buttons__set,
        br_code=ButtonRequestType.ProtectCall,
    )

    storage_device.set_u2f_counter(msg.u2f_counter)

    return Success(message="U2F counter set")
