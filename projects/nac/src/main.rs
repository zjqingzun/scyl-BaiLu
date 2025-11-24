use std::process::Command;

fn main() {
    let mut args = std::env::args();

    // Skip the first argument which is the program name
    args.next();

    let Some(subcmd) = args.next() else {
        eprintln!("Usage: nac <subcommand> [args...]");
        std::process::exit(1);
    };

    let binary = subcmd;

    let status = Command::new(&binary)
        .args(args)
        .status()
        .unwrap_or_else(|e| {
            eprintln!("Failed to run '{}': {}", binary, e);
            std::process::exit(1);
        });

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
