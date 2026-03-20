use std::{path::PathBuf, process::Command, str::FromStr};
#[allow(unused_imports)]
use std::io::{self, Write};


pub enum ShellCommand {
    Type(Vec<String>),
    Echo(Vec<String>),
    Exit,
    Other(String, Vec<String>),
}

impl ShellCommand {
    pub fn execute(&self) -> Result<(), anyhow::Error> {
        match self {
            ShellCommand::Type(args) => {
                handle_type_command(args.clone());
                Ok(())
            },
            ShellCommand::Echo(args) => {
                println!("{}", args.join(" "));
                Ok(())
            },
            ShellCommand::Exit => std::process::exit(0),
            ShellCommand::Other(command, args) =>  {
                if let Some(_) = is_executable(command) {
                    let _status = Command::new(command)
                        .args(args)
                        .status()
                        .expect("failed to execute process");
                } else {
                    println!("{command}: command not found");
                }
                Ok(())
            },
        }
    }
}

impl FromStr for ShellCommand {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split_whitespace();

        let cmd = parts.next().ok_or_else(|| anyhow::anyhow!("Empty input"))?;
        let args: Vec<String> = parts.map(|s| s.to_string()).collect();

        match cmd {
            "type" => Ok(ShellCommand::Type(args)),
            "echo" => Ok(ShellCommand::Echo(args)),
            "exit" => Ok(ShellCommand::Exit),
            _ => Ok(ShellCommand::Other(cmd.to_string(), args)),
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let entire_command_line = command.trim().to_string();
        let command = ShellCommand::from_str(&entire_command_line)?;
        command.execute()?;
    }
}

fn handle_type_command(args: Vec<String>) {
    match args[0].as_str() {
        "echo" | "exit" | "type" => {
            println!("{} is a shell builtin", args[0]);
        }
        _ => {
            if let Some(path) = is_executable(args[0].as_str()) {
                println!("{} is {}", args[0], path.display());
            } else {
                println!("{}: not found", args[0]);
            }
        }
    }
}

fn is_executable(command: &str) -> Option<PathBuf> {
    if let Ok(path) = which::which(command) {
        Some(path)
    } else {
        None
    }
}