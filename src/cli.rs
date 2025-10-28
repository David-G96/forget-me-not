use std::str::FromStr;

pub const HELP: &str =
    "Forget-me-not, a universal package recorder.\nusage: fmn <command> <option> [<args>]\n
    command:\n. ";

pub fn print_help() {
    eprintln!("{}", HELP);
}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum CliCommand {
    // track
    Track,
    // list
    List,
    // help
    Help,
}

impl FromStr for CliCommand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "track" => Ok(Self::Track),
            "list" => Ok(Self::List),
            "help" => Ok(Self::Help),
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
        // 跳过程序名
        args.next();
        // 获取命令参数（如 "track" 或 "list"）
        let command_str = args.next().unwrap_or_else(|| {
            eprintln!("No command provided");
            std::process::exit(1);
        });
        let command: CliCommand = command_str.parse().unwrap_or_else(|_| {
            eprintln!("Unknown command: {}", command_str);
            std::process::exit(1);
        });

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
