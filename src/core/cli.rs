
use clap::{Parser, Subcommand};

/// A fictional versioning CLI
#[derive(Debug, Parser, PartialEq, Eq)] // requires `derive` feature
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum Commands {
    Record { name: String },
    Install { name: String, source: String },
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_cli() {
        let cli = Cli::parse_from(vec!["target/debug/forget-me-not", "record", "abc"]);
        let expected = Cli {
            command: Commands::Record { name: "abc".into() },
        };

        assert_eq!(expected, cli);
    }
}
