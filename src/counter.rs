use std::sync::atomic::{AtomicI32, Ordering};

// Atomic global counter
static GLOBAL_COUNTER: AtomicI32 = AtomicI32::new(0);

// Read the current value of the global counter
pub fn read_global_counter() -> i32 {
    GLOBAL_COUNTER.load(Ordering::Acquire)
}

// Increment the global counter, resetting it to 0 if it reaches i32::MAX
pub fn increment_global_counter() {
    GLOBAL_COUNTER.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |count| {
        if count == i32::MAX {
            Some(0)
        } else {
            Some(count + 1)
        }
    }).ok();
}

// Reset the global counter to 0
pub fn reset_global_counter() {
    GLOBAL_COUNTER.store(0, Ordering::SeqCst);
}
