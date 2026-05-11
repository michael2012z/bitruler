use std::{env, process};

fn main() {
    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| "bitruler".to_string());

    let input = match (args.next(), args.next()) {
        (Some(input), None) => input,
        _ => {
            eprintln!("Usage: {program_name} <unsigned-number>");
            process::exit(2);
        }
    };

    match parse_unsigned(&input) {
        Ok(number) => print_formats(number),
        Err(error) => {
            eprintln!("Error: {error}");
            process::exit(1);
        }
    }
}

fn parse_unsigned(input: &str) -> Result<u64, String> {
    let normalized = input.replace('_', "");

    if normalized.is_empty() {
        return Err("number is empty".to_string());
    }

    let (digits, radix) = if let Some(digits) = normalized
        .strip_prefix("0x")
        .or_else(|| normalized.strip_prefix("0X"))
    {
        (digits, 16)
    } else if let Some(digits) = normalized
        .strip_prefix("0o")
        .or_else(|| normalized.strip_prefix("0O"))
    {
        (digits, 8)
    } else if let Some(digits) = normalized
        .strip_prefix("0b")
        .or_else(|| normalized.strip_prefix("0B"))
    {
        (digits, 2)
    } else {
        (normalized.as_str(), 10)
    };

    if digits.is_empty() {
        return Err("number has no digits".to_string());
    }

    u64::from_str_radix(digits, radix).map_err(|_| format!("invalid unsigned number: {input}"))
}

fn print_formats(number: u64) {
    println!("HEX: {}", format_hex(number));
    println!("DEC: {number}");
    println!("OCT: 0o{number:o}");
    println!("BIN: {}", format_bin(number));
}

fn format_hex(number: u64) -> String {
    let digits = format!("{number:x}");
    let mut formatted = String::from("0x");

    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && index % 4 == 0 {
            formatted.push('_');
        }
        formatted.push(digit);
    }

    formatted
}

fn format_bin(number: u64) -> String {
    let digits = format!("{number:b}");
    let padding = (4 - digits.len() % 4) % 4;
    let digits = format!("{}{}", "0".repeat(padding), digits);
    let mut formatted = String::from("0b");

    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && index % 4 == 0 {
            formatted.push('_');
        }
        formatted.push(digit);
    }

    formatted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_supported_radixes() {
        assert_eq!(parse_unsigned("0x1234_1234_1234_1234"), Ok(0x1234_1234_1234_1234));
        assert_eq!(parse_unsigned("1311693406324658740"), Ok(0x1234_1234_1234_1234));
        assert_eq!(parse_unsigned("0o110640443202215011064"), Ok(0x1234_1234_1234_1234));
        assert_eq!(parse_unsigned("0b0001001000110100000100100011010000010010001101000001001000110100"), Ok(0x1234_1234_1234_1234));
    }

    #[test]
    fn formats_hex_without_leading_zeroes() {
        assert_eq!(format_hex(0x1234_1234_1234_1234), "0x1234_1234_1234_1234");
        assert_eq!(format_hex(1), "0x1");
        assert_eq!(format_hex(0), "0x0");
    }

    #[test]
    fn formats_bin_aligned_to_4_bits() {
        assert_eq!(format_bin(0x1234), "0b0001_0010_0011_0100");
        assert_eq!(format_bin(1), "0b0001");
        assert_eq!(format_bin(0), "0b0000");
    }
}
