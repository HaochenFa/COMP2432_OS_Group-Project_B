mod health_monitor;
mod logging;
mod sim;
mod task_queue;
mod types;
mod zones;

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("bench") => sim::run_benchmark(),
        _ => sim::run_demo(),
    }
}
