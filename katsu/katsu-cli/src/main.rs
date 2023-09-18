mod commands;

use clap::Parser;
use commands::clone::CloneCommand;
use inquire::{error::InquireResult, min_length, Confirm, MultiSelect, Password, Select, Text};

#[derive(Debug, Parser)]
struct Cli {
    command: Option<String>,
    args: Vec<String>,
}

fn main() -> InquireResult<()> {
    let args = Cli::parse();
    println!("{:?}", args);
    match args.command {
        Some(command) => match command.as_str() {
            "clone" => {
                if args.args.is_empty() {
                    panic!("No URL was provided!");
                }
                let command: CloneCommand = match args.args.len() {
                    1 => CloneCommand::new(&args.args[0], None),
                    2 => CloneCommand::new(&args.args[0], Some(&args.args[1])),
                    _ => panic!("Too many arguments!"),
                };
                command.run();

            }
            _ => {
                println!("Command '{}' not found!", command.as_str());
            }
        },
        None => {
            panic!("No Command");
        }
    }
    // match args.command.as_str() {
    //     "clone" => (),
    //     _ => println!("Command not found"),
    // }
    // println!("{:?}", args);
    Ok(())
}
