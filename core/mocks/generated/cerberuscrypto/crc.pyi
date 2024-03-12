from typing import *


# extmod/modcerberuscrypto/modcerberuscrypto-crc.h
def crc32(data: bytes, crc: int = 0) -> int:
    """
    Computes a CRC32 checksum of `data`.
    """
