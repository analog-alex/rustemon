use std::process::Command;

pub fn run_command(cmd_with_args: Vec<String>) {
    assert!(cmd_with_args.len() > 0, "Command must have at least one argument");

    // todo: Handle args in command
    let mut compiled_cmd = Command::new(cmd_with_args[0].clone());

    for arg in cmd_with_args.iter().skip(1) {
        compiled_cmd.arg(arg);
    }

    let output = compiled_cmd.output();

    // if command failed, inform user and return function here
    if output.is_err() {
        println!("Command compiled with error: {:?}", output.err());
        return;
    }

    // safe unwrap, we just checked for error
    let output = output.unwrap();

    // inform user of command output and keep on trucking
    if output.status.success() {
        println!("{}", String::from_utf8(output.stdout).unwrap());
    } else {
        println!("Execute command failed with exit code: {}", output.status);
    }
}