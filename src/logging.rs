use std::fmt::Arguments;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn dev_log(args: Arguments) {
    if !cfg!(debug_assertions) {
        return;
    }

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let current = thread::current();
    let thread_name = current.name().unwrap_or("unnamed");
    println!("[{ts}ms][{thread_name}] {args}");
}

#[macro_export]
macro_rules! log_dev {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::logging::dev_log(format_args!($($arg)*));
        }
    };
}
