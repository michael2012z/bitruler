// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

//! Unit area ruler labels and connector rendering.

use super::{hex, layout};

pub(super) fn render_ruler_with_left_labels(hex_digits: &[char], left_labels: &str) -> Vec<String> {
    let ruler_labels = hex_digits
        .iter()
        .enumerate()
        .map(|(index, _)| {
            let shift = (hex_digits.len() - index - 1) * 4;
            format_power_of_two(shift)
        })
        .collect::<Vec<_>>();
    let split_index = ruler_labels.len().div_ceil(2);
    let left_overhang = ruler_left_overhang(hex_digits, &ruler_labels, split_index);

    render_ruler_from_labels(hex_digits, &ruler_labels, left_overhang)
        .into_iter()
        .enumerate()
        .map(|(index, line)| {
            let label = left_labels.chars().nth(index).unwrap_or(' ');
            let padding_width = (layout::LEFT_LABEL_WIDTH - 1).saturating_sub(left_overhang);
            format!("{label}{:width$}{line}", "", width = padding_width)
        })
        .collect()
}

fn render_ruler_from_labels(
    hex_digits: &[char],
    labels: &[String],
    left_overhang: usize,
) -> Vec<String> {
    let width = layout::visual_width(labels.len()) + left_overhang;
    let split_index = labels.len().div_ceil(2);
    let row_count = split_index.max(labels.len() - split_index);
    let mut lines = vec![vec![' '; width]; row_count];

    for (index, label) in labels.iter().enumerate() {
        let row = ruler_row(index, split_index);
        let is_right_half = index >= split_index;
        let hinge_column = ruler_hinge_column(index, is_right_half, hex_digits) + left_overhang;
        let connector = if is_right_half {
            format!("┌─ {label}")
        } else if hinge_column + 1 < label.chars().count() + 3 {
            format!("{label} ┐")
        } else {
            format!("{label} ─┐")
        };
        let connector_column = if is_right_half {
            hinge_column as isize
        } else {
            hinge_column as isize - (connector.chars().count() as isize - 1)
        };
        layout::write_at(&mut lines[row], connector_column, &connector);

        for vertical_row in (row + 1)..lines.len() {
            layout::write_at(&mut lines[vertical_row], hinge_column as isize, "│");
        }
    }

    lines
        .into_iter()
        .map(|line| line.into_iter().collect())
        .collect()
}

fn ruler_left_overhang(hex_digits: &[char], labels: &[String], split_index: usize) -> usize {
    labels
        .iter()
        .take(split_index)
        .enumerate()
        .map(|(index, label)| {
            let hinge_column = ruler_hinge_column(index, false, hex_digits);
            let connector_width = format!("{label} ┐").chars().count();
            connector_width.saturating_sub(hinge_column + 1)
        })
        .max()
        .unwrap_or(0)
}

fn ruler_row(index: usize, split_index: usize) -> usize {
    if index < split_index {
        split_index - index - 1
    } else {
        index - split_index
    }
}

fn ruler_hinge_column(index: usize, is_right_half: bool, hex_digits: &[char]) -> usize {
    let token_count = hex_digits.len();
    if is_right_half {
        layout::token_start_column(index, token_count)
    } else {
        layout::token_start_column(index, token_count) + hex::hex_right_edge(hex_digits[index])
    }
}

fn format_power_of_two(shift: usize) -> String {
    match shift {
        0 => "1".to_string(),
        4 => "16".to_string(),
        8 => "256".to_string(),
        _ => {
            let suffixes = ["K", "M", "G", "T", "P", "E", "Z", "Y"];
            let suffix_index = shift / 10 - 1;
            let exponent = shift % 10;
            suffixes
                .get(suffix_index)
                .map(|suffix| format!("{}{}", 1_u16 << exponent, suffix))
                .unwrap_or_else(|| format!("2^{shift}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::strip_ansi;

    #[test]
    fn formats_power_of_two_labels() {
        assert_eq!(format_power_of_two(0), "1");
        assert_eq!(format_power_of_two(8), "256");
        assert_eq!(format_power_of_two(12), "4K");
        assert_eq!(format_power_of_two(60), "1E");
        assert_eq!(format_power_of_two(64), "16E");
        assert_eq!(format_power_of_two(124), "2^124");
    }

    #[test]
    fn aligns_ruler_connectors_for_all_hex_lengths() {
        for digit_count in 1..=32 {
            let hex_digits = (0..digit_count)
                .map(|index| char::from(b"123456789abcdef0"[index % 16]))
                .collect::<Vec<_>>();
            let lines = render_ruler_with_left_labels(&hex_digits, "UNIT");
            let lines = lines
                .iter()
                .map(|line| strip_ansi(line))
                .collect::<Vec<_>>();

            assert_ruler_connectors_align(&lines, digit_count);
        }
    }

    #[test]
    fn renders_expected_ruler_labels_for_key_lengths() {
        for digit_count in [1, 2, 3, 5, 9, 17, 32] {
            let hex_digits = vec!['f'; digit_count];
            let lines = render_ruler_with_left_labels(&hex_digits, "UNIT");
            let rendered = strip_ansi(&lines.join("\n"));
            let highest_label = format_power_of_two((digit_count - 1) * 4);

            assert!(
                rendered.contains(&highest_label),
                "digit_count={digit_count}"
            );
            assert!(rendered.contains('1'), "digit_count={digit_count}");
            assert_eq!(
                lines.len(),
                digit_count.div_ceil(2),
                "digit_count={digit_count}"
            );
        }
    }

    fn assert_ruler_connectors_align(lines: &[String], digit_count: usize) {
        for (row, line) in lines.iter().enumerate() {
            for (column, character) in line.chars().enumerate() {
                if character == '┐' {
                    for line in lines.iter().skip(row + 1) {
                        assert_eq!(
                            line.chars().nth(column),
                            Some('│'),
                            "left connector at row {row}, column {column} for {digit_count} digits is not aligned"
                        );
                    }
                } else if character == '┌' && row + 1 < lines.len() {
                    assert_eq!(
                        lines[row + 1].chars().nth(column),
                        Some('│'),
                        "right connector at row {row}, column {column} for {digit_count} digits is not aligned"
                    );
                }
            }
        }
    }
}
