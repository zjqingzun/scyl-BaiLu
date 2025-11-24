use clap::Parser;
use std::fs;

/// A tiny grep clone (grrs)
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The pattern to search for
    pattern: String,

    /// The file to search in
    path: String,
}

fn main() {
    let args = Args::parse();

    let content = fs::read_to_string(&args.path)
        .expect("Could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
