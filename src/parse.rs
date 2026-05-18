// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

//! Input number parsing for all accepted radix prefixes.

pub(super) fn parse_unsigned(input: &str) -> Result<u128, String> {
    let normalized = input.replace('_', "");

    if normalized.is_empty() {
        return Err("number is empty".to_string());
    }

    let (digits, radix) = split_radix(&normalized);
    let (digits, multiplier) = split_size_suffix(digits);

    if digits.is_empty() {
        return Err("number has no digits".to_string());
    }

    u128::from_str_radix(digits, radix)
        .ok()
        .and_then(|number| number.checked_mul(multiplier))
        .ok_or_else(|| format!("invalid unsigned number: {input}"))
}

fn split_size_suffix(digits: &str) -> (&str, u128) {
    match digits.as_bytes().last().copied() {
        Some(b'K' | b'k') => (&digits[..digits.len() - 1], 1024),
        Some(b'M' | b'm') => (&digits[..digits.len() - 1], 1024 * 1024),
        Some(b'G' | b'g') => (&digits[..digits.len() - 1], 1024 * 1024 * 1024),
        Some(_) | None => (digits, 1),
    }
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
    fn parses_binary_size_suffixes() {
        assert_eq!(parse_unsigned("40K"), Ok(40 * 1024));
        assert_eq!(parse_unsigned("40k"), Ok(40 * 1024));
        assert_eq!(parse_unsigned("3M"), Ok(3 * 1024 * 1024));
        assert_eq!(parse_unsigned("80m"), Ok(80 * 1024 * 1024));
        assert_eq!(parse_unsigned("25G"), Ok(25 * 1024 * 1024 * 1024));
        assert_eq!(parse_unsigned("34g"), Ok(34 * 1024 * 1024 * 1024));
    }

    #[test]
    fn parses_size_suffixes_with_other_radixes_and_separators() {
        assert_eq!(parse_unsigned("0x10K"), Ok(0x10 * 1024));
        assert_eq!(parse_unsigned("0b10M"), Ok(2 * 1024 * 1024));
        assert_eq!(parse_unsigned("1_024K"), Ok(1024 * 1024));
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
        for input in [
            "", "_", "0x", "0o", "0b", "0xg", "0o8", "0b2", "12abc", "K", "1T",
        ] {
            assert!(
                parse_unsigned(input).is_err(),
                "{input:?} should be invalid"
            );
        }
    }

    #[test]
    fn rejects_size_suffix_overflow() {
        assert!(parse_unsigned("0x4000_0000_0000_0000_0000_0000_0000_0000K").is_err());
    }
}
