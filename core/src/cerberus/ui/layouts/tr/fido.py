import cerberusui2
from cerberus.enums import ButtonRequestType

from ..common import interact
from . import RustLayout


async def confirm_fido(
    header: str,
    app_name: str,
    icon_name: str | None,
    accounts: list[str | None],
) -> int:
    """Webauthn confirmation for one or more credentials."""
    confirm = RustLayout(
        cerberusui2.confirm_fido(  # type: ignore [Argument missing for parameter "icon_name"]
            title=header.upper(),
            app_name=app_name,
            accounts=accounts,
        )
    )
    result = await interact(confirm, "confirm_fido", ButtonRequestType.Other)

    # The Rust side returns either an int or `CANCELLED`. We detect the int situation
    # and assume cancellation otherwise.
    if isinstance(result, int):
        return result

    # For the usage in device tests, assuming CONFIRMED (sent by debuglink)
    # is choosing the first credential.
    if __debug__ and result is cerberusui2.CONFIRMED:
        return 0

    # Late import won't get executed on the happy path.
    from cerberus.wire import ActionCancelled

    raise ActionCancelled


async def confirm_fido_reset() -> bool:
    from cerberus import TR

    confirm = RustLayout(
        cerberusui2.confirm_action(
            title=TR.fido__title_reset,
            description=TR.fido__wanna_erase_credentials,
            action=None,
            verb_cancel="",
            verb=TR.buttons__confirm,
        )
    )
    return (await confirm) is cerberusui2.CONFIRMED
