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
