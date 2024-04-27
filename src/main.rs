use std::io::{self, Read};

use jf::flatten;

fn main() -> io::Result<()> {
    let mut buffer = Vec::<u8>::new();
    io::stdin().read_to_end(&mut buffer)?;

    let json_tree = serde_json::from_slice(&buffer)?;
    let flat_json_tree = flatten(json_tree);

    println!("{:}", &flat_json_tree);
    Ok(())
}
