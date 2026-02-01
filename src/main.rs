mod health_monitor;
mod logging;
mod sim;
mod task_queue;
mod types;
mod zones;

use std::io::Write;

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

fn write_usage<W: Write>(out: &mut W, program: &str) {
    let _ = writeln!(out, "Project Blaze CLI");
    let _ = writeln!(out, "Usage:");
    let _ = writeln!(out, "  {program} (run demo)");
    let _ = writeln!(
        out,
        "  {program} bench [robots] [tasks_per_robot] [zones] [work_ms] [validate] [offline-demo]"
    );
    let _ = writeln!(
        out,
        "  {program} stress [robot_sets] [task_sets] [zone_sets] [work_ms] [validate] [offline-demo]"
    );
    let _ = writeln!(out, "  {program} --help");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "Sets are comma-separated lists (e.g., 1,2,4). Use \"-\" to keep defaults for robot/task/zone sets."
    );
    let _ = writeln!(out, "Omit work_ms to keep its default.");
    let _ = writeln!(out, "Defaults:");
    let _ = writeln!(out, "  bench  robots=4 tasks_per_robot=25 zones=2 work_ms=5");
    let _ = writeln!(
        out,
        "  stress robots=1,2,4,8,12 tasks_per_robot=10,25,50 zones=1,2,4 work_ms=5"
    );
    let _ = writeln!(out, "Flags:");
    let _ = writeln!(out, "  validate       enable extra safety checks");
    let _ = writeln!(
        out,
        "  offline-demo   simulate a robot going offline (alias: offline)"
    );
}

fn print_usage_stdout(program: &str) {
    let mut out = std::io::stdout();
    write_usage(&mut out, program);
}

fn print_usage_stderr(program: &str) {
    let mut out = std::io::stderr();
    write_usage(&mut out, program);
}

fn exit_with_usage(program: &str, message: &str) -> ! {
    eprintln!("{message}");
    print_usage_stderr(program);
    std::process::exit(2);
}

fn main() {
    let program = std::env::args()
        .next()
        .unwrap_or_else(|| "project_blaze".to_string());
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("bench") => {
            let mut robots: Option<usize> = None;
            let mut tasks_per_robot: Option<usize> = None;
            let mut zones: Option<u64> = None;
            let mut work_ms: Option<u64> = None;
            let mut validate = false;
            let mut simulate_offline = false;
            for arg in args {
                match arg.as_str() {
                    "validate" => validate = true,
                    "offline" | "offline-demo" | "--offline-demo" => simulate_offline = true,
                    _ => {
                        if robots.is_none() {
                            robots = arg.parse::<usize>().ok();
                            if robots.is_none() {
                                exit_with_usage(
                                    &program,
                                    &format!("bench: invalid robots value: {arg}"),
                                );
                            }
                        } else if tasks_per_robot.is_none() {
                            tasks_per_robot = arg.parse::<usize>().ok();
                            if tasks_per_robot.is_none() {
                                exit_with_usage(
                                    &program,
                                    &format!("bench: invalid tasks_per_robot value: {arg}"),
                                );
                            }
                        } else if zones.is_none() {
                            zones = arg.parse::<u64>().ok();
                            if zones.is_none() {
                                exit_with_usage(
                                    &program,
                                    &format!("bench: invalid zones value: {arg}"),
                                );
                            } else if zones == Some(0) {
                                exit_with_usage(&program, "bench: zones must be > 0");
                            }
                        } else if work_ms.is_none() {
                            work_ms = arg.parse::<u64>().ok();
                            if work_ms.is_none() {
                                exit_with_usage(
                                    &program,
                                    &format!("bench: invalid work_ms value: {arg}"),
                                );
                            }
                        } else {
                            exit_with_usage(&program, &format!("bench: unexpected argument: {arg}"));
                        }
                    }
                }
            }
            sim::run_benchmark(robots, tasks_per_robot, zones, work_ms, validate, simulate_offline);
        }
        Some("stress") => {
            let mut robot_sets: Option<Vec<usize>> = None;
            let mut task_sets: Option<Vec<usize>> = None;
            let mut zone_sets: Option<Vec<u64>> = None;
            let mut work_ms: Option<u64> = None;
            let mut robot_sets_skipped = false;
            let mut task_sets_skipped = false;
            let mut zone_sets_skipped = false;
            let mut validate = false;
            let mut simulate_offline = false;

            for arg in args {
                match arg.as_str() {
                    "validate" => {
                        validate = true;
                        continue;
                    }
                    "offline" | "offline-demo" | "--offline-demo" => {
                        simulate_offline = true;
                        continue;
                    }
                    _ => {}
                }

                let mut consumed = false;
                if robot_sets.is_none() && !robot_sets_skipped {
                    if arg == "-" {
                        robot_sets_skipped = true;
                        consumed = true;
                    } else if let Some(values) = parse_usize_list(&arg) {
                        robot_sets = Some(values);
                        consumed = true;
                    }
                    if !consumed {
                        exit_with_usage(&program, &format!("stress: invalid robot_sets value: {arg}"));
                    }
                    continue;
                }
                if task_sets.is_none() && !task_sets_skipped {
                    if arg == "-" {
                        task_sets_skipped = true;
                        consumed = true;
                    } else if let Some(values) = parse_usize_list(&arg) {
                        task_sets = Some(values);
                        consumed = true;
                    }
                    if !consumed {
                        exit_with_usage(&program, &format!("stress: invalid task_sets value: {arg}"));
                    }
                    continue;
                }
                if zone_sets.is_none() && !zone_sets_skipped {
                    if arg == "-" {
                        zone_sets_skipped = true;
                        consumed = true;
                    } else if let Some(values) = parse_u64_list(&arg) {
                        zone_sets = Some(values);
                        consumed = true;
                    }
                    if !consumed {
                        exit_with_usage(&program, &format!("stress: invalid zone_sets value: {arg}"));
                    }
                    continue;
                }
                if work_ms.is_none() {
                    if let Ok(value) = arg.parse::<u64>() {
                        work_ms = Some(value);
                    } else {
                        exit_with_usage(&program, &format!("stress: invalid work_ms value: {arg}"));
                    }
                    continue;
                }

                exit_with_usage(&program, &format!("stress: unexpected argument: {arg}"));
            }

            if let Some(zones) = zone_sets.as_mut() {
                let before = zones.len();
                zones.retain(|&zones| zones > 0);
                let dropped = before.saturating_sub(zones.len());
                if dropped > 0 {
                    eprintln!("stress warning: ignored {dropped} zone set(s) <= 0");
                }
                if zones.is_empty() {
                    exit_with_usage(&program, "stress: zones must be > 0");
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
        Some("--help") | Some("-h") | Some("help") => print_usage_stdout(&program),
        Some(other) => {
            exit_with_usage(&program, &format!("unknown command: {other}"));
        }
        None => sim::run_demo(),
    }
}
