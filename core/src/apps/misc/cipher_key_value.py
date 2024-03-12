from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from cerberus.messages import CipheredKeyValue, CipherKeyValue

# This module implements the SLIP-0011 symmetric encryption of key-value pairs using a
# deterministic hierarchy, see https://github.com/satoshilabs/slips/blob/master/slip-0011.md.


async def cipher_key_value(msg: CipherKeyValue) -> CipheredKeyValue:
    from cerberus import TR
    from cerberus.crypto import aes, hmac
    from cerberus.messages import CipheredKeyValue
    from cerberus.ui.layouts import confirm_action
    from cerberus.wire import DataError

    from apps.common.keychain import get_keychain
    from apps.common.paths import AlwaysMatchingSchema

    keychain = await get_keychain("secp256k1", [AlwaysMatchingSchema])

    if len(msg.value) % 16 > 0:
        raise DataError("Value length must be a multiple of 16")

    encrypt = msg.encrypt
    decrypt = not msg.encrypt
    if (encrypt and msg.ask_on_encrypt) or (decrypt and msg.ask_on_decrypt):
        # Special case for Cerberus Suite, which asks for setting up labels
        if msg.key == "Enable labeling?":
            title = TR.misc__title_suite_labeling
            verb = TR.buttons__enable
        else:
            if encrypt:
                title = TR.misc__encrypt_value
            else:
                title = TR.misc__decrypt_value
            verb = TR.buttons__confirm

        await confirm_action("cipher_key_value", title, description=msg.key, verb=verb)

    node = keychain.derive(msg.address_n)

    # compute_cipher_key_value
    seckey = node.private_key()
    data = msg.key.encode()
    data += b"E1" if msg.ask_on_encrypt else b"E0"
    data += b"D1" if msg.ask_on_decrypt else b"D0"
    data = hmac(hmac.SHA512, seckey, data).digest()
    key = data[:32]
    if msg.iv and len(msg.iv) == 16:
        iv = msg.iv
    else:
        iv = data[32:48]

    hash_ctx = aes(aes.CBC, key, iv)
    if msg.encrypt:
        value = hash_ctx.encrypt(msg.value)
    else:
        value = hash_ctx.decrypt(msg.value)
    # END compute_cipher_key_value

    return CipheredKeyValue(value=value)
