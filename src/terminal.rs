use std::process::Stdio;

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
