// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

pub fn is_help_flag(input: &str) -> bool {
    input == "-h" || input == "--help"
}

pub fn is_version_flag(input: &str) -> bool {
    input == "-v" || input == "--version"
}

pub fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

pub fn print_help(program_name: &str) {
    println!(
        "bitruler - visualize, decode, and inspect binary data\n\n\
Usage:\n  {program_name} <unsigned-number>\n  {program_name} --help / -h\n  {program_name} --version / -v\n\n\
Arguments:\n  <unsigned-number>    Unsigned 128-bit integer to inspect\n\n\
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
}
