use std::thread;
use std::time::Duration;
use std::path::Path;

use notify::{RecursiveMode, Watcher, EventKind, event, recommended_watcher};

use crate::command::run_command;
use crate::counter::{increment_global_counter, read_global_counter, reset_global_counter};


// store the main logic of the program
pub fn do_loop(target_folders: Vec<String>, cmd: String) {

    let mut watcher = recommended_watcher(|res| {
        match res {
            // we got a valid event, let's check and action on it
            Ok(event) => {
                if handle_event(event) {
                    increment_global_counter();
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

    // don't shutdown program, just sleep a small divisor of polling interval
    loop {
        thread::sleep(Duration::from_millis(100));

        // if something has changed since last loop, run command
        if read_global_counter() > 0 {
            run_command(cmd.clone());
            reset_global_counter();
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
