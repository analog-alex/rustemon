use std::thread;
use std::time::Duration;
use std::path::Path;
use std::sync::atomic::{AtomicI32, Ordering};

use notify::{RecursiveMode, Watcher, EventKind, event, recommended_watcher};

use crate::command::run_command;

pub struct Looper {
    target_folders: Vec<String>,
}

impl Looper {
    pub fn new(target_folders: Vec<String>) -> Looper {
        Looper { target_folders }
    }

    // store the main logic of the program
    pub fn do_loop(&self, cmd: String) {

        // Use the move keyword to capture self by value
        let mut watcher = recommended_watcher(move |res| {
            match res {
                // we got a valid event, let's check and action on it
                Ok(event) => {
                    if handle_event(event) {
                        increment_global_counter();
                    }
                },
                // something went wrong, stop program, allow user to restart
                Err(e) => panic!("watch error: {:?}", e),
            }
        }).unwrap_or_else(|e| panic!("watcher error: {:?}", e));

        // watch paths
        for path in self.target_folders.clone() {
            watcher.watch(Path::new(&path), RecursiveMode::Recursive)
                .unwrap_or_else(|e| panic!("watcher error: {:?}", e));
        }

        // don't shutdown program, just sleep a small divisor of polling interval
        loop {
            thread::sleep(Duration::from_millis(200));

            // if something has changed since last loop, run command
            if read_global_counter() > 0 {
                run_command(cmd.clone());
                reset_global_counter();
            }
        }
    }
}

// Atomic global counter
static GLOBAL_COUNTER: AtomicI32 = AtomicI32::new(0);

// Read the current value of the global counter
fn read_global_counter() -> i32 {
    GLOBAL_COUNTER.load(Ordering::Acquire)
}

// Increment the global counter, resetting it to 0 if it reaches i32::MAX
fn increment_global_counter() {
    GLOBAL_COUNTER.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |count| {
        if count == i32::MAX {
            Some(0)
        } else {
            Some(count + 1)
        }
    }).ok();
}

// Reset the global counter to 0
fn reset_global_counter() {
    GLOBAL_COUNTER.store(0, Ordering::SeqCst);
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




