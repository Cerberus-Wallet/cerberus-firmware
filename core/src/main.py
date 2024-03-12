# isort:skip_file
# fmt: off

# We are tightly controlling the memory layout. Order of imports is important.
# Modules imported directly from here also must take care to have as few dependencies
# as possible.

# === Import always-active modules
# cerberus imports only C modules
import cerberus
# cerberus.utils import only C modules
from cerberus import utils
# we need space for 30 items in the cerberus module
utils.presize_module("cerberus", 30)

# storage imports storage.common, storage.cache and storage.device.
# These import cerberus, cerberus.config (which is a C module), cerberus.utils, and each other.
import storage
# we will need space for 12 items in the storage module
utils.presize_module("storage", 12)

if not utils.BITCOIN_ONLY:
    # storage.fido2 only imports C modules
    import storage.fido2  # noqa: F401

if __debug__:
    # storage.debug only imports C modules
    import storage.debug

# cerberus.pin imports cerberus.utils
# We need it as an always-active module because cerberus.pin.show_pin_timeout is used
# as a UI callback for storage, which can be invoked at any time
import cerberus.pin  # noqa: F401

# === Prepare the USB interfaces first. Do not connect to the host yet.
# usb imports cerberus.utils and cerberus.uraanai.com which is a C module
import usb

# create an unimport manager that will be reused in the main loop
unimport_manager = utils.unimport()

# unlock the device, unload the boot module afterwards
with unimport_manager:
    import boot
    del boot

# start the USB
import storage.device

usb.bus.open(storage.device.get_device_id())

# run the endless loop
while True:
    with unimport_manager:
        import session  # noqa: F401
        del session
