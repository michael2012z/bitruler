// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

//! Text area formatting: HEX, DEC, OCT, BIN, and ASC lines.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Endian {
    Big,
    Little,
}

pub(super) fn format_lines(number: u128, hex_digits: Option<usize>, endian: Endian) -> Vec<String> {
    vec![
        format!("HEX: {}", format_hex(number, hex_digits, endian)),
        format!("DEC: {number}"),
        format!("OCT: 0o{number:o}"),
        format!("BIN: {}", format_bin(number, hex_digits, endian)),
        format!("ASC: {}", format_ascii(number, hex_digits, endian)),
    ]
}

pub(super) fn format_hex(number: u128, hex_digits: Option<usize>, endian: Endian) -> String {
    let digits = display_hex_digits(number, hex_digits, endian);
    let mut formatted = String::from("0x");
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 4 == 0 {
            formatted.push('_');
        }
        formatted.push(digit);
    }

    formatted
}

fn format_bin(number: u128, hex_digits: Option<usize>, endian: Endian) -> String {
    let digits = display_hex_digits(number, hex_digits, endian)
        .chars()
        .map(|digit| {
            let value = digit
                .to_digit(16)
                .expect("display hex digits are hexadecimal");
            format!("{value:04b}")
        })
        .collect::<String>();
    let mut formatted = String::from("0b");

    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && index % 4 == 0 {
            formatted.push('_');
        }
        formatted.push(digit);
    }

    formatted
}

fn format_ascii(number: u128, hex_digits: Option<usize>, endian: Endian) -> String {
    display_bytes(number, hex_digits, endian)
        .into_iter()
        .map(|byte| match byte {
            0x20..=0x7e => char::from(byte),
            _ => '.',
        })
        .collect()
}

pub(super) fn display_hex_digits(
    number: u128,
    hex_digits: Option<usize>,
    endian: Endian,
) -> String {
    let digits = match hex_digits {
        Some(width) => format!("{number:0width$x}"),
        None => format!("{number:x}"),
    };

    match endian {
        Endian::Big => digits,
        Endian::Little => reverse_hex_digit_bytes(&digits),
    }
}

fn display_bytes(number: u128, hex_digits: Option<usize>, endian: Endian) -> Vec<u8> {
    let byte_width = hex_digits
        .map(|hex_digits| hex_digits.div_ceil(2))
        .unwrap_or_else(|| byte_width(number));
    let bytes = (0..byte_width)
        .rev()
        .map(|index| ((number >> (index * 8)) & 0xff) as u8)
        .collect::<Vec<_>>();

    match endian {
        Endian::Big => bytes,
        Endian::Little => bytes.into_iter().rev().collect(),
    }
}

fn reverse_hex_digit_bytes(digits: &str) -> String {
    let padded_digits = if digits.len().is_multiple_of(2) {
        digits.to_string()
    } else {
        format!("0{digits}")
    };
    padded_digits
        .as_bytes()
        .chunks(2)
        .rev()
        .map(|chunk| std::str::from_utf8(chunk).expect("hex digits are valid UTF-8"))
        .collect::<String>()
}

fn byte_width(number: u128) -> usize {
    if number == 0 {
        1
    } else {
        (128 - number.leading_zeros() as usize).div_ceil(8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_hex_without_leading_zeroes() {
        assert_eq!(
            format_hex(0x1234_1234_1234_1234, None, Endian::Big),
            "0x1234_1234_1234_1234"
        );
        assert_eq!(
            format_hex(0x1_2345_6789, None, Endian::Big),
            "0x1_2345_6789"
        );
        assert_eq!(format_hex(0x123_4567, None, Endian::Big), "0x123_4567");
        assert_eq!(format_hex(1, None, Endian::Big), "0x1");
        assert_eq!(format_hex(0, None, Endian::Big), "0x0");
    }

    #[test]
    fn formats_bin_aligned_to_4_bits() {
        assert_eq!(
            format_bin(0x1234, None, Endian::Big),
            "0b0001_0010_0011_0100"
        );
        assert_eq!(format_bin(1, None, Endian::Big), "0b0001");
        assert_eq!(format_bin(0, None, Endian::Big), "0b0000");
    }

    #[test]
    fn formats_ascii_from_bytes() {
        assert_eq!(format_ascii(0x4865_6c6c_6f, None, Endian::Big), "Hello");
        assert_eq!(format_ascii(0x41_7f, None, Endian::Big), "A.");
        assert_eq!(format_ascii(0, None, Endian::Big), ".");
    }

    #[test]
    fn formats_display_bytes_in_little_endian() {
        assert_eq!(format_hex(0x1234_5678, None, Endian::Little), "0x7856_3412");
        assert_eq!(format_hex(0x123, None, Endian::Little), "0x2301");
        assert_eq!(
            format_bin(0x1234, None, Endian::Little),
            "0b0011_0100_0001_0010"
        );
        assert_eq!(format_ascii(0x4865_6c6c_6f, None, Endian::Little), "olleH");
    }

    #[test]
    fn formats_all_text_area_lines() {
        assert_eq!(
            format_lines(0x1234, None, Endian::Big),
            vec![
                "HEX: 0x1234",
                "DEC: 4660",
                "OCT: 0o11064",
                "BIN: 0b0001_0010_0011_0100",
                "ASC: .4",
            ]
        );
    }

    #[test]
    fn formats_text_area_with_fixed_hex_digits() {
        assert_eq!(format_hex(0x1234, Some(8), Endian::Big), "0x0000_1234");
        assert_eq!(
            format_bin(0x1234, Some(8), Endian::Big),
            "0b0000_0000_0000_0000_0001_0010_0011_0100"
        );
        assert_eq!(format_ascii(0x1234, Some(8), Endian::Big), "...4");
        assert_eq!(
            format_lines(0x1234, Some(8), Endian::Big),
            vec![
                "HEX: 0x0000_1234",
                "DEC: 4660",
                "OCT: 0o11064",
                "BIN: 0b0000_0000_0000_0000_0001_0010_0011_0100",
                "ASC: ...4",
            ]
        );
    }
}
