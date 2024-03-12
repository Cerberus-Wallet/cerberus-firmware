from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from cerberus.messages import Entropy, GetEntropy


async def get_entropy(msg: GetEntropy) -> Entropy:
    from cerberus import TR
    from cerberus.crypto import random
    from cerberus.enums import ButtonRequestType
    from cerberus.messages import Entropy
    from cerberus.ui.layouts import confirm_action

    await confirm_action(
        "get_entropy",
        TR.entropy__title_confirm,
        TR.entropy__send,
        TR.words__know_what_your_doing,
        br_code=ButtonRequestType.ProtectCall,
    )

    size = min(msg.size, 1024)
    entropy = random.bytes(size, True)

    return Entropy(entropy=entropy)
