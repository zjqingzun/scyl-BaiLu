use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cat <file>");
        std::process::exit(1);
    }

    let content = fs::read_to_string(&args[1])
        .expect("Could not read file");

    io::stdout().write_all(content.as_bytes()).unwrap();
}
