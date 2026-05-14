// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

//! CLI flag handling and user-facing help/version text.

pub(super) fn is_help_flag(input: &str) -> bool {
    input == "-h" || input == "--help"
}

pub(super) fn is_version_flag(input: &str) -> bool {
    input == "-v" || input == "--version"
}

pub(super) fn is_no_color_flag(input: &str) -> bool {
    input == "--no-color"
}

pub(super) fn is_text_only_flag(input: &str) -> bool {
    input == "--text-only"
}

pub(super) fn is_compact_flag(input: &str) -> bool {
    input == "--compact"
}

pub(super) fn is_hex_digits_flag(input: &str) -> bool {
    input == "--hex-digits"
}

pub(super) fn parse_hex_digits(input: &str) -> Result<usize, String> {
    let hex_digits = input
        .parse::<usize>()
        .map_err(|_| format!("invalid --hex-digits value: {input}"))?;

    if (1..=32).contains(&hex_digits) {
        Ok(hex_digits)
    } else {
        Err(format!(
            "--hex-digits must be between 1 and 32, got {hex_digits}"
        ))
    }
}

pub(super) fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

pub(super) fn print_help(program_name: &str) {
    println!(
        "bitruler - visualize, decode, and inspect binary data\n\n\
Usage:\n  {program_name} [--no-color] [--compact | --text-only] [--hex-digits <N>] <unsigned-number>\n  {program_name} --help / -h\n  {program_name} --version / -v\n\n\
Arguments:\n  <unsigned-number>    Unsigned 128-bit integer to inspect\n\n\
Options:\n  --no-color           Disable ANSI colors in the visual output\n  --compact            Print Bit and Position areas plus text output\n  --text-only          Print only HEX, DEC, OCT, BIN, and ASC lines\n  --hex-digits <N>     Render with exactly N hex digits, from 1 to 32\n\n\
Accepted input formats:\n  Hexadecimal          0x1234\n  Decimal              4660\n  Octal                0o11064\n  Binary               0b0001_0010_0011_0100\n\n\
Notes:\n  - Underscores are allowed as digit separators\n  - Maximum value is 340282366920938463463374607431768211455\n  - Maximum hexadecimal value is 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff\n\n\
Examples:\n  {program_name} 4660\n  {program_name} 0x1234\n  {program_name} 0b0001_0010_0011_0100"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_help_flags() {
        assert!(is_help_flag("-h"));
        assert!(is_help_flag("--help"));
        assert!(!is_help_flag("0x1234"));
    }

    #[test]
    fn recognizes_version_flags() {
        assert!(is_version_flag("-v"));
        assert!(is_version_flag("--version"));
        assert!(!is_version_flag("0x1234"));
    }

    #[test]
    fn recognizes_no_color_flag() {
        assert!(is_no_color_flag("--no-color"));
        assert!(!is_no_color_flag("--color"));
    }

    #[test]
    fn recognizes_text_only_flag() {
        assert!(is_text_only_flag("--text-only"));
        assert!(!is_text_only_flag("--text"));
    }

    #[test]
    fn recognizes_compact_flag() {
        assert!(is_compact_flag("--compact"));
        assert!(!is_compact_flag("--small"));
    }

    #[test]
    fn recognizes_hex_digits_flag() {
        assert!(is_hex_digits_flag("--hex-digits"));
        assert!(!is_hex_digits_flag("--bits"));
    }

    #[test]
    fn parses_hex_digits_option() {
        assert_eq!(parse_hex_digits("1"), Ok(1));
        assert_eq!(parse_hex_digits("32"), Ok(32));
        assert!(parse_hex_digits("0").is_err());
        assert!(parse_hex_digits("33").is_err());
        assert!(parse_hex_digits("abc").is_err());
    }
}
