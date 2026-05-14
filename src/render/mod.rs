// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

//! Visual area rendering for Unit, Hex, Bit, and Position areas.

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
    pub(super) hex_digits: Option<usize>,
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
    let hex_string = format_hex_digits(number, options.hex_digits);
    let bit_width = hex_string.len() * 4;
    let bit_digits = format!("{number:0bit_width$b}");
    let hex_digits = hex_string.chars().collect::<Vec<_>>();
    let color_mode = style::ColorMode::from(options.color);

    let mut lines = Vec::new();
    lines.push(String::new());
    lines.extend(ruler::render_ruler_with_left_labels(&hex_digits, "UNIT"));
    lines.push(String::new());
    lines.extend(layout::add_left_labels(
        hex::render_hex_digits(&hex_digits, color_mode),
        "HEX",
    ));
    lines.push(String::new());
    lines.extend(layout::add_left_labels(
        bit::render_bit_area(&bit_digits, color_mode),
        " BIT POS",
    ));
    lines
        .into_iter()
        .map(|line| style::grey_visual_scaffolding(&line, color_mode))
        .collect()
}

fn format_hex_digits(number: u128, hex_digits: Option<usize>) -> String {
    match hex_digits {
        Some(width) => format!("{number:0width$x}"),
        None => format!("{number:x}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::format_hex;
    use crate::test_support::strip_ansi;

    fn color_options(color: RenderColor) -> RenderOptions {
        RenderOptions {
            color,
            hex_digits: None,
        }
    }

    fn fixed_width_options(hex_digits: usize) -> RenderOptions {
        RenderOptions {
            color: RenderColor::Color,
            hex_digits: Some(hex_digits),
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
        assert_eq!(strip_ansi(lines[17].trim_end()), "S      12    8    4    0");
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
            strip_ansi(lines[19].trim_end()),
            "S      24   20   16     12    8    4    0"
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
        assert!(strip_ansi(&rendered).contains("S     124  120  116  112"));
        assert_eq!(
            format_hex(u128::MAX, None),
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
                .find(|line| line.starts_with('S'))
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
            (0xf, "S       0", "I    1111"),
            (0xff, "S       4    0", "I    1111_1111"),
            (0xfff, "S       8    4    0", "I    1111_1111_1111"),
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
        assert!(rendered.contains("S      28   24   20   16     12    8    4    0"));
    }
}
