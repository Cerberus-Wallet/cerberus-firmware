from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from cerberus.messages import FirmwareHash, GetFirmwareHash
    from cerberus.ui.layouts.common import ProgressLayout

_progress_obj: ProgressLayout | None = None


async def get_firmware_hash(msg: GetFirmwareHash) -> FirmwareHash:
    from cerberus import wire, workflow
    from cerberus.messages import FirmwareHash
    from cerberus.ui.layouts.progress import progress
    from cerberus.utils import firmware_hash

    workflow.close_others()
    global _progress_obj
    _progress_obj = progress()

    try:
        hash = firmware_hash(msg.challenge, _render_progress)
    except ValueError as e:
        raise wire.DataError(str(e))
    finally:
        _progress_obj = None

    return FirmwareHash(hash=hash)


def _render_progress(progress: int, total: int) -> None:
    global _progress_obj
    if _progress_obj is not None:
        _progress_obj.report(1000 * progress // total)
