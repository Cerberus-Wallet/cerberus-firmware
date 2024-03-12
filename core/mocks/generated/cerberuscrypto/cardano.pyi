from typing import *
from cerberuscrypto.bip32 import HDNode


# extmod/modcerberuscrypto/modcerberuscrypto-cardano.h
def derive_icarus(
    mnemonic: str,
    passphrase: str,
    cerberus_derivation: bool,
    callback: Callable[[int, int], None] | None = None,
) -> bytes:
    """
    Derives a Cardano master secret from a mnemonic and passphrase using the
    Icarus derivation scheme.
    If `cerberus_derivation` is True, the Icarus-Cerberus variant is used (see
    CIP-3).
    """


# extmod/modcerberuscrypto/modcerberuscrypto-cardano.h
def from_secret(secret: bytes) -> HDNode:
    """
    Creates a Cardano HD node from a master secret.
    """


# extmod/modcerberuscrypto/modcerberuscrypto-cardano.h
def from_seed_slip23(seed: bytes) -> HDNode:
   """
   Creates a Cardano HD node from a seed via SLIP-23 derivation.
   """


# extmod/modcerberuscrypto/modcerberuscrypto-cardano.h
def from_seed_ledger(seed: bytes) -> HDNode:
    """
    Creates a Cardano HD node from a seed via Ledger derivation.
    """
