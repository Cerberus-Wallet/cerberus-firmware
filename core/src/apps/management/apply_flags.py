from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from cerberus.messages import ApplyFlags, Success


async def apply_flags(msg: ApplyFlags) -> Success:
    import storage.device
    from storage.device import set_flags
    from cerberus.messages import Success
    from cerberus.wire import NotInitialized

    if not storage.device.is_initialized():
        raise NotInitialized("Device is not initialized")
    set_flags(msg.flags)
    return Success(message="Flags applied")
