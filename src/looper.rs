use std::thread;
use std::time::Duration;
use std::path::Path;
use colored::Colorize;

use notify::{RecursiveMode, Watcher, EventKind, event, recommended_watcher};

use crate::command::run_command;
use crate::counter;

static WAIT_TIME_MILLIS: u64 = 200;

// store the main logic of the program
pub fn do_loop(target_folders: Vec<String>, cmd_with_args: Vec<String>) {

    let mut watcher = recommended_watcher(|res| {
        match res {
            // we got a valid event, let's check and action on it
            Ok(event) => {
                if handle_event(event) {
                    counter::increment_global_counter();
                }
            }
            // something went wrong, stop program, allow user to restart
            Err(e) => panic!("watch error: {:?}", e),
        }
    }).unwrap_or_else(|e| panic!("watcher error: {:?}", e));

    // watch paths
    for path in target_folders.clone() {
        watcher.watch(Path::new(&path), RecursiveMode::Recursive)
            .unwrap_or_else(|e| panic!("watcher error: {:?}", e));
    }


    // first, run the command for the first time
    run_command(cmd_with_args.clone());


    // sleep a small divisor of polling interval
    // wake up to check if something has changed
    loop {
        thread::sleep(Duration::from_millis(WAIT_TIME_MILLIS));

        // if something has changed since last loop, run command with args and reset the counter
        if counter::read_global_counter() > 0 {
            counter::reset_global_counter();
            println!("\n{}\n", "Changes detected. Run then listen.".bright_green()); // tell user we're listening again
            run_command(cmd_with_args.clone());
        }
    }
}


fn handle_event(event: notify::Event) -> bool {
    if let
        EventKind::Create(event::CreateKind::File) | // a new file was created
        EventKind::Modify(event::ModifyKind::Data(event::DataChange::Content)) | // a file's content was modified
        EventKind::Remove(event::RemoveKind::File) = event.kind  // a file was deleted
    {
        return true;
    }
    return false;
}
