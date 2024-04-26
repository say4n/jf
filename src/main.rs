use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let mut stdout = io::stdout().lock();
    writeln!(stdout, "{}", buffer)?;
    Ok(())
}
