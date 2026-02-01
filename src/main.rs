mod health_monitor;
mod logging;
mod sim;
mod task_queue;
mod types;
mod zones;

fn parse_usize_list(arg: &str) -> Option<Vec<usize>> {
    if arg == "-" {
        return None;
    }
    let mut values = Vec::new();
    for part in arg.split(',') {
        if part.trim().is_empty() {
            return None;
        }
        let value = part.trim().parse::<usize>().ok()?;
        values.push(value);
    }
    Some(values)
}

fn parse_u64_list(arg: &str) -> Option<Vec<u64>> {
    if arg == "-" {
        return None;
    }
    let mut values = Vec::new();
    for part in arg.split(',') {
        if part.trim().is_empty() {
            return None;
        }
        let value = part.trim().parse::<u64>().ok()?;
        values.push(value);
    }
    Some(values)
}

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("bench") => {
            let robots = args.next().and_then(|v| v.parse::<usize>().ok());
            let tasks_per_robot = args.next().and_then(|v| v.parse::<usize>().ok());
            let zones = args.next().and_then(|v| v.parse::<u64>().ok());
            let work_ms = args.next().and_then(|v| v.parse::<u64>().ok());
            let mut validate = false;
            let mut simulate_offline = false;
            for arg in args {
                match arg.as_str() {
                    "validate" => validate = true,
                    "offline" => simulate_offline = true,
                    _ => {}
                }
            }
            sim::run_benchmark(robots, tasks_per_robot, zones, work_ms, validate, simulate_offline);
        }
        Some("stress") => {
            let mut robot_sets: Option<Vec<usize>> = None;
            let mut task_sets: Option<Vec<usize>> = None;
            let mut zone_sets: Option<Vec<u64>> = None;
            let mut work_ms: Option<u64> = None;
            let mut validate = false;
            let mut simulate_offline = false;

            for arg in args {
                match arg.as_str() {
                    "validate" => {
                        validate = true;
                        continue;
                    }
                    "offline" => {
                        simulate_offline = true;
                        continue;
                    }
                    _ => {}
                }

                if robot_sets.is_none() {
                    if let Some(values) = parse_usize_list(&arg) {
                        robot_sets = Some(values);
                        continue;
                    }
                }
                if task_sets.is_none() {
                    if let Some(values) = parse_usize_list(&arg) {
                        task_sets = Some(values);
                        continue;
                    }
                }
                if zone_sets.is_none() {
                    if let Some(values) = parse_u64_list(&arg) {
                        zone_sets = Some(values);
                        continue;
                    }
                }
                if work_ms.is_none() {
                    if let Ok(value) = arg.parse::<u64>() {
                        work_ms = Some(value);
                        continue;
                    }
                }
            }

            sim::run_stress(
                robot_sets,
                task_sets,
                zone_sets,
                work_ms,
                validate,
                simulate_offline,
            );
        }
        _ => sim::run_demo(),
    }
}
