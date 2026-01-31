use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn dev_log(message: &str) {
    if !cfg!(debug_assertions) {
        return;
    }

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let current = thread::current();
    let thread_name = current.name().unwrap_or("unnamed");
    println!("[{ts}ms][{thread_name}] {message}");
}

#[macro_export]
macro_rules! log_dev {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::logging::dev_log(&format!($($arg)*));
        }
    };
}
