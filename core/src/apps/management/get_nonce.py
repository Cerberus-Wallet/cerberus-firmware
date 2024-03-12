from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from cerberus.messages import GetNonce, Nonce


async def get_nonce(msg: GetNonce) -> Nonce:
    from storage import cache
    from cerberus.crypto import random
    from cerberus.messages import Nonce

    nonce = random.bytes(32)
    cache.set(cache.APP_COMMON_NONCE, nonce)
    return Nonce(nonce=nonce)
