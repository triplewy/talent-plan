use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"), 
    author = env!("CARGO_PKG_AUTHORS"), 
    version = env!("CARGO_PKG_VERSION"), 
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Doc comment
#[derive(Subcommand)]
#[command(version)]
enum Command {
    /// Set a kv pair
    #[command(name = "set")]
    Set {
        /// Key
        key: String,
        /// Value
        value: String,
    },

    /// Get a kv pair
    #[command(name = "get")]
    Get {
        /// Key
        key: String,
    },

    /// Remove a kv pair
    #[command(name = "rm")]
    Remove {
        /// Key
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Get { key } => {
            panic!("unimplemented")
        }
        Command::Set { key, value } => {
            panic!("unimplemented")
        }
        Command::Remove { key } => {
            panic!("unimplemented")
        }
    }
}
