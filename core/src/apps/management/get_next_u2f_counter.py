from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from cerberus.messages import GetNextU2FCounter, NextU2FCounter


async def get_next_u2f_counter(msg: GetNextU2FCounter) -> NextU2FCounter:
    import storage.device as storage_device
    from cerberus import TR
    from cerberus.enums import ButtonRequestType
    from cerberus.messages import NextU2FCounter
    from cerberus.ui.layouts import confirm_action
    from cerberus.wire import NotInitialized

    if not storage_device.is_initialized():
        raise NotInitialized("Device is not initialized")

    await confirm_action(
        "get_u2f_counter",
        TR.u2f__title_get,
        description=TR.u2f__get,
        br_code=ButtonRequestType.ProtectCall,
    )

    return NextU2FCounter(u2f_counter=storage_device.next_u2f_counter())
