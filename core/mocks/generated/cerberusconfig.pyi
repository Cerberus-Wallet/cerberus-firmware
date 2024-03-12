from typing import *


# extmod/modcerberusconfig/modcerberusconfig.c
def init(
   ui_wait_callback: Callable[[int, int, str], bool] | None = None
) -> None:
    """
    Initializes the storage.  Must be called before any other method is
    called from this module!
    """


# extmod/modcerberusconfig/modcerberusconfig.c
def unlock(pin: str, ext_salt: bytes | None) -> bool:
    """
    Attempts to unlock the storage with the given PIN and external salt.
    Returns True on success, False on failure.
    """


# extmod/modcerberusconfig/modcerberusconfig.c
def check_pin(pin: str, ext_salt: bytes | None) -> bool:
    """
    Check the given PIN with the given external salt.
    Returns True on success, False on failure.
    """


# extmod/modcerberusconfig/modcerberusconfig.c
def lock() -> None:
    """
    Locks the storage.
    """


# extmod/modcerberusconfig/modcerberusconfig.c
def is_unlocked() -> bool:
    """
    Returns True if storage is unlocked, False otherwise.
    """


# extmod/modcerberusconfig/modcerberusconfig.c
def has_pin() -> bool:
    """
    Returns True if storage has a configured PIN, False otherwise.
    """


# extmod/modcerberusconfig/modcerberusconfig.c
def get_pin_rem() -> int:
    """
    Returns the number of remaining PIN entry attempts.
    """


# extmod/modcerberusconfig/modcerberusconfig.c
def change_pin(
    oldpin: str,
    newpin: str,
    old_ext_salt: bytes | None,
    new_ext_salt: bytes | None,
) -> bool:
    """
    Change PIN and external salt. Returns True on success, False on failure.
    """


# extmod/modcerberusconfig/modcerberusconfig.c
def ensure_not_wipe_code(pin: str) -> None:
    """
    Wipes the device if the entered PIN is the wipe code.
    """


# extmod/modcerberusconfig/modcerberusconfig.c
def has_wipe_code() -> bool:
    """
    Returns True if storage has a configured wipe code, False otherwise.
    """


# extmod/modcerberusconfig/modcerberusconfig.c
def change_wipe_code(
    pin: str,
    ext_salt: bytes | None,
    wipe_code: str,
) -> bool:
    """
    Change wipe code. Returns True on success, False on failure.
    """


# extmod/modcerberusconfig/modcerberusconfig.c
def get(app: int, key: int, public: bool = False) -> bytes | None:
    """
    Gets the value of the given key for the given app (or None if not set).
    Raises a RuntimeError if decryption or authentication of the stored
    value fails.
    """


# extmod/modcerberusconfig/modcerberusconfig.c
def set(app: int, key: int, value: bytes, public: bool = False) -> None:
    """
    Sets a value of given key for given app.
    """


# extmod/modcerberusconfig/modcerberusconfig.c
def delete(
    app: int, key: int, public: bool = False, writable_locked: bool = False
) -> bool:
    """
    Deletes the given key of the given app.
    """


# extmod/modcerberusconfig/modcerberusconfig.c
def set_counter(
    app: int, key: int, count: int, writable_locked: bool = False
) -> None:
    """
    Sets the given key of the given app as a counter with the given value.
    """


# extmod/modcerberusconfig/modcerberusconfig.c
def next_counter(
   app: int, key: int, writable_locked: bool = False,
) -> int:
    """
    Increments the counter stored under the given key of the given app and
    returns the new value.
    """


# extmod/modcerberusconfig/modcerberusconfig.c
def wipe() -> None:
    """
    Erases the whole config. Use with caution!
    """
