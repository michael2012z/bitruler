// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

//! Input number parsing for all accepted radix prefixes.

pub(super) fn parse_unsigned(input: &str) -> Result<u128, String> {
    let normalized = input.replace('_', "");

    if normalized.is_empty() {
        return Err("number is empty".to_string());
    }

    let (digits, radix) = split_radix(&normalized);

    if digits.is_empty() {
        return Err("number has no digits".to_string());
    }

    u128::from_str_radix(digits, radix).map_err(|_| format!("invalid unsigned number: {input}"))
}

fn split_radix(normalized: &str) -> (&str, u32) {
    if let Some(digits) = normalized
        .strip_prefix("0x")
        .or_else(|| normalized.strip_prefix("0X"))
    {
        (digits, 16)
    } else if let Some(digits) = normalized
        .strip_prefix("0o")
        .or_else(|| normalized.strip_prefix("0O"))
    {
        (digits, 8)
    } else if let Some(digits) = normalized
        .strip_prefix("0b")
        .or_else(|| normalized.strip_prefix("0B"))
    {
        (digits, 2)
    } else {
        (normalized, 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_supported_radixes() {
        assert_eq!(
            parse_unsigned("0x1234_1234_1234_1234"),
            Ok(0x1234_1234_1234_1234)
        );
        assert_eq!(
            parse_unsigned("0X1234_1234_1234_1234"),
            Ok(0x1234_1234_1234_1234)
        );
        assert_eq!(
            parse_unsigned("1311693406324658740"),
            Ok(0x1234_1234_1234_1234)
        );
        assert_eq!(
            parse_unsigned("0o110640443202215011064"),
            Ok(0x1234_1234_1234_1234)
        );
        assert_eq!(
            parse_unsigned("0O110640443202215011064"),
            Ok(0x1234_1234_1234_1234)
        );
        assert_eq!(
            parse_unsigned("0b0001001000110100000100100011010000010010001101000001001000110100"),
            Ok(0x1234_1234_1234_1234)
        );
        assert_eq!(
            parse_unsigned("0B0001001000110100000100100011010000010010001101000001001000110100"),
            Ok(0x1234_1234_1234_1234)
        );
    }

    #[test]
    fn parses_u128_boundaries() {
        assert_eq!(
            parse_unsigned("0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff"),
            Ok(u128::MAX)
        );
        assert!(parse_unsigned("0x1_0000_0000_0000_0000_0000_0000_0000_0000").is_err());
    }

    #[test]
    fn rejects_invalid_numbers() {
        for input in ["", "_", "0x", "0o", "0b", "0xg", "0o8", "0b2", "12abc"] {
            assert!(
                parse_unsigned(input).is_err(),
                "{input:?} should be invalid"
            );
        }
    }
}
