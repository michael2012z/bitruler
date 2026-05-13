// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

use super::{layout, style};

pub fn render_bit_area(bit_digits: &str) -> Vec<String> {
    let chunks = bit_digits
        .as_bytes()
        .chunks(4)
        .map(|chunk| std::str::from_utf8(chunk).expect("binary digits are valid UTF-8"))
        .collect::<Vec<_>>();
    let nibble_count = chunks.len();
    let top_connectors = vec!["├┬┬┤".to_string(); nibble_count];
    let bottom_connectors = vec!["└──┤".to_string(); nibble_count];
    let bottom_verticals = vec!["   │".to_string(); nibble_count];
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

    #[test]
    fn highlights_bit_digits() {
        assert_eq!(
            highlight_bit_digits(0, "10_01"),
            "\x1b[1m\x1b[38;2;110;158;248m1\x1b[0m\x1b[1m\x1b[38;2;110;158;248m0\x1b[0m_\x1b[1m\x1b[38;2;110;158;248m0\x1b[0m\x1b[1m\x1b[38;2;110;158;248m1\x1b[0m"
        );
    }
}
