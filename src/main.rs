use std::{env, process};

use bitruler::{cli, output, parse};

fn main() {
    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| "bitruler".to_string());

    let input = match (args.next(), args.next()) {
        (Some(flag), None) if cli::is_help_flag(&flag) => {
            cli::print_help(&program_name);
            return;
        }
        (Some(flag), None) if cli::is_version_flag(&flag) => {
            cli::print_version();
            return;
        }
        (Some(input), None) => input,
        _ => {
            eprintln!("Usage: {program_name} <unsigned-number>");
            eprintln!("Try '{program_name} --help' for more information.");
            process::exit(2);
        }
    };

    match parse::parse_unsigned(&input) {
        Ok(number) => output::print_output(number),
        Err(error) => {
            eprintln!("Error: {error}");
            process::exit(1);
        }
    }
}
