use std::{path::PathBuf, process::Command};
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let entire_command_line = command.trim().to_string();
        let args = command.split_whitespace().skip(1).collect::<Vec<&str>>();
        let command = entire_command_line.split_whitespace().next().unwrap();
        match entire_command_line.split_whitespace().next() {
            Some("type") => handle_type_command(args),
            Some("echo") => {
                println!("{}", args.join(" "));
            },
            Some("exit") => break,
            _ =>  {
                if let Some(_) = is_executable(command) {
                    let _status = Command::new(command)
                        .args(args)
                        .status()
                        .expect("failed to execute process");
                } else {
                    println!("{command}: command not found");
                }
            },
        };
    }

}

fn handle_type_command(args: Vec<&str>) {
    match args[0] {
        "echo" | "exit" | "type" => {
            println!("{} is a shell builtin", args[0]);
        }
        _ => {
            if let Some(path) = is_executable(args[0]) {
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