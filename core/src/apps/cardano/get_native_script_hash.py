from typing import TYPE_CHECKING

from . import seed

if TYPE_CHECKING:
    from cerberus.messages import CardanoGetNativeScriptHash, CardanoNativeScriptHash


@seed.with_keychain
async def get_native_script_hash(
    msg: CardanoGetNativeScriptHash, keychain: seed.Keychain
) -> CardanoNativeScriptHash:
    from cerberus.enums import CardanoNativeScriptHashDisplayFormat
    from cerberus.messages import CardanoNativeScriptHash

    from . import layout, native_script

    native_script.validate_native_script(msg.script)

    script_hash = native_script.get_native_script_hash(keychain, msg.script)

    if msg.display_format != CardanoNativeScriptHashDisplayFormat.HIDE:
        await layout.show_native_script(msg.script)
        await layout.show_script_hash(script_hash, msg.display_format)

    return CardanoNativeScriptHash(script_hash=script_hash)
