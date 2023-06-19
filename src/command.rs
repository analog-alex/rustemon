use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use colored::Colorize;

struct ChildProcesses {
    processes: Vec<Child>,
}

impl ChildProcesses {
    fn new() -> Self {
        ChildProcesses {
            processes: Vec::new(),
        }
    }

    fn add(&mut self, child: Child) {
        self.processes.push(child);
    }

    fn clear(&mut self) {
        for child in self.processes.iter_mut() {
            child.kill().unwrap_or_else(|_| {
                println!("Failed to kill child process with id: {}", child.id());
            });
        }
    }
}

lazy_static::lazy_static! {
    static ref RUNNING_CHILD_PROCESSES: Arc<Mutex<ChildProcesses>> = Arc::new(Mutex::new(ChildProcesses::new()));
}

pub fn run_command(cmd_with_args: Vec<String>) {
    assert!(!cmd_with_args.is_empty(), "Command must have at least one argument");

    // Kill all child processes spawned before and that may be running
    let mut running_children = RUNNING_CHILD_PROCESSES.lock().unwrap();
    running_children.clear();

    // Compile the command with args
    let mut compiled_cmd = Command::new(&cmd_with_args[0]);

    for arg in cmd_with_args.iter().skip(1) {
        compiled_cmd.arg(arg);
    }

    let child_process = compiled_cmd.stdout(Stdio::piped()).spawn();

    // If command spawn failed, inform user and return function here
    if let Err(err) = child_process {
        println!("{}", format!("{:?}", err).bright_red());
        return;
    }

    // Safe unwrap, we just checked for error
    let mut child = child_process.unwrap();
    
    // As long as the child process is running, print its output
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);

        // Run this on another thread
        // TODO -- handle potential thread leak here
        thread::Builder::new()
            .name("output_reader".to_string())
            .spawn(move || {
                for line in reader.lines() {
                    if let Ok(line) = line {
                        println!("{}", line.bright_cyan());
                    }
                }
            })
            .unwrap();

    } else {
        println!("{}",
                 "Failed to capture stdout from the child process. Will kill and listen.".bright_red());
        child.kill().unwrap();
    }

    // Store the child process
    running_children.add(child);
}
