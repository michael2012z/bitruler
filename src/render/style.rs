// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

pub const HIGHLIGHT_START: &str = "\x1b[1m";
pub const HIGHLIGHT_END: &str = "\x1b[0m";
pub const GREY: &str = "\x1b[90m";
pub const COLOR_CYCLE: [&str; 4] = [
    "\x1b[38;2;110;158;248m",
    "\x1b[38;2;180;244;164m",
    "\x1b[38;2;212;155;255m",
    "\x1b[38;2;39;200;238m",
];

pub fn colorize(color_index: usize, input: &str) -> String {
    let color = COLOR_CYCLE[color_index % COLOR_CYCLE.len()];
    format!("{color}{input}{HIGHLIGHT_END}")
}

pub fn grey_visual_scaffolding(line: &str) -> String {
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
        assert_eq!(colorize(2, "████"), "\x1b[38;2;212;155;255m████\x1b[0m");
    }

    #[test]
    fn greys_visual_scaffolding() {
        assert_eq!(
            grey_visual_scaffolding("A \x1b[34mB\x1b[0m"),
            "\x1b[90mA\x1b[0m \x1b[34mB\x1b[0m"
        );
    }
}
