// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

use crate::{format::format_lines, render::render_visual, terminal};

pub fn print_output(number: u128) {
    let visual_lines = render_visual(number);
    let format_lines = format_lines(number);
    let visual_line_count = visual_lines.len();
    let mut lines = visual_lines;
    lines.push(String::new());
    lines.extend(format_lines);

    let terminal_width = terminal::terminal_width();
    warn_if_output_exceeds_terminal_width(&lines, terminal_width);

    for (index, line) in lines.iter().enumerate() {
        if index < visual_line_count {
            println!("{}", clip_line(&trim_trailing_spaces(line), terminal_width));
        } else {
            println!("{line}");
        }
    }
}

fn warn_if_output_exceeds_terminal_width(lines: &[String], terminal_width: Option<usize>) {
    let Some(terminal_width) = terminal_width else {
        return;
    };
    let output_width = output_width(lines);

    if output_width > terminal_width {
        let warning = format!(
            "Warning: output is {output_width} columns wide, but terminal is {terminal_width} columns wide."
        );
        eprintln!("{}", clip_line(&warning, Some(terminal_width)));
    }
}

fn output_width(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|line| terminal::display_width(&trim_trailing_spaces(line)))
        .max()
        .unwrap_or(0)
}

fn trim_trailing_spaces(line: &str) -> String {
    enum Segment {
        Ansi(String),
        Character(char),
    }

    let mut segments = Vec::new();
    let mut chars = line.chars().peekable();

    while let Some(character) = chars.next() {
        if character == '\x1b' {
            let mut sequence = String::from(character);
            for character in chars.by_ref() {
                sequence.push(character);
                if character == 'm' {
                    break;
                }
            }
            segments.push(Segment::Ansi(sequence));
        } else {
            segments.push(Segment::Character(character));
        }
    }

    let Some(last_visible_index) = segments
        .iter()
        .rposition(|segment| matches!(segment, Segment::Character(character) if *character != ' '))
    else {
        return String::new();
    };

    segments
        .into_iter()
        .enumerate()
        .filter_map(|(index, segment)| match segment {
            Segment::Ansi(sequence) => Some(sequence),
            Segment::Character(character) if index <= last_visible_index => {
                Some(character.to_string())
            }
            Segment::Character(_) => None,
        })
        .collect()
}

fn clip_line(line: &str, terminal_width: Option<usize>) -> String {
    let Some(terminal_width) = terminal_width else {
        return line.to_string();
    };

    let mut clipped = String::new();
    let mut width = 0;
    let mut chars = line.chars().peekable();

    while let Some(character) = chars.next() {
        if character == '\x1b' {
            clipped.push(character);
            for character in chars.by_ref() {
                clipped.push(character);
                if character == 'm' {
                    break;
                }
            }
            continue;
        }

        if width == terminal_width {
            break;
        }

        clipped.push(character);
        width += 1;
    }

    clipped
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strip_ansi(input: &str) -> String {
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
    fn output_width_ignores_trailing_spaces() {
        assert_eq!(
            output_width(&["abc   ".to_string(), "\x1b[90mde\x1b[0m   ".to_string()]),
            3
        );
    }

    #[test]
    fn clips_lines_to_terminal_width() {
        assert_eq!(clip_line("abcdef", Some(4)), "abcd");
        assert_eq!(clip_line("┌─ 16", Some(3)), "┌─ ");
        assert_eq!(strip_ansi(&clip_line("\x1b[1m0\x1b[0m123", Some(3))), "012");
        assert_eq!(clip_line("abcdef", None), "abcdef");
    }
}
