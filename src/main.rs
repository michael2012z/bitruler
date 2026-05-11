use std::{env, io, process};

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

fn main() {
    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| "bitruler".to_string());

    let input = match (args.next(), args.next()) {
        (Some(flag), None) if is_help_flag(&flag) => {
            print_help(&program_name);
            return;
        }
        (Some(input), None) => input,
        _ => {
            eprintln!("Usage: {program_name} <unsigned-number>");
            eprintln!("Try '{program_name} --help' for more information.");
            process::exit(2);
        }
    };

    match parse_unsigned(&input) {
        Ok(number) => print_output(number),
        Err(error) => {
            eprintln!("Error: {error}");
            process::exit(1);
        }
    }
}

fn is_help_flag(input: &str) -> bool {
    input == "-h" || input == "--help"
}

fn print_help(program_name: &str) {
    println!(
        "bitruler - visualize, decode, and inspect binary data\n\n\
Usage:\n  {program_name} <unsigned-number>\n  {program_name} --help / -h\n\n\
Arguments:\n  <unsigned-number>    Unsigned 128-bit integer to inspect\n\n\
Accepted input formats:\n  Hexadecimal          0x1234\n  Decimal              4660\n  Octal                0o11064\n  Binary               0b0001_0010_0011_0100\n\n\
Notes:\n  - Underscores are allowed as digit separators\n  - Maximum value is 340282366920938463463374607431768211455\n  - Maximum hexadecimal value is 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff\n\n\
Examples:\n  {program_name} 4660\n  {program_name} 0x1234\n  {program_name} 0b0001_0010_0011_0100"
    );
}

fn parse_unsigned(input: &str) -> Result<u128, String> {
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

    u128::from_str_radix(digits, radix).map_err(|_| format!("invalid unsigned number: {input}"))
}

fn print_output(number: u128) {
    let lines = render_visual(number);
    warn_if_output_exceeds_terminal_width(&lines);

    for line in lines {
        println!("{line}");
    }

    println!();
    print_formats(number);
}

fn warn_if_output_exceeds_terminal_width(lines: &[String]) {
    let Some(terminal_width) = terminal_width() else {
        return;
    };
    let output_width = lines
        .iter()
        .map(|line| display_width(line))
        .max()
        .unwrap_or(0);

    if output_width > terminal_width {
        eprintln!(
            "Warning: output is {output_width} columns wide, but terminal is {terminal_width} columns wide."
        );
    }
}

fn terminal_width() -> Option<usize> {
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

fn display_width(line: &str) -> usize {
    line.chars().count()
}

fn render_visual(number: u128) -> Vec<String> {
    let hex_digits = format!("{number:x}");
    let bit_width = hex_digits.len() * 4;
    let bit_digits = format!("{number:0bit_width$b}");
    let hex_digits = hex_digits.chars().collect::<Vec<_>>();

    let mut lines = Vec::new();
    lines.push(String::new());
    lines.extend(render_ruler(&hex_digits));
    lines.push(String::new());
    lines.extend(render_hex_digits(&hex_digits));
    lines.extend(render_bit_area(&bit_digits));
    lines
}

fn render_ruler(hex_digits: &[char]) -> Vec<String> {
    let labels = hex_digits
        .iter()
        .enumerate()
        .map(|(index, _)| {
            let shift = (hex_digits.len() - index - 1) * 4;
            format_power_of_two(shift)
        })
        .collect::<Vec<_>>();
    let width = visual_width(hex_digits.len());
    let split_index = labels.len().div_ceil(2);
    let row_count = split_index.max(labels.len() - split_index);
    let mut lines = vec![vec![' '; width]; row_count];

    for (index, label) in labels.iter().enumerate() {
        let row = ruler_row(index, split_index);
        let hinge_column = ruler_hinge_column(index);
        let is_right_half = index >= split_index;
        let connector = if is_right_half {
            format!("┌─ {label}")
        } else if hinge_column + 1 < label.chars().count() + 3 {
            format!("{label} ┐")
        } else {
            format!("{label} ─┐")
        };
        let connector_column = if is_right_half {
            hinge_column
        } else {
            hinge_column.saturating_sub(connector.chars().count() - 1)
        };
        write_at(&mut lines[row], connector_column, &connector);

        for vertical_row in (row + 1)..lines.len() {
            write_at(&mut lines[vertical_row], hinge_column, "│");
        }
    }

    lines
        .into_iter()
        .map(|line| line.into_iter().collect())
        .collect()
}

fn ruler_row(index: usize, split_index: usize) -> usize {
    if index < split_index {
        split_index - index - 1
    } else {
        index - split_index
    }
}

fn render_hex_digits(hex_digits: &[char]) -> Vec<String> {
    (0..7)
        .map(|row| {
            let tokens = hex_digits
                .iter()
                .map(|digit| hex_pattern(*digit)[row].to_string())
                .collect::<Vec<_>>();
            format!("  {}", join_visual_tokens(&tokens))
        })
        .collect()
}

fn render_bit_area(bit_digits: &str) -> Vec<String> {
    let chunks = bit_digits
        .as_bytes()
        .chunks(4)
        .map(|chunk| std::str::from_utf8(chunk).expect("binary digits are valid UTF-8"))
        .collect::<Vec<_>>();
    let nibble_count = chunks.len();
    let top_connectors = vec!["──┬─".to_string(); nibble_count];
    let vertical_connectors = vec!["  │ ".to_string(); nibble_count];
    let bit_connectors = vec!["┌┬┼┐".to_string(); nibble_count];
    let bottom_connectors = vec!["├──┘".to_string(); nibble_count];
    let bottom_verticals = vec!["│   ".to_string(); nibble_count];
    let bit_labels = (0..nibble_count)
        .map(|index| format!("{:<4}", (nibble_count - index) * 4 - 1))
        .collect::<Vec<_>>();

    vec![
        format!("  {}", join_visual_tokens(&top_connectors)),
        format!("  {}", join_visual_tokens(&vertical_connectors)),
        format!("  {}", join_visual_tokens(&vertical_connectors)),
        format!("  {}", join_visual_tokens(&bit_connectors)),
        String::new(),
        format!("  {}", join_bit_chunks(&chunks)),
        String::new(),
        format!("  {}", join_visual_tokens(&bottom_connectors)),
        format!("  {}", join_visual_tokens(&bottom_verticals)),
        format!("  {}", join_visual_tokens(&bottom_verticals)),
        format!("  {}", join_visual_tokens(&bit_labels)),
    ]
}

fn join_visual_tokens(tokens: &[String]) -> String {
    tokens
        .iter()
        .enumerate()
        .fold(String::new(), |mut output, (index, token)| {
            if index > 0 {
                if index % 4 == 0 {
                    output.push_str("   ");
                } else {
                    output.push(' ');
                }
            }
            output.push_str(token);
            output
        })
}

fn visual_width(token_count: usize) -> usize {
    if token_count == 0 {
        0
    } else {
        ruler_hinge_column(token_count - 1) + 10
    }
}

fn ruler_hinge_column(index: usize) -> usize {
    index * 5 + (index / 4) * 2 + 3
}

fn write_at(line: &mut [char], column: usize, text: &str) {
    for (offset, character) in text.chars().enumerate() {
        if let Some(slot) = line.get_mut(column + offset) {
            *slot = character;
        }
    }
}

fn join_bit_chunks(chunks: &[&str]) -> String {
    chunks
        .iter()
        .enumerate()
        .fold(String::new(), |mut output, (index, chunk)| {
            if index > 0 {
                if index % 4 == 0 {
                    output.push_str(" _ ");
                } else {
                    output.push('_');
                }
            }
            output.push_str(chunk);
            output
        })
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

fn hex_pattern(digit: char) -> [&'static str; 7] {
    match digit.to_ascii_lowercase() {
        '0' => ["████", "█  █", "█  █", "█  █", "█  █", "█  █", "████"],
        '1' => ["  █ ", " ██ ", "  █ ", "  █ ", "  █ ", "  █ ", " ███"],
        '2' => ["████", "   █", "   █", "████", "█   ", "█   ", "████"],
        '3' => ["████", "   █", "   █", "████", "   █", "   █", "████"],
        '4' => ["█  █", "█  █", "█  █", "████", "   █", "   █", "   █"],
        '5' => ["████", "█   ", "█   ", "████", "   █", "   █", "████"],
        '6' => ["████", "█   ", "█   ", "████", "█  █", "█  █", "████"],
        '7' => ["████", "   █", "   █", "  █ ", " █  ", " █  ", " █  "],
        '8' => ["████", "█  █", "█  █", "████", "█  █", "█  █", "████"],
        '9' => ["████", "█  █", "█  █", "████", "   █", "   █", "████"],
        'a' => ["████", "█  █", "█  █", "████", "█  █", "█  █", "█  █"],
        'b' => ["███ ", "█  █", "█  █", "███ ", "█  █", "█  █", "███ "],
        'c' => ["████", "█   ", "█   ", "█   ", "█   ", "█   ", "████"],
        'd' => ["███ ", "█  █", "█  █", "█  █", "█  █", "█  █", "███ "],
        'e' => ["████", "█   ", "█   ", "████", "█   ", "█   ", "████"],
        'f' => ["████", "█   ", "█   ", "████", "█   ", "█   ", "█   "],
        _ => unreachable!("hex formatter only emits hexadecimal digits"),
    }
}

fn print_formats(number: u128) {
    println!("HEX: {}", format_hex(number));
    println!("DEC: {number}");
    println!("OCT: 0o{number:o}");
    println!("BIN: {}", format_bin(number));
}

fn format_hex(number: u128) -> String {
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

fn format_bin(number: u128) -> String {
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
    fn recognizes_help_flags() {
        assert!(is_help_flag("-h"));
        assert!(is_help_flag("--help"));
        assert!(!is_help_flag("0x1234"));
    }

    #[test]
    fn parses_supported_radixes() {
        assert_eq!(
            parse_unsigned("0x1234_1234_1234_1234"),
            Ok(0x1234_1234_1234_1234)
        );
        assert_eq!(
            parse_unsigned("1311693406324658740"),
            Ok(0x1234_1234_1234_1234)
        );
        assert_eq!(
            parse_unsigned("0o110640443202215011064"),
            Ok(0x1234_1234_1234_1234)
        );
        assert_eq!(
            parse_unsigned("0b0001001000110100000100100011010000010010001101000001001000110100"),
            Ok(0x1234_1234_1234_1234)
        );
    }

    #[test]
    fn parses_u128_boundaries() {
        assert_eq!(
            parse_unsigned("0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff"),
            Ok(u128::MAX)
        );
        assert!(parse_unsigned("0x1_0000_0000_0000_0000_0000_0000_0000_0000").is_err());
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

    #[test]
    fn display_width_counts_rendered_columns() {
        assert_eq!(display_width("abc"), 3);
        assert_eq!(display_width("┌─ 16"), 5);
    }

    #[test]
    fn renders_visual_layout_for_hex_digits() {
        let rendered = render_visual(0x1234).join("\n");

        assert!(rendered.contains("256 ─┐    ┌─ 16"));
        assert!(rendered.contains("4K ┐    │    │    ┌─ 1"));
        assert!(rendered.contains("  0001_0010_0011_0100"));
        assert!(rendered.contains("  15   11   7    3"));
    }

    #[test]
    fn renders_balanced_ruler_for_32_bit_values() {
        let rendered = render_visual(0x1234_5678).join("\n");

        assert!(rendered.contains("64K ─┐      ┌─ 4K"));
        assert!(rendered.contains("256M ┐  │    │    │      │    │    │    ┌─ 1"));
    }

    #[test]
    fn renders_split_ruler_for_64_bit_values() {
        let rendered = render_visual(0x1234_1234_1234_1234).join("\n");

        assert!(rendered.contains("4G ─┐      ┌─ 256M"));
        assert!(rendered.contains("1E ┐    │    │    │"));
        assert!(rendered.contains("│    │    │    ┌─ 1"));
    }

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
    fn renders_128_bit_values() {
        let rendered = render_visual(u128::MAX).join("\n");

        assert!(rendered.contains("2^124"));
        assert!(rendered.contains("16E"));
        assert!(rendered.contains("│    │    │    ┌─ 1"));
        assert_eq!(
            format_hex(u128::MAX),
            "0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff"
        );
    }
}
