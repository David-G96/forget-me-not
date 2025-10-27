use std::str::FromStr;

pub const HELP: &str =
    "Forget-me-not, a universal package recorder.\nusage: fmn <command> <option> [<args>]";

pub fn print_help() {
    eprintln!("{}", HELP);
}

#[derive(Clone, Copy)]
pub enum CliCommand {
    // track
    Track,
    // list
    List,
}

impl FromStr for CliCommand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "track" => Ok(Self::Track),
            "list" => Ok(Self::List),
            s => Err(format!("Cannot parse command `{}`", s)),
        }
    }
}

#[derive(Clone, Copy)]
pub enum CliOption {
    // --help
    Help,
}

impl FromStr for CliOption {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "--help" => Ok(Self::Help),
            s => {
                unreachable!("Internal Error in parsing option `{}`", s);
            }
        }
    }
}

pub struct Cli {
    command: CliCommand,
    options: Vec<CliOption>,
    args: Vec<String>,
}

impl Cli {
    pub fn parse() -> Self {
        let mut args = std::env::args();
        let command: CliCommand = args.next().unwrap().parse().unwrap();

        let args: Vec<String> = args.collect();
        Self {
            command,
            options: vec![],
            args,
        }
    }

    pub fn command(&self) -> CliCommand {
        self.command
    }

    pub fn options(&self) -> &Vec<CliOption> {
        &self.options
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }
}
