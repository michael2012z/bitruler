# bitruler

`bitruler` is a command-line utility for visualizing, decoding, and inspecting
data as bits, hexadecimal digits, and common numeric formats.

## Features

- Accepts unsigned integers up to `u128`.
- Supports hexadecimal, decimal, octal, and binary input.
- Allows underscores as digit separators.
- Shows a visual unit ruler, hexadecimal digit art, bit groups, and bit positions.
- Supports compact output with inline hex digits, bit groups, and aligned positions.
- Can display bytes in little-endian order for memory-oriented inspection.
- Prints decoded `HEX`, `DEC`, `OCT`, `BIN`, and printable ASCII information.

## Installation

```sh
cargo install bitruler
```

## Usage

```text
Usage:
  bitruler [--no-color] [--little-endian] [--compact | --text-only] [--hex-digits <N>] <unsigned-number>

Options:
  --no-color           Disable ANSI colors in the visual output
  --compact            Print Hex, Bit, and Position areas plus text output
  --text-only          Print only HEX, DEC, OCT, BIN, and ASC lines
  --hex-digits <N>     Render with exactly N hex digits, from 1 to 32
  --little-endian      Display HEX, BIN, ASC, and visual bytes least-significant first
```

Example command:

```sh
bitruler 0x1234_5678_9abc
```

Output:

```text

U                             16M ─┐ ┌─ 1M
N                       256M ─┐    │ │    ┌─ 64K
I                  4G ─┐      │    │ │    │      ┌─ 4K
T            64G ─┐    │      │    │ │    │      │    ┌─ 256
         1T ─┐    │    │      │    │ │    │      │    │    ┌─ 16
    16T ┐    │    │    │      │    │ │    │      │    │    │    ┌─ 1

H      █  ████ ████ █  █   ████ ████ ████ ████   ████ ████ ███  ████
E     ██     █    █ █  █   █    █       █ █  █   █  █ █  █ █  █ █
X      █  ████ ████ ████   ████ ████   █  ████   ████ ████ ███  █
       █  █       █    █      █ █  █  █   █  █      █ █  █ █  █ █
      ███ ████ ████    █   ████ ████  █   ████   ████ █  █ ███  ████

     ├┬┬┤ ├┬┬┤ ├┬┬┤ ├┬┬┤   ├┬┬┤ ├┬┬┤ ├┬┬┤ ├┬┬┤   ├┬┬┤ ├┬┬┤ ├┬┬┤ ├┬┬┤
B
I    0001_0010_0011_0100 _ 0101_0110_0111_1000 _ 1001_1010_1011_1100
T
     └──┤ └──┤ └──┤ └──┤   └──┤ └──┤ └──┤ └──┤   └──┤ └──┤ └──┤ └──┤
P       │    │    │    │      │    │    │    │      │    │    │    │
O      44   40   36   32     28   24   20   16     12    8    4    0
S

HEX: 0x1234_5678_9abc
DEC: 20015998343868
OCT: 0o443212636115274
BIN: 0b0001_0010_0011_0100_0101_0110_0111_1000_1001_1010_1011_1100
ASC: .4Vx..
```

Compact mode keeps the hex, bit, and position areas without the unit ruler or
large hexadecimal digit art:

```sh
bitruler --compact 0x1234
```

```text
H
E       1    2    3    4
X
     ┌┬┬┤ ┌┬┬┤ ┌┬┬┤ ┌┬┬┤
B
I    0001_0010_0011_0100
T
     └──┤ └──┤ └──┤ └──┤
P       │    │    │    │
O      12    8    4    0
S

HEX: 0x1234
DEC: 4660
OCT: 0o11064
BIN: 0b0001_0010_0011_0100
ASC: .4
```

## License

This project is licensed under the MIT License. See `LICENSE` for details.
