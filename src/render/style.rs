// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

//! ANSI color styling for data and grey visual scaffolding.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ColorMode {
    Color(ColorPalette),
    NoColor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ColorPalette {
    Default,
    Ansi256([u8; 4]),
}

pub(super) const HIGHLIGHT_START: &str = "\x1b[1m";
pub(super) const HIGHLIGHT_END: &str = "\x1b[0m";
const GREY: &str = "\x1b[90m";
pub(super) const DEFAULT_COLOR_INDEXES: [u8; 4] = [1, 2, 3, 4];

pub(super) fn colorize(color_index: usize, input: &str, color_mode: ColorMode) -> String {
    if color_mode == ColorMode::NoColor {
        return input.to_string();
    }

    let color = color_escape(color_index, color_mode).expect("color mode is enabled");
    format!("{color}{input}{HIGHLIGHT_END}")
}

pub(super) fn color_index_from_right(index: usize, token_count: usize) -> usize {
    const COLOR_COUNT: usize = DEFAULT_COLOR_INDEXES.len();

    index + (COLOR_COUNT - token_count % COLOR_COUNT)
}

pub(super) fn color_escape(color_index: usize, color_mode: ColorMode) -> Option<String> {
    match color_mode {
        ColorMode::Color(ColorPalette::Default) => {
            Some(ansi_256_color_escape(color_index, DEFAULT_COLOR_INDEXES))
        }
        ColorMode::Color(ColorPalette::Ansi256(indexes)) => {
            Some(ansi_256_color_escape(color_index, indexes))
        }
        ColorMode::NoColor => None,
    }
}

fn ansi_256_color_escape(color_index: usize, indexes: [u8; 4]) -> String {
    format!("\x1b[38;5;{}m", indexes[color_index % indexes.len()])
}

pub(super) fn grey_visual_scaffolding(line: &str, color_mode: ColorMode) -> String {
    if color_mode == ColorMode::NoColor {
        return line.to_string();
    }

    let mut output = String::new();
    let mut chars = line.chars().peekable();
    let mut styled_content = false;

    while let Some(character) = chars.next() {
        if character == '\x1b' {
            let mut sequence = String::from(character);
            for character in chars.by_ref() {
                sequence.push(character);
                if character == 'm' {
                    break;
                }
            }
            styled_content = sequence != HIGHLIGHT_END;
            output.push_str(&sequence);
        } else if styled_content || character == ' ' {
            output.push(character);
        } else {
            output.push_str(GREY);
            output.push(character);
            output.push_str(HIGHLIGHT_END);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colorizes_hex_digits() {
        assert_eq!(
            colorize(2, "████", ColorMode::Color(ColorPalette::Default)),
            "\x1b[38;5;3m████\x1b[0m"
        );
    }

    #[test]
    fn colorizes_with_ansi_256_indexes() {
        assert_eq!(
            colorize(
                5,
                "████",
                ColorMode::Color(ColorPalette::Ansi256([1, 2, 3, 4]))
            ),
            "\x1b[38;5;2m████\x1b[0m"
        );
    }

    #[test]
    fn leaves_hex_digits_uncolored_in_no_color_mode() {
        assert_eq!(colorize(2, "████", ColorMode::NoColor), "████");
    }

    #[test]
    fn calculates_color_indexes_from_the_right() {
        let indexes = (0..5)
            .map(|index| color_index_from_right(index, 5))
            .collect::<Vec<_>>();

        assert_eq!(indexes, vec![3, 4, 5, 6, 7]);
    }

    #[test]
    fn greys_visual_scaffolding() {
        assert_eq!(
            grey_visual_scaffolding(
                "A \x1b[34mB\x1b[0m",
                ColorMode::Color(ColorPalette::Default)
            ),
            "\x1b[90mA\x1b[0m \x1b[34mB\x1b[0m"
        );
    }

    #[test]
    fn leaves_scaffolding_uncolored_in_no_color_mode() {
        assert_eq!(grey_visual_scaffolding("A B", ColorMode::NoColor), "A B");
    }
}
