// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

//! Bit area, dividers, and Position area rendering.

use super::{layout, style};

const TOP_CONNECTOR: &str = "├┬┬┤";
const BOTTOM_CONNECTOR: &str = "└──┤";
const BOTTOM_VERTICAL: &str = "   │";

pub(super) fn render_bit_area(bit_digits: &str) -> Vec<String> {
    let chunks = bit_digits
        .as_bytes()
        .chunks(4)
        .map(|chunk| std::str::from_utf8(chunk).expect("binary digits are valid UTF-8"))
        .collect::<Vec<_>>();
    let nibble_count = chunks.len();
    let top_connectors = repeated_tokens(TOP_CONNECTOR, nibble_count);
    let bottom_connectors = repeated_tokens(BOTTOM_CONNECTOR, nibble_count);
    let bottom_verticals = repeated_tokens(BOTTOM_VERTICAL, nibble_count);
    let bit_labels = (0..nibble_count)
        .map(|index| format!("{:>4}", (nibble_count - index - 1) * 4))
        .collect::<Vec<_>>();

    vec![
        format!(
            "{}{}",
            " ".repeat(layout::DATA_INDENT),
            layout::join_visual_tokens(&top_connectors)
        ),
        String::new(),
        format!(
            "{}{}",
            " ".repeat(layout::DATA_INDENT),
            render_bit_chunks(&chunks)
        ),
        String::new(),
        format!(
            "{}{}",
            " ".repeat(layout::DATA_INDENT),
            layout::join_visual_tokens(&bottom_connectors)
        ),
        format!(
            "{}{}",
            " ".repeat(layout::DATA_INDENT),
            layout::join_visual_tokens(&bottom_verticals)
        ),
        format!(
            "{}{}",
            " ".repeat(layout::DATA_INDENT),
            layout::join_visual_tokens(&bottom_verticals)
        ),
        format!(
            "{}{}",
            " ".repeat(layout::DATA_INDENT),
            layout::join_visual_tokens(&bit_labels)
        ),
    ]
}

fn repeated_tokens(token: &str, count: usize) -> Vec<String> {
    vec![token.to_string(); count]
}

fn render_bit_chunks(chunks: &[&str]) -> String {
    chunks
        .iter()
        .enumerate()
        .fold(String::new(), |mut output, (index, chunk)| {
            if index > 0 {
                if layout::is_wide_group_gap(index, chunks.len()) {
                    output.push_str(" _ ");
                } else {
                    output.push('_');
                }
            }
            output.push_str(&highlight_bit_digits(index, chunk));
            output
        })
}

fn highlight_bit_digits(color_index: usize, input: &str) -> String {
    let color = style::COLOR_CYCLE[color_index % style::COLOR_CYCLE.len()];

    input
        .chars()
        .map(|character| match character {
            '0' | '1' => format!(
                "{}{}{}{}",
                style::HIGHLIGHT_START,
                color,
                character,
                style::HIGHLIGHT_END
            ),
            _ => character.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::strip_ansi;

    #[test]
    fn highlights_bit_digits() {
        assert_eq!(
            highlight_bit_digits(0, "10_01"),
            "\x1b[1m\x1b[38;2;110;158;248m1\x1b[0m\x1b[1m\x1b[38;2;110;158;248m0\x1b[0m_\x1b[1m\x1b[38;2;110;158;248m0\x1b[0m\x1b[1m\x1b[38;2;110;158;248m1\x1b[0m"
        );
    }

    #[test]
    fn renders_single_nibble_bit_area() {
        let lines = render_bit_area("1010")
            .into_iter()
            .map(|line| strip_ansi(&line))
            .collect::<Vec<_>>();

        assert_eq!(lines[0], "├┬┬┤");
        assert_eq!(lines[2], "1010");
        assert_eq!(lines[4], "└──┤");
        assert_eq!(lines[7], "   0");
    }

    #[test]
    fn renders_bit_area_grouped_from_the_right() {
        let lines = render_bit_area("00010010001101000101")
            .into_iter()
            .map(|line| strip_ansi(&line))
            .collect::<Vec<_>>();

        assert_eq!(lines[0], "├┬┬┤   ├┬┬┤ ├┬┬┤ ├┬┬┤ ├┬┬┤");
        assert_eq!(lines[2], "0001 _ 0010_0011_0100_0101");
        assert_eq!(lines[4], "└──┤   └──┤ └──┤ └──┤ └──┤");
        assert_eq!(lines[7], "  16     12    8    4    0");
    }

    #[test]
    fn renders_bit_area_positions_for_key_lengths() {
        let cases = [
            (1, "   0"),
            (4, "  12    8    4    0"),
            (8, "  28   24   20   16     12    8    4    0"),
            (9, "  32     28   24   20   16     12    8    4    0"),
        ];

        for (nibble_count, expected_positions) in cases {
            let bit_digits = "0".repeat(nibble_count * 4);
            let lines = render_bit_area(&bit_digits)
                .into_iter()
                .map(|line| strip_ansi(&line))
                .collect::<Vec<_>>();

            assert_eq!(lines[7], expected_positions, "nibble_count={nibble_count}");
        }
    }
}
