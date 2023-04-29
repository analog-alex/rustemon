use std::process::Command;

pub fn run_command(command: String) {
    // todo: Handle args in command
    let output = Command::new(command)
        .output();

    // if command failed, inform user and return function here
    if output.is_err() {
        println!("Command failed with error: {:?}", output.err());
        return;
    }

    // safe unwrap, we just checked for error
    let output = output.unwrap();

    // inform user of command output and keep on trucking
    if output.status.success() {
        println!("{}", String::from_utf8(output.stdout).unwrap());
    } else {
        println!("Command failed with exit code: {}", output.status);
    }
}