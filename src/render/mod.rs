// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

mod bit;
mod hex;
mod layout;
mod ruler;
mod style;

pub fn render_visual(number: u128) -> Vec<String> {
    let hex_digits = format!("{number:x}");
    let bit_width = hex_digits.len() * 4;
    let bit_digits = format!("{number:0bit_width$b}");
    let hex_digits = hex_digits.chars().collect::<Vec<_>>();

    let mut lines = Vec::new();
    lines.push(String::new());
    lines.extend(ruler::render_ruler_with_left_labels(&hex_digits, "UNIT"));
    lines.push(String::new());
    lines.extend(layout::add_left_labels(
        hex::render_hex_digits(&hex_digits),
        "HEX",
    ));
    lines.push(String::new());
    lines.extend(layout::add_left_labels(
        bit::render_bit_area(&bit_digits),
        " BIT POS",
    ));
    lines
        .into_iter()
        .map(|line| style::grey_visual_scaffolding(&line))
        .collect()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::format::format_hex;

    pub(crate) fn strip_ansi(input: &str) -> String {
        let mut stripped = String::new();
        let mut chars = input.chars().peekable();

        while let Some(character) = chars.next() {
            if character == '\x1b' {
                for character in chars.by_ref() {
                    if character == 'm' {
                        break;
                    }
                }
            } else {
                stripped.push(character);
            }
        }

        stripped
    }

    #[test]
    fn renders_visual_layout_for_hex_digits() {
        let rendered = render_visual(0x1234).join("\n");

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
        let rendered = render_visual(0x1234_5678).join("\n");

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
        let rendered = render_visual(0x123_4567).join("\n");

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
        let rendered = render_visual(0x1234_1234_1234_1234).join("\n");

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
        let rendered = render_visual(u128::MAX).join("\n");

        let lines = rendered.lines().collect::<Vec<_>>();

        assert!(strip_ansi(lines[1]).starts_with("U"));
        assert!(strip_ansi(lines[1]).contains("16E ─┐   ┌─ 1E"));
        assert!(strip_ansi(lines[16].trim_end()).starts_with("  2^124 ┐    │"));
        assert!(strip_ansi(&rendered).contains("H    "));
        assert!(strip_ansi(&rendered).contains("S     124  120  116  112"));
        assert_eq!(
            format_hex(u128::MAX),
            "0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff"
        );
    }
}
