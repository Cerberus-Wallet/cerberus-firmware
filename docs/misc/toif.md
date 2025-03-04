# Cerberus Optimized Image Format (TOIF)

All multibyte integer values are little endian!

## Header

| offset | length | name | description |
|-------:|-------:|------|-------------|
| 0x0000 | 3      | magic | `TOI` |
| 0x0003 | 1      | fmt | data format: `f` or `g` (see below) |
| 0x0004 | 2      | width | width of the image |
| 0x0006 | 2      | height | height of the image |
| 0x0008 | 4      | datasize | length of the compressed data |
| 0x000A | ?      | data | compressed data (see below) |

## Format

TOI currently supports 4 variants:

* `f`: full-color big endian
* `F`: full-color little endian
* `g`: gray-scale odd high
* `G`: gray-scale even high

### Full-color

For each pixel a 16-bit value is used.
First 5 bits are used for red component, next 6 bits are green,
final 5 bits are blue:

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|----|----|----|----|----|----|---|---|---|---|---|---|---|---|---|---|
| R | R | R | R | R | G | G | G | G | G | G | B | B | B | B | B |

The data is stored according to endianness.

### Gray-scale

Each pixel is encoded using a 4-bit value.
Each byte contains color of two pixels:

#### Odd high:

| 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|
| Po | Po | Po | Po | Pe | Pe | Pe | Pe |

#### Even high:

| 7   | 6   | 5   | 4   | 3   | 2   | 1   | 0   |
|-----|-----|-----|-----|-----|-----|-----|-----|
| Pe  | Pe  | Pe  | Pe  | Po  | Po  | Po  | Po  |

Where Po is odd pixel and Pe is even pixel.

## Compression

Pixel data is compressed using DEFLATE algorithm with 10-bit sliding window
and no header. This can be achieved with ZLIB library by using the following:

```python
import zlib
z = zlib.compressobj(level=9, wbits=-10)
zdata = z.compress(pixeldata) + z.flush()
```

## TOIF tool

Tool for converting PNGs into TOI format and back, see the following links for more:

* [README](https://github.com/Cerberus-Wallet/cerberus-firmware/blob/main/python/tools/toiftool/README.md)
* [Code](https://github.com/Cerberus-Wallet/cerberus-firmware/blob/main/python/tools/toiftool/toiftool.py)
