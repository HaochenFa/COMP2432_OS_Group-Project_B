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

fn print_usage(program: &str) {
    println!("Project Blaze CLI");
    println!("Usage:");
    println!("  {program} (run demo)");
    println!("  {program} bench [robots] [tasks_per_robot] [zones] [work_ms] [validate] [offline]");
    println!("  {program} stress [robot_sets] [task_sets] [zone_sets] [work_ms] [validate] [offline]");
    println!("  {program} --help");
    println!();
    println!("Sets are comma-separated lists (e.g., 1,2,4). Use \"-\" to keep defaults for a slot.");
    println!("Defaults:");
    println!("  bench  robots=4 tasks_per_robot=25 zones=2 work_ms=5");
    println!("  stress robots=1,2,4,8,12 tasks=10,25,50 zones=1,2,4 work_ms=5");
    println!("Flags:");
    println!("  validate  enable extra safety checks");
    println!("  offline   simulate a robot going offline");
}

fn main() {
    let program = std::env::args()
        .next()
        .unwrap_or_else(|| "project_blaze".to_string());
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
                    if arg == "-" {
                        robot_sets = Some(Vec::new());
                        continue;
                    }
                    if let Some(values) = parse_usize_list(&arg) {
                        robot_sets = Some(values);
                        continue;
                    }
                }
                if task_sets.is_none() {
                    if arg == "-" {
                        task_sets = Some(Vec::new());
                        continue;
                    }
                    if let Some(values) = parse_usize_list(&arg) {
                        task_sets = Some(values);
                        continue;
                    }
                }
                if zone_sets.is_none() {
                    if arg == "-" {
                        zone_sets = Some(Vec::new());
                        continue;
                    }
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
        Some("--help") | Some("-h") | Some("help") => print_usage(&program),
        Some(other) => {
            eprintln!("unknown command: {other}");
            print_usage(&program);
        }
        None => sim::run_demo(),
    }
}
