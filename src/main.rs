use std::{
    fs::read,
    io::{self, IsTerminal, Read},
    process::exit,
};

use clap::{command, ArgAction, CommandFactory, Parser};
use jf::flatten;

#[derive(Parser, Debug)]
#[command(
    version,
    about="flatten them json",
    long_about=None,
    override_usage="\t$ jf -f foo.json
\t{\"foo.bar.0\":\"baz\"}

\t$ jf --filename foo.json
\t{\"foo.bar.0\":\"baz\"}

\t$ echo '{\"foo\": {\"bar\": [\"baz\"]}}' | jf
\t{\"foo.bar.0\":\"baz\"}
"
)]
struct Args {
    #[arg(short, long, value_name = "path_to_file.json")]
    filename: Option<String>,
    
    #[arg(short, long, default_value=".")]
    separator: String,

    #[arg(short, long, action=ArgAction::SetTrue)]
    pretty: bool,
}

fn main() -> io::Result<()> {
    let mut cmd = Args::command();
    let args = Args::parse();

    let mut buffer = Vec::<u8>::new();

    match args.filename {
        Some(filename) => {
            buffer = read(filename)?;
        }
        None => {
            let input = std::io::stdin();

            if input.is_terminal() {
                let _ = cmd.print_long_help();
                exit(1)
            } else {
                io::stdin().read_to_end(&mut buffer)?;
            }
        }
    };

    let json_tree = serde_json::from_slice(&buffer)?;
    let flat_json_tree = flatten(json_tree, &args.separator);

    if !args.pretty {
        println!("{}", &flat_json_tree);
    } else {
        println!("{}", serde_json::to_string_pretty(&flat_json_tree)?);
    }
    Ok(())
}
