// Copyright (c) 2026 Michael Zhao
// SPDX-License-Identifier: MIT

//! Core application flow for the `bitruler` binary.

mod cli;
mod format;
mod output;
mod parse;
mod render;
mod terminal;

#[cfg(test)]
mod test_support;

pub fn run(args: impl IntoIterator<Item = String>) -> i32 {
    let mut args = args.into_iter();
    let program_name = args.next().unwrap_or_else(|| "bitruler".to_string());

    let input = match (args.next(), args.next()) {
        (Some(flag), None) if cli::is_help_flag(&flag) => {
            cli::print_help(&program_name);
            return 0;
        }
        (Some(flag), None) if cli::is_version_flag(&flag) => {
            cli::print_version();
            return 0;
        }
        (Some(input), None) => input,
        _ => {
            eprintln!("Usage: {program_name} <unsigned-number>");
            eprintln!("Try '{program_name} --help' for more information.");
            return 2;
        }
    };

    match parse::parse_unsigned(&input) {
        Ok(number) => {
            output::print_output(number);
            0
        }
        Err(error) => {
            eprintln!("Error: {error}");
            1
        }
    }
}
