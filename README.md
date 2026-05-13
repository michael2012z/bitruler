# bitruler

`bitruler` is a command-line utility for visualizing, decoding, and inspecting
unsigned integers as bits, hexadecimal digits, and common numeric formats.

It prints a compact bit ruler that makes it easier to see where each nibble and
bit position sits inside a value.

## Features

- Accepts unsigned integers up to `u128`.
- Supports hexadecimal, decimal, octal, and binary input.
- Allows underscores as digit separators.
- Shows a visual unit ruler, hexadecimal digit art, bit groups, and bit positions.
- Prints decoded `HEX`, `DEC`, `OCT`, `BIN`, and printable ASCII information.

## Installation

Build from source with Cargo:

```sh
cargo build --release
```

The binary will be available at:

```sh
target/release/bitruler
```

For local development, run it directly with:

```sh
cargo run -- 0x1234
```

## Usage

```text
bitruler <unsigned-number>
bitruler --help
bitruler --version
```

Accepted input formats:

```text
Hexadecimal  0x1234
Decimal      4660
Octal        0o11064
Binary       0b0001_0010_0011_0100
```

Notes:

- Underscores are allowed as digit separators.
- The maximum value is `340282366920938463463374607431768211455`.
- The maximum hexadecimal value is `0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff`.

## Example

Command:

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
O       │    │    │    │      │    │    │    │      │    │    │    │
S      44   40   36   32     28   24   20   16     12    8    4    0

HEX: 0x1234_5678_9abc
DEC: 20015998343868
OCT: 0o443212636115274
BIN: 0b0001_0010_0011_0100_0101_0110_0111_1000_1001_1010_1011_1100
ASC: .4Vx..
```

## License

This project is licensed under the MIT License. See `LICENSE` for details.
