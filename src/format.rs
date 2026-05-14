// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

//! Text area formatting: HEX, DEC, OCT, BIN, and ASC lines.

pub(super) fn format_lines(number: u128, hex_digits: Option<usize>) -> Vec<String> {
    vec![
        format!("HEX: {}", format_hex(number, hex_digits)),
        format!("DEC: {number}"),
        format!("OCT: 0o{number:o}"),
        format!("BIN: {}", format_bin(number, hex_digits)),
        format!("ASC: {}", format_ascii(number, hex_digits)),
    ]
}

pub(super) fn format_hex(number: u128, hex_digits: Option<usize>) -> String {
    let digits = match hex_digits {
        Some(width) => format!("{number:0width$x}"),
        None => format!("{number:x}"),
    };
    let mut formatted = String::from("0x");
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 4 == 0 {
            formatted.push('_');
        }
        formatted.push(digit);
    }

    formatted
}

fn format_bin(number: u128, hex_digits: Option<usize>) -> String {
    let bit_width = hex_digits.map(|hex_digits| hex_digits * 4);
    let digits = match bit_width {
        Some(width) => format!("{number:0width$b}"),
        None => {
            let digits = format!("{number:b}");
            let padding = (4 - digits.len() % 4) % 4;
            format!("{}{}", "0".repeat(padding), digits)
        }
    };
    let mut formatted = String::from("0b");

    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && index % 4 == 0 {
            formatted.push('_');
        }
        formatted.push(digit);
    }

    formatted
}

fn format_ascii(number: u128, hex_digits: Option<usize>) -> String {
    let byte_width = hex_digits
        .map(|hex_digits| hex_digits.div_ceil(2))
        .unwrap_or_else(|| byte_width(number));

    (0..byte_width)
        .rev()
        .map(|index| {
            let byte = ((number >> (index * 8)) & 0xff) as u8;
            match byte {
                0x20..=0x7e => char::from(byte),
                _ => '.',
            }
        })
        .collect()
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
            format_hex(0x1234_1234_1234_1234, None),
            "0x1234_1234_1234_1234"
        );
        assert_eq!(format_hex(0x1_2345_6789, None), "0x1_2345_6789");
        assert_eq!(format_hex(0x123_4567, None), "0x123_4567");
        assert_eq!(format_hex(1, None), "0x1");
        assert_eq!(format_hex(0, None), "0x0");
    }

    #[test]
    fn formats_bin_aligned_to_4_bits() {
        assert_eq!(format_bin(0x1234, None), "0b0001_0010_0011_0100");
        assert_eq!(format_bin(1, None), "0b0001");
        assert_eq!(format_bin(0, None), "0b0000");
    }

    #[test]
    fn formats_ascii_from_bytes() {
        assert_eq!(format_ascii(0x4865_6c6c_6f, None), "Hello");
        assert_eq!(format_ascii(0x41_7f, None), "A.");
        assert_eq!(format_ascii(0, None), ".");
    }

    #[test]
    fn formats_all_text_area_lines() {
        assert_eq!(
            format_lines(0x1234, None),
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
        assert_eq!(format_hex(0x1234, Some(8)), "0x0000_1234");
        assert_eq!(
            format_bin(0x1234, Some(8)),
            "0b0000_0000_0000_0000_0001_0010_0011_0100"
        );
        assert_eq!(format_ascii(0x1234, Some(8)), "...4");
        assert_eq!(
            format_lines(0x1234, Some(8)),
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
