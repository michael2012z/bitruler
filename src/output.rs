// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

//! Terminal output assembly, clipping, and width warnings.

use crate::{
    format::{format_lines, Endian},
    render, terminal,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct OutputOptions {
    pub(super) color: OutputColor,
    pub(super) mode: OutputMode,
    pub(super) hex_digits: Option<usize>,
    pub(super) endian: Endian,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum OutputColor {
    Color,
    Ansi256([u8; 4]),
    NoColor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum OutputMode {
    VisualAndText,
    Compact,
    TextOnly,
}

pub(super) fn print_output(number: u128, options: OutputOptions) {
    let (lines, visual_line_count) = output_lines(number, options);

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

fn output_lines(number: u128, options: OutputOptions) -> (Vec<String>, usize) {
    let text_lines = format_lines(number, options.hex_digits, options.endian);
    let visual_lines = match options.mode {
        OutputMode::VisualAndText | OutputMode::Compact => render::render_visual(
            number,
            render::RenderOptions {
                color: render_color(options.color),
                mode: render_mode(options.mode),
                hex_digits: options.hex_digits,
                endian: options.endian,
            },
        ),
        OutputMode::TextOnly => Vec::new(),
    };
    let visual_line_count = visual_lines.len();
    let mut lines = visual_lines;
    if !lines.is_empty() {
        lines.push(String::new());
    }
    lines.extend(text_lines);
    (lines, visual_line_count)
}

fn render_mode(mode: OutputMode) -> render::RenderMode {
    match mode {
        OutputMode::VisualAndText => render::RenderMode::Full,
        OutputMode::Compact => render::RenderMode::Compact,
        OutputMode::TextOnly => render::RenderMode::Full,
    }
}

fn render_color(color: OutputColor) -> render::RenderColor {
    match color {
        OutputColor::Color => render::RenderColor::Color,
        OutputColor::Ansi256(indexes) => render::RenderColor::Ansi256(indexes),
        OutputColor::NoColor => render::RenderColor::NoColor,
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
    use crate::test_support::strip_ansi;

    #[test]
    fn assembles_text_only_output_without_visual_lines() {
        let (lines, visual_line_count) = output_lines(
            0x1234,
            OutputOptions {
                color: OutputColor::NoColor,
                mode: OutputMode::TextOnly,
                hex_digits: Some(8),
                endian: Endian::Big,
            },
        );

        assert_eq!(visual_line_count, 0);
        assert_eq!(
            lines,
            vec![
                "HEX: 0x0000_1234",
                "DEC: 4660",
                "OCT: 0o11064",
                "BIN: 0b0000_0000_0000_0000_0001_0010_0011_0100",
                "ASC: ...4",
            ]
        );
    }

    #[test]
    fn assembles_visual_and_text_output_with_separator() {
        let (lines, visual_line_count) = output_lines(
            0x1234,
            OutputOptions {
                color: OutputColor::NoColor,
                mode: OutputMode::VisualAndText,
                hex_digits: None,
                endian: Endian::Big,
            },
        );

        assert!(visual_line_count > 0);
        assert_eq!(lines[visual_line_count], "");
        assert_eq!(lines[visual_line_count + 1], "HEX: 0x1234");
    }

    #[test]
    fn assembles_compact_output_with_bit_area_and_text() {
        let (lines, visual_line_count) = output_lines(
            0x1234,
            OutputOptions {
                color: OutputColor::NoColor,
                mode: OutputMode::Compact,
                hex_digits: None,
                endian: Endian::Big,
            },
        );
        let rendered = lines.join("\n");

        assert!(visual_line_count > 0);
        assert!(!rendered.contains("U    "));
        assert!(rendered.contains("E       1    2    3    4"));
        assert!(rendered.contains("     ┌┬┬┤ ┌┬┬┤ ┌┬┬┤ ┌┬┬┤"));
        assert!(rendered.contains("I    0001_0010_0011_0100"));
        assert_eq!(lines[visual_line_count], "");
        assert_eq!(lines[visual_line_count + 1], "HEX: 0x1234");
    }

    #[test]
    fn assembles_little_endian_output() {
        let (lines, _visual_line_count) = output_lines(
            0x1234,
            OutputOptions {
                color: OutputColor::NoColor,
                mode: OutputMode::Compact,
                hex_digits: None,
                endian: Endian::Little,
            },
        );
        let rendered = lines.join("\n");

        assert!(rendered.contains("E       3    4    1    2"));
        assert!(rendered.contains("I    0011_0100_0001_0010"));
        assert!(rendered.contains("HEX: 0x3412"));
        assert!(rendered.contains("DEC: 4660"));
        assert!(rendered.contains("ASC: 4."));
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

    #[test]
    fn trims_trailing_spaces_without_dropping_ansi_sequences() {
        assert_eq!(trim_trailing_spaces("abc   "), "abc");
        assert_eq!(
            trim_trailing_spaces("\x1b[90mabc\x1b[0m   "),
            "\x1b[90mabc\x1b[0m"
        );
    }

    #[test]
    fn clips_lines_without_splitting_ansi_sequences() {
        let clipped = clip_line("\x1b[31mab\x1b[0mcd", Some(3));

        assert_eq!(strip_ansi(&clipped), "abc");
        assert!(clipped.contains("\x1b[31m"));
        assert!(clipped.contains("\x1b[0m"));
    }
}
