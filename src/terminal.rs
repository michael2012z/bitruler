// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

//! Terminal size probing and ANSI-aware display width helpers.

use std::{env, io};

#[cfg(unix)]
const TIOCGWINSZ: u64 = 0x5413;

#[cfg(unix)]
#[repr(C)]
struct WinSize {
    rows: u16,
    columns: u16,
    x_pixels: u16,
    y_pixels: u16,
}

#[cfg(unix)]
unsafe extern "C" {
    fn ioctl(file_descriptor: i32, request: u64, ...) -> i32;
}

pub(super) fn terminal_width() -> Option<usize> {
    terminal_width_from_ioctl().or_else(terminal_width_from_env)
}

#[cfg(unix)]
fn terminal_width_from_ioctl() -> Option<usize> {
    terminal_width_from_file(&io::stdout())
        .or_else(|| terminal_width_from_file(&io::stderr()))
        .or_else(|| terminal_width_from_file(&io::stdin()))
}

#[cfg(unix)]
fn terminal_width_from_file<T: std::os::fd::AsRawFd>(file: &T) -> Option<usize> {
    let mut size = WinSize {
        rows: 0,
        columns: 0,
        x_pixels: 0,
        y_pixels: 0,
    };
    let result = unsafe { ioctl(file.as_raw_fd(), TIOCGWINSZ, &mut size) };

    if result == 0 && size.columns > 0 {
        Some(usize::from(size.columns))
    } else {
        None
    }
}

#[cfg(not(unix))]
fn terminal_width_from_ioctl() -> Option<usize> {
    None
}

fn terminal_width_from_env() -> Option<usize> {
    env::var("COLUMNS")
        .ok()
        .and_then(|columns| columns.parse::<usize>().ok())
        .filter(|columns| *columns > 0)
}

pub(super) fn display_width(line: &str) -> usize {
    let mut width = 0;
    let mut chars = line.chars().peekable();

    while let Some(character) = chars.next() {
        if character == '\x1b' {
            for character in chars.by_ref() {
                if character == 'm' {
                    break;
                }
            }
        } else {
            width += 1;
        }
    }

    width
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_width_counts_rendered_columns() {
        assert_eq!(display_width("abc"), 3);
        assert_eq!(display_width("┌─ 16"), 5);
        assert_eq!(display_width("\x1b[1m0\x1b[0m"), 1);
    }

    #[test]
    fn display_width_ignores_multiple_ansi_sequences() {
        assert_eq!(display_width("\x1b[31mab\x1b[0m\x1b[1mcd\x1b[0m"), 4);
    }
}
