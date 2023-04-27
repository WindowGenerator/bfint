use bfint::interpret;
use std::fs;
use std::io::{self};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[group(required = true, multiple = false)]
struct Cli {
    /// Path to file with brainfuck code
    #[arg()]
    file: Option<String>,
    /// Brainfuck source code
    #[arg(short, long)]
    code: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(source_code) = &cli.code {
        interpret(
            source_code.as_bytes().to_vec(),
            &mut io::stout(),
            &mut io::stdin(),
        )
    }

    if let Some(file_path) = &cli.file {
        match fs::read(file_path) {
            Ok(source_code) => interpret(source_code, &mut io::stdout(), &mut io::stdin()),
            Err(err) => {
                println!(
                    "Cannot read file with path: '{}', error: '{}'",
                    file_path, err
                );
                ::std::process::exit(1)
            }
        }
    }
}
