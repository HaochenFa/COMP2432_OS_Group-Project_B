mod health_monitor;
mod logging;
mod sim;
mod task_queue;
mod types;
mod zones;

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("bench") => {
            let robots = args.next().and_then(|v| v.parse::<usize>().ok());
            let tasks_per_robot = args.next().and_then(|v| v.parse::<usize>().ok());
            let zones = args.next().and_then(|v| v.parse::<u64>().ok());
            sim::run_benchmark(robots, tasks_per_robot, zones);
        }
        _ => sim::run_demo(),
    }
}
