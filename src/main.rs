mod looper;
mod command;
mod counter;

use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "analog-alex",
    version = "0.1",
    about = "Run command on file changes",
    long_about = None
)]
struct Args {
    // comma seated list of folders to watch
    #[arg(short, long)]
    target_folders: String,

    // command to run -- ideally from a Makefile
    #[arg(short, long)]
    command: String,
}

#[derive(Debug)]
pub struct ParsedArgs {
    target_folders: Vec<String>,
    command: Vec<String>,
}

impl ParsedArgs {
    pub fn validate_paths(&self) {
        let current_dir = std::env::current_dir().unwrap();
        println!("Finding folders in: {}", current_dir.display());

        for path in self.target_folders.clone() {
            if !fs::metadata(&path).is_ok() {
                eprintln!("Directory {} does not exist", path);
                std::process::exit(1);
            }
        }
    }

    pub fn validate_command(&self) {
        if self.command.len() == 0 {
            eprintln!("Command must have at least one argument");
            std::process::exit(1);
        }
    }
}

const COMMA_SEPARATOR: &str = ",";
const SPACE_SEPARATOR: &str = " ";

fn main() {
    let args = Args::parse();

    // parse the raw args
    let parsed_args = ParsedArgs {
        target_folders: args.target_folders.split(COMMA_SEPARATOR).map(|s| s.to_string()).collect(),
        command: args.command.split(SPACE_SEPARATOR).map(|s| s.to_string()).collect(),
    };

    // validate the paths (we panic if something is wrong)
    parsed_args.validate_paths();
    parsed_args.validate_command();

    // helpful console prints
    println!("Listen on folder(s): {:?}", parsed_args.target_folders);
    println!("Run '{:?}' command when file changes are detected", parsed_args.command);
    println!("Press Ctrl+C to stop listening...\n"); // one extra new line for visual separation

    // start the main loop
    looper::do_loop(parsed_args.target_folders, parsed_args.command);
}
