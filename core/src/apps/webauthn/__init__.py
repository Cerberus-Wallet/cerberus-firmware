def boot() -> None:
    import usb
    from cerberus import loop

    from .fido2 import handle_reports

    loop.schedule(handle_reports(usb.iface_webauthn))
