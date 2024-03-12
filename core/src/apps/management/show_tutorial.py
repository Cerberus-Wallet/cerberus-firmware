from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from cerberus.messages import ShowDeviceTutorial, Success


async def show_tutorial(msg: ShowDeviceTutorial) -> Success:
    from cerberus.messages import Success

    # NOTE: tutorial is defined only for TR, and this function should
    # also be called only in case of TR
    from cerberus.ui.layouts import tutorial

    await tutorial()

    return Success(message="Tutorial shown")
