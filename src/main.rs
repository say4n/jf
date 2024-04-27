use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut buffer = Vec::<u8>::new();
    io::stdin().read_to_end(&mut buffer)?;

    let mut stdout = io::stdout().lock();
    stdout.write_all(&buffer)?;
    Ok(())
}
