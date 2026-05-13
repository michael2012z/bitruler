// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

use super::{layout, style};

pub fn render_hex_digits(hex_digits: &[char]) -> Vec<String> {
    (0..5)
        .map(|row| {
            let tokens = hex_digits
                .iter()
                .enumerate()
                .map(|(index, digit)| style::colorize(index, hex_pattern(*digit)[row]))
                .collect::<Vec<_>>();
            format!(
                "{}{}",
                " ".repeat(layout::DATA_INDENT),
                layout::join_visual_tokens(&tokens)
            )
        })
        .collect()
}

pub fn hex_right_edge(digit: char) -> usize {
    hex_pattern(digit)
        .iter()
        .filter_map(|row| {
            row.chars()
                .enumerate()
                .filter_map(|(index, character)| (character != ' ').then_some(index))
                .last()
        })
        .max()
        .unwrap_or(layout::HEX_DIGIT_WIDTH - 1)
}

fn hex_pattern(digit: char) -> [&'static str; 5] {
    match digit.to_ascii_lowercase() {
        '0' => ["████", "█  █", "█  █", "█  █", "████"],
        '1' => ["  █ ", " ██ ", "  █ ", "  █ ", " ███"],
        '2' => ["████", "   █", "████", "█   ", "████"],
        '3' => ["████", "   █", "████", "   █", "████"],
        '4' => ["█  █", "█  █", "████", "   █", "   █"],
        '5' => ["████", "█   ", "████", "   █", "████"],
        '6' => ["████", "█   ", "████", "█  █", "████"],
        '7' => ["████", "   █", "  █ ", " █  ", " █  "],
        '8' => ["████", "█  █", "████", "█  █", "████"],
        '9' => ["████", "█  █", "████", "   █", "████"],
        'a' => ["████", "█  █", "████", "█  █", "█  █"],
        'b' => ["███ ", "█  █", "███ ", "█  █", "███ "],
        'c' => ["████", "█   ", "█   ", "█   ", "████"],
        'd' => ["███ ", "█  █", "█  █", "█  █", "███ "],
        'e' => ["████", "█   ", "████", "█   ", "████"],
        'f' => ["████", "█   ", "████", "█   ", "█   "],
        _ => unreachable!("hex formatter only emits hexadecimal digits"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::tests::strip_ansi;

    #[test]
    fn renders_all_hex_digit_patterns() {
        let hex_digits = "0123456789abcdef".chars().collect::<Vec<_>>();
        let rendered = render_hex_digits(&hex_digits)
            .into_iter()
            .map(|line| strip_ansi(&line))
            .collect::<Vec<_>>();

        assert_eq!(rendered.len(), 5);
        assert_eq!(
            rendered[0],
            "████   █  ████ ████   █  █ ████ ████ ████   ████ ████ ████ ███    ████ ███  ████ ████"
        );
        assert_eq!(
            rendered[4].trim_end(),
            "████  ███ ████ ████      █ ████ ████  █     ████ ████ █  █ ███    ████ ███  ████ █"
        );
    }

    #[test]
    fn measures_hex_digit_right_edges() {
        assert_eq!(hex_right_edge('0'), 3);
        assert_eq!(hex_right_edge('1'), 3);
        assert_eq!(hex_right_edge('7'), 3);
        assert_eq!(hex_right_edge('b'), 3);
        assert_eq!(hex_right_edge('f'), 3);
    }
}
