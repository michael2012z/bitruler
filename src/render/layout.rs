// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

pub const LEFT_LABEL_WIDTH: usize = 5;
pub const DATA_INDENT: usize = 0;
pub const HEX_DIGIT_WIDTH: usize = 4;

pub fn add_left_labels(lines: Vec<String>, labels: &str) -> Vec<String> {
    lines
        .into_iter()
        .enumerate()
        .map(|(index, line)| {
            let label = labels.chars().nth(index).unwrap_or(' ');
            format!("{label}{:width$}{line}", "", width = LEFT_LABEL_WIDTH - 1)
        })
        .collect()
}

pub fn join_visual_tokens(tokens: &[String]) -> String {
    tokens
        .iter()
        .enumerate()
        .fold(String::new(), |mut output, (index, token)| {
            if index > 0 {
                if is_wide_group_gap(index, tokens.len()) {
                    output.push_str("   ");
                } else {
                    output.push(' ');
                }
            }
            output.push_str(token);
            output
        })
}

pub fn visual_width(token_count: usize) -> usize {
    if token_count == 0 {
        0
    } else {
        token_start_column(token_count - 1, token_count) + HEX_DIGIT_WIDTH + 10
    }
}

pub fn token_start_column(index: usize, token_count: usize) -> usize {
    DATA_INDENT + index * 5 + group_gap_count_before(index, token_count) * 2
}

pub fn group_gap_count_before(index: usize, token_count: usize) -> usize {
    if index == 0 {
        return 0;
    }

    let first_group_width = token_count % 4;
    if first_group_width == 0 {
        index / 4
    } else if index < first_group_width {
        0
    } else {
        1 + (index - first_group_width) / 4
    }
}

pub fn is_wide_group_gap(index: usize, token_count: usize) -> bool {
    index > 0
        && group_gap_count_before(index, token_count)
            > group_gap_count_before(index - 1, token_count)
}

pub fn write_at(line: &mut [char], column: isize, text: &str) {
    for (offset, character) in text.chars().enumerate() {
        let Some(column) = column.checked_add(offset as isize) else {
            continue;
        };
        let Ok(column) = usize::try_from(column) else {
            continue;
        };

        if let Some(slot) = line.get_mut(column) {
            *slot = character;
        }
    }
}
