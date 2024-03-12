from typing import *


# extmod/modcerberuscrypto/modcerberuscrypto-optiga.h
class OptigaError(Exception):
    """Error returned by the Optiga chip."""


# extmod/modcerberuscrypto/modcerberuscrypto-optiga.h
class SigningInaccessible(OptigaError):
    """The signing key is inaccessible.
    Typically, this will happen after the bootloader has been unlocked.
    """


# extmod/modcerberuscrypto/modcerberuscrypto-optiga.h
def get_certificate(cert_index: int) -> bytes:
    """
    Return the certificate stored at the given index.
    """


# extmod/modcerberuscrypto/modcerberuscrypto-optiga.h
def sign(
    key_index: int,
    digest: bytes,
) -> bytes:
    """
    Uses the private key at key_index to produce a DER-encoded signature of
    the digest.
    """
DEVICE_CERT_INDEX: int
DEVICE_ECC_KEY_INDEX: int
