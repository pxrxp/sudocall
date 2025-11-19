mod terminal;

fn main() {
    let (rows, cols) = terminal::get_size().expect("Terminal isn't supported.");
    println!("{}, {}", rows + 1, cols + 1);
}
