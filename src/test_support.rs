// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

pub(crate) fn strip_ansi(input: &str) -> String {
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
