// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

//! Visual area rendering for Unit, Hex, Bit, and Position areas.

use crate::format::{display_hex_digits, Endian};

mod bit;
mod hex;
mod layout;
mod ruler;
mod style;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RenderColor {
    Color,
    NoColor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct RenderOptions {
    pub(super) color: RenderColor,
    pub(super) mode: RenderMode,
    pub(super) hex_digits: Option<usize>,
    pub(super) endian: Endian,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RenderMode {
    Full,
    Compact,
}

impl From<RenderColor> for style::ColorMode {
    fn from(color: RenderColor) -> Self {
        match color {
            RenderColor::Color => Self::Color,
            RenderColor::NoColor => Self::NoColor,
        }
    }
}

pub(super) fn render_visual(number: u128, options: RenderOptions) -> Vec<String> {
    let hex_string = display_hex_digits(number, options.hex_digits, options.endian);
    let hex_digits = hex_string.chars().collect::<Vec<_>>();
    let bit_digits = hex_digits
        .iter()
        .map(|digit| {
            let value = digit
                .to_digit(16)
                .expect("display hex digits are hexadecimal");
            format!("{value:04b}")
        })
        .collect::<String>();
    let color_mode = style::ColorMode::from(options.color);

    let mut lines = match options.mode {
        RenderMode::Full => render_full_prefix(&hex_digits, color_mode),
        RenderMode::Compact => render_compact_prefix(&hex_digits, color_mode),
    };
    let top_connector = match options.mode {
        RenderMode::Full => bit::TopConnector::Full,
        RenderMode::Compact => bit::TopConnector::Compact,
    };
    lines.extend(layout::add_left_labels(
        bit::render_bit_area(&bit_digits, color_mode, top_connector),
        " BIT POS",
    ));
    lines
        .into_iter()
        .map(|line| style::grey_visual_scaffolding(&line, color_mode))
        .collect()
}

fn render_full_prefix(hex_digits: &[char], color_mode: style::ColorMode) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(String::new());
    lines.extend(ruler::render_ruler_with_left_labels(hex_digits, "UNIT"));
    lines.push(String::new());
    lines.extend(layout::add_left_labels(
        hex::render_hex_digits(hex_digits, color_mode),
        "HEX",
    ));
    lines.push(String::new());
    lines
}

fn render_compact_prefix(hex_digits: &[char], color_mode: style::ColorMode) -> Vec<String> {
    layout::add_left_labels(
        vec![
            String::new(),
            render_compact_hex_digits(hex_digits, color_mode),
            String::new(),
        ],
        "HEX",
    )
}

fn render_compact_hex_digits(hex_digits: &[char], color_mode: style::ColorMode) -> String {
    let tokens = hex_digits
        .iter()
        .enumerate()
        .map(|(index, digit)| style::colorize(index, &format!("{:>4}", digit), color_mode))
        .collect::<Vec<_>>();

    format!(
        "{}{}",
        " ".repeat(layout::DATA_INDENT),
        layout::join_visual_tokens(&tokens)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::format_hex;
    use crate::test_support::strip_ansi;

    fn color_options(color: RenderColor) -> RenderOptions {
        RenderOptions {
            color,
            mode: RenderMode::Full,
            hex_digits: None,
            endian: Endian::Big,
        }
    }

    fn fixed_width_options(hex_digits: usize) -> RenderOptions {
        RenderOptions {
            color: RenderColor::Color,
            mode: RenderMode::Full,
            hex_digits: Some(hex_digits),
            endian: Endian::Big,
        }
    }

    fn compact_options() -> RenderOptions {
        RenderOptions {
            color: RenderColor::NoColor,
            mode: RenderMode::Compact,
            hex_digits: None,
            endian: Endian::Big,
        }
    }

    fn little_endian_options() -> RenderOptions {
        RenderOptions {
            color: RenderColor::NoColor,
            mode: RenderMode::Compact,
            hex_digits: None,
            endian: Endian::Little,
        }
    }

    #[test]
    fn renders_visual_layout_for_hex_digits() {
        let rendered = render_visual(0x1234, color_options(RenderColor::Color)).join("\n");

        let lines = rendered.lines().collect::<Vec<_>>();

        assert_eq!(strip_ansi(lines[1].trim_end()), "U       256 ─┐ ┌─ 16");
        assert_eq!(strip_ansi(lines[2].trim_end()), "N    4K ┐    │ │    ┌─ 1");
        assert_eq!(strip_ansi(lines[4].trim_end()), "H      █  ████ ████ █  █");
        assert_eq!(strip_ansi(lines[10].trim_end()), "     ├┬┬┤ ├┬┬┤ ├┬┬┤ ├┬┬┤");
        assert_eq!(strip_ansi(lines[12].trim_end()), "I    0001_0010_0011_0100");
        assert_eq!(strip_ansi(lines[16].trim_end()), "O      12    8    4    0");
    }

    #[test]
    fn renders_balanced_ruler_for_32_bit_values() {
        let rendered = render_visual(0x1234_5678, color_options(RenderColor::Color)).join("\n");

        let lines = rendered.lines().collect::<Vec<_>>();

        assert_eq!(
            strip_ansi(lines[1].trim_end()),
            "U                 64K ─┐   ┌─ 4K"
        );
        assert_eq!(
            strip_ansi(lines[4].trim_end()),
            "T  256M ┐    │    │    │   │    │    │    ┌─ 1"
        );
    }

    #[test]
    fn renders_visual_groups_from_the_right() {
        let rendered = render_visual(0x123_4567, color_options(RenderColor::Color)).join("\n");

        let lines = rendered.lines().collect::<Vec<_>>();

        assert_eq!(
            strip_ansi(lines[6].trim_end()),
            "H      █  ████ ████   █  █ ████ ████ ████"
        );
        assert_eq!(
            strip_ansi(lines[12].trim_end()),
            "     ├┬┬┤ ├┬┬┤ ├┬┬┤   ├┬┬┤ ├┬┬┤ ├┬┬┤ ├┬┬┤"
        );
        assert_eq!(
            strip_ansi(lines[14].trim_end()),
            "I    0001_0010_0011 _ 0100_0101_0110_0111"
        );
        assert_eq!(
            strip_ansi(lines[18].trim_end()),
            "O      24   20   16     12    8    4    0"
        );
    }

    #[test]
    fn renders_split_ruler_for_64_bit_values() {
        let rendered =
            render_visual(0x1234_1234_1234_1234, color_options(RenderColor::Color)).join("\n");

        let lines = rendered.lines().collect::<Vec<_>>();

        assert_eq!(
            strip_ansi(lines[1].trim_end()),
            "U                                        4G ─┐   ┌─ 256M"
        );
        assert_eq!(
            strip_ansi(lines[8].trim_end()),
            "     1E ┐    │    │    │      │    │    │    │   │    │    │    │      │    │    │    ┌─ 1"
        );
    }

    #[test]
    fn renders_128_bit_values() {
        let rendered = render_visual(u128::MAX, color_options(RenderColor::Color)).join("\n");

        let lines = rendered.lines().collect::<Vec<_>>();

        assert!(strip_ansi(lines[1]).starts_with("U"));
        assert!(strip_ansi(lines[1]).contains("16E ─┐   ┌─ 1E"));
        assert!(strip_ansi(lines[16].trim_end()).starts_with("  2^124 ┐    │"));
        assert!(strip_ansi(&rendered).contains("H    "));
        assert!(strip_ansi(&rendered).contains("O     124  120  116  112"));
        assert_eq!(
            format_hex(u128::MAX, None, Endian::Big),
            "0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff"
        );
    }

    #[test]
    fn renders_all_hex_lengths_with_expected_position_labels() {
        for hex_length in 1..=32 {
            let number = if hex_length == 32 {
                u128::MAX
            } else {
                (1_u128 << (hex_length * 4)) - 1
            };
            let rendered = render_visual(number, color_options(RenderColor::Color)).join("\n");
            let stripped = strip_ansi(&rendered);
            let lines = stripped.lines().collect::<Vec<_>>();
            let position_line = lines
                .iter()
                .find(|line| line.starts_with('O'))
                .expect("rendered output has a Position area line");
            let positions = position_line
                .split_whitespace()
                .skip(1)
                .map(|position| position.parse::<usize>().expect("position is numeric"))
                .collect::<Vec<_>>();
            let expected_positions = (0..hex_length)
                .rev()
                .map(|index| index * 4)
                .collect::<Vec<_>>();

            assert_eq!(positions, expected_positions, "hex_length={hex_length}");
            assert!(stripped.contains("H    "), "hex_length={hex_length}");
            assert!(stripped.contains("I    "), "hex_length={hex_length}");
        }
    }

    #[test]
    fn renders_short_hex_lengths() {
        let cases = [
            (0xf, "O       0", "I    1111"),
            (0xff, "O       4    0", "I    1111_1111"),
            (0xfff, "O       8    4    0", "I    1111_1111_1111"),
        ];

        for (number, expected_positions, expected_bits) in cases {
            let rendered =
                strip_ansi(&render_visual(number, color_options(RenderColor::Color)).join("\n"));

            assert!(rendered.contains(expected_positions), "number={number:#x}");
            assert!(rendered.contains(expected_bits), "number={number:#x}");
        }
    }

    #[test]
    fn renders_without_ansi_sequences_in_no_color_mode() {
        let rendered = render_visual(0x1234, color_options(RenderColor::NoColor)).join("\n");

        assert!(!rendered.contains('\x1b'));
        assert!(rendered.contains("H      █  ████ ████ █  █"));
        assert!(rendered.contains("I    0001_0010_0011_0100"));
    }

    #[test]
    fn renders_fixed_hex_digit_width() {
        let rendered = strip_ansi(&render_visual(0x1234, fixed_width_options(8)).join("\n"));

        assert!(rendered.contains("I    0000_0000_0000_0000 _ 0001_0010_0011_0100"));
        assert!(rendered.contains("O      28   24   20   16     12    8    4    0"));
    }

    #[test]
    fn renders_compact_visual_with_hex_header() {
        let rendered = render_visual(0x1234, compact_options()).join("\n");

        assert!(!rendered.contains("U    "));
        assert!(rendered.contains("E       1    2    3    4"));
        assert!(rendered.contains("     ┌┬┬┤ ┌┬┬┤ ┌┬┬┤ ┌┬┬┤"));
        assert!(rendered.contains("I    0001_0010_0011_0100"));
        assert!(rendered.contains("O      12    8    4    0"));
    }

    #[test]
    fn renders_little_endian_visual_bytes() {
        let rendered = render_visual(0x1234, little_endian_options()).join("\n");

        assert!(rendered.contains("E       3    4    1    2"));
        assert!(rendered.contains("I    0011_0100_0001_0010"));
        assert!(rendered.contains("O      12    8    4    0"));
    }
}
