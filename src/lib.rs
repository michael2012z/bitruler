// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

//! Core application flow for the `bitruler` binary.

mod cli;
mod format;
mod output;
mod parse;
mod render;
mod terminal;

use format::Endian;

#[cfg(test)]
mod test_support;

pub fn run(args: impl IntoIterator<Item = String>) -> i32 {
    let mut args = args.into_iter();
    let program_name = args.next().unwrap_or_else(|| "bitruler".to_string());
    let mut no_color = false;
    let mut little_endian = false;
    let mut compact = false;
    let mut text_only = false;
    let mut hex_digits = None;
    let mut color_indexes = None;
    let mut input = None;
    let mut pending_hex_digits = false;
    let mut pending_colors = false;

    for argument in args {
        if pending_colors {
            match cli::parse_colors(&argument) {
                Ok(value) => color_indexes = Some(value),
                Err(error) => {
                    eprintln!("Error: {error}");
                    return 2;
                }
            }
            pending_colors = false;
        } else if pending_hex_digits {
            match cli::parse_hex_digits(&argument) {
                Ok(value) => hex_digits = Some(value),
                Err(error) => {
                    eprintln!("Error: {error}");
                    return 2;
                }
            }
            pending_hex_digits = false;
        } else if cli::is_help_flag(&argument) {
            cli::print_help(&program_name);
            return 0;
        } else if cli::is_version_flag(&argument) {
            cli::print_version();
            return 0;
        } else if cli::is_no_color_flag(&argument) {
            no_color = true;
        } else if cli::is_little_endian_flag(&argument) {
            little_endian = true;
        } else if cli::is_compact_flag(&argument) {
            compact = true;
        } else if cli::is_text_only_flag(&argument) {
            text_only = true;
        } else if cli::is_hex_digits_flag(&argument) {
            pending_hex_digits = true;
        } else if cli::is_colors_flag(&argument) {
            pending_colors = true;
        } else if input.replace(argument).is_some() {
            eprintln!("Usage: {program_name} <unsigned-number>");
            eprintln!("Try '{program_name} --help' for more information.");
            return 2;
        }
    }

    if pending_hex_digits {
        eprintln!("Error: --hex-digits requires a value");
        return 2;
    }
    if pending_colors {
        eprintln!("Error: --colors requires a value");
        return 2;
    }

    let Some(input) = input else {
        eprintln!("Usage: {program_name} <unsigned-number>");
        eprintln!("Try '{program_name} --help' for more information.");
        return 2;
    };
    let hex_digits = hex_digits.or_else(|| inferred_hex_digit_width(&input));

    match parse::parse_unsigned(&input) {
        Ok(number) if hex_digits.is_some_and(|width| !fits_hex_digits(number, width)) => {
            eprintln!(
                "Error: value does not fit in {} hex digits",
                hex_digits.expect("checked as Some")
            );
            1
        }
        Ok(number) => {
            output::print_output(
                number,
                output_options(
                    no_color,
                    little_endian,
                    compact,
                    text_only,
                    hex_digits,
                    color_indexes,
                ),
            );
            0
        }
        Err(error) => {
            eprintln!("Error: {error}");
            1
        }
    }
}

fn output_options(
    no_color: bool,
    little_endian: bool,
    compact: bool,
    text_only: bool,
    hex_digits: Option<usize>,
    color_indexes: Option<[u8; 4]>,
) -> output::OutputOptions {
    let color = if no_color {
        output::OutputColor::NoColor
    } else if let Some(indexes) = color_indexes {
        output::OutputColor::Ansi256(indexes)
    } else {
        output::OutputColor::Color
    };
    let mode = if text_only {
        output::OutputMode::TextOnly
    } else if compact {
        output::OutputMode::Compact
    } else {
        output::OutputMode::VisualAndText
    };

    output::OutputOptions {
        color,
        mode,
        hex_digits,
        endian: if little_endian {
            Endian::Little
        } else {
            Endian::Big
        },
    }
}

fn fits_hex_digits(number: u128, hex_digits: usize) -> bool {
    hex_digit_count(number) <= hex_digits
}

fn hex_digit_count(number: u128) -> usize {
    if number == 0 {
        1
    } else {
        (128 - number.leading_zeros() as usize).div_ceil(4)
    }
}

fn inferred_hex_digit_width(input: &str) -> Option<usize> {
    let normalized = input.replace('_', "");
    if let Some(digits) = normalized
        .strip_prefix("0x")
        .or_else(|| normalized.strip_prefix("0X"))
    {
        return (!digits.is_empty()
            && digits.len() <= 32
            && digits.chars().all(|digit| digit.is_ascii_hexdigit()))
        .then_some(digits.len());
    }

    let digits = normalized
        .strip_prefix("0b")
        .or_else(|| normalized.strip_prefix("0B"))?;

    (!digits.is_empty()
        && digits.len() <= 128
        && digits.chars().all(|digit| matches!(digit, '0' | '1')))
    .then_some(digits.len().div_ceil(4))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infers_hex_digit_width_from_input() {
        assert_eq!(inferred_hex_digit_width("0x001234"), Some(6));
        assert_eq!(inferred_hex_digit_width("0X001_234"), Some(6));
        assert_eq!(inferred_hex_digit_width("0x10K"), None);
        assert_eq!(inferred_hex_digit_width("0x0000_0000_0000_0000_0000_0000_0000_0000"), Some(32));
        assert_eq!(inferred_hex_digit_width("0x0000_0000_0000_0000_0000_0000_0000_0000_0"), None);
        assert_eq!(inferred_hex_digit_width("0b000000101"), Some(3));
        assert_eq!(inferred_hex_digit_width("0B0000_0010_1"), Some(3));
        assert_eq!(inferred_hex_digit_width("0b10M"), None);
        assert_eq!(inferred_hex_digit_width("4660"), None);
    }
}
