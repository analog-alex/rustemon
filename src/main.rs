mod looper;
mod command;

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
    command: String,
}

impl ParsedArgs {
    pub fn validate_paths(&self) {
        let current_dir = std::env::current_dir().unwrap();
        println!("Finding folders in: {}", current_dir.display());

        for path in self.target_folders.clone() {
            if !fs::metadata(&path).is_ok() {
                panic!("Directory {} does not exist", path);
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let target_folders_list = args.target_folders;
    let command = args.command;

    // parse the raw args
    let parsed_args = ParsedArgs {
        target_folders: target_folders_list.split(",").map(|s| s.to_string()).collect(),
        command,
    };

    // validate the paths
    parsed_args.validate_paths();

    // helpful console prints
    println!("Listen on folder(s): {:?}", parsed_args.target_folders);
    println!("Run '{}' commands when file changes are detected", parsed_args.command);

    // start the main loop
    looper::Looper::new(
        parsed_args.target_folders,
    ).do_loop(parsed_args.command);
}
