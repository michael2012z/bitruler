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
    let mut no_color = false;
    let mut input = None;

    for argument in args {
        if cli::is_help_flag(&argument) {
            cli::print_help(&program_name);
            return 0;
        } else if cli::is_version_flag(&argument) {
            cli::print_version();
            return 0;
        } else if cli::is_no_color_flag(&argument) {
            no_color = true;
        } else if input.replace(argument).is_some() {
            eprintln!("Usage: {program_name} <unsigned-number>");
            eprintln!("Try '{program_name} --help' for more information.");
            return 2;
        }
    }

    let Some(input) = input else {
        eprintln!("Usage: {program_name} <unsigned-number>");
        eprintln!("Try '{program_name} --help' for more information.");
        return 2;
    };

    match parse::parse_unsigned(&input) {
        Ok(number) => {
            output::print_output(number, output_options(no_color));
            0
        }
        Err(error) => {
            eprintln!("Error: {error}");
            1
        }
    }
}

fn output_options(no_color: bool) -> output::OutputOptions {
    let color = if no_color {
        output::OutputColor::NoColor
    } else {
        output::OutputColor::Color
    };

    output::OutputOptions { color }
}
