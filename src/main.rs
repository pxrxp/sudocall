mod terminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    terminal::hide_cursor();
    ctrlc::set_handler(move || {
        terminal::show_cursor();
        std::process::exit(0);
    })?;

    loop {
        terminal::clear();
        terminal::plot(255, 0, 0);
        terminal::plot(0, 255, 0);
        terminal::plot(0, 0, 255);
        terminal::plot(255, 0, 0);
        terminal::plot(0, 255, 0);
        terminal::plot(0, 0, 255);
        terminal::render()?;
    }
}
