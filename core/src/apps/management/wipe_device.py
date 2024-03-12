from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from cerberus.messages import Success, WipeDevice


async def wipe_device(msg: WipeDevice) -> Success:
    import storage
    from cerberus import TR, translations
    from cerberus.enums import ButtonRequestType
    from cerberus.messages import Success
    from cerberus.ui.layouts import confirm_action

    from apps.base import reload_settings_from_storage

    await confirm_action(
        "confirm_wipe",
        TR.wipe__title,
        TR.wipe__info,
        TR.wipe__want_to_wipe,
        reverse=True,
        verb=TR.buttons__hold_to_confirm,
        hold=True,
        hold_danger=True,
        br_code=ButtonRequestType.WipeDevice,
    )

    # wipe storage
    storage.wipe()
    # erase translations
    translations.deinit()
    translations.erase()

    # reload settings
    reload_settings_from_storage()

    return Success(message="Device wiped")
