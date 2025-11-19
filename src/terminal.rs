use std::{io::Write, process::Stdio};

pub fn get_size() -> Option<(u16, u16)> {
    let out = std::process::Command::new("stty")
        .arg("size")
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to query terminal size.");

    let out_str = String::from_utf8(out.stdout).expect("Failed to convert bytes into string.");
    let mut tokens = out_str.split_whitespace();
    let rows = tokens
        .next()?
        .parse::<u16>()
        .expect("Failed to parse terminal size (rows)");
    let cols = tokens
        .next()?
        .parse::<u16>()
        .expect("Failed to parse terminal size (cols)");

    Some((rows, cols))
}

pub fn clear() {
    const CLEAR_ANSI: &str = "\x1B[2J";
    const HOME_ANSI: &str = "\x1B[H";

    print!("{CLEAR_ANSI}{HOME_ANSI}");
}

pub fn plot(r: u8, g: u8, b: u8) {
    const BG_COLOR_ANSI: &str = "\x1B[48;2;";

    print!("{BG_COLOR_ANSI}{r};{g};{b}m ");
}

pub fn render() -> std::io::Result<()> {
    const BG_COLOR_ANSI: &str = "\x1B[48;2;";

    print!("{BG_COLOR_ANSI}{};{};{}m", 0, 0, 0);
    std::io::stdout().flush()
}

pub fn hide_cursor() {
    print!("\x1B[?25l");
}

pub fn show_cursor() {
    print!("\x1B[?25h");
}
