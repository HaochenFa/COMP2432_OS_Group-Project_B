use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use crate::health_monitor::HealthMonitor;
use crate::log_dev;
use crate::task_queue::TaskQueue;
use crate::types::Task;
use crate::zones::ZoneAccess;

pub fn run_demo() {
    log_dev!("[DEMO] start");

    let queue = Arc::new(TaskQueue::new());
    let zones = Arc::new(ZoneAccess::new());
    let monitor = Arc::new(HealthMonitor::new());

    let robots = 3;
    let tasks_per_robot = 3;
    let zones_total = 2;

    for id in 0..(robots * tasks_per_robot) {
        queue.push(Task::new(id as u64, format!("deliver-{id}")));
    }
    log_dev!(
        "[QUEUE] loaded tasks total={} per_robot={}",
        robots * tasks_per_robot,
        tasks_per_robot
    );

    let stop_flag = Arc::new(AtomicBool::new(false));
    for robot_id in 0..robots {
        monitor.register_robot(robot_id as u64);
    }

    let monitor_thread = {
        let monitor = Arc::clone(&monitor);
        let stop_flag = Arc::clone(&stop_flag);
        thread::Builder::new()
            .name("health-monitor".to_string())
            .spawn(move || {
                let timeout = Duration::from_millis(200);
                let mut already_offline = HashSet::new();
                while !stop_flag.load(Ordering::SeqCst) {
                    let offline = monitor.detect_offline(timeout);
                    for robot in offline {
                        if already_offline.insert(robot) {
                            log_dev!("[HEALTH] robot {robot} marked offline");
                        }
                    }
                    thread::sleep(Duration::from_millis(50));
                }
            })
            .expect("failed to spawn health monitor")
    };

    let mut handles = Vec::new();
    for robot_id in 0..robots {
        let queue = Arc::clone(&queue);
        let zones = Arc::clone(&zones);
        let monitor = Arc::clone(&monitor);
        let name = format!("robot-{robot_id}");
        let handle = thread::Builder::new()
            .name(name.clone())
            .spawn(move || {
                let mut completed = 0;
                let stop_heartbeat_after = if robot_id == 1 { 2 } else { usize::MAX };
                while completed < tasks_per_robot {
                    let task = queue.pop_blocking();
                    log_dev!("[QUEUE] {name} fetched task {}", task.id);
                    let zone = (task.id % zones_total as u64) + 1;
                    zones.acquire(zone, robot_id as u64);
                    log_dev!("[ZONE] {name} entered zone {zone} for task {}", task.id);
                    thread::sleep(Duration::from_millis(80));
                    let released = zones.release(zone, robot_id as u64);
                    if !released {
                        log_dev!("[ZONE] {name} failed to release zone {zone}");
                    }
                    log_dev!("[ZONE] {name} left zone {zone} for task {}", task.id);
                    completed += 1;
                    if completed <= stop_heartbeat_after {
                        monitor.heartbeat(robot_id as u64);
                        log_dev!("[HEALTH] {name} heartbeat");
                    } else {
                        log_dev!("[HEALTH] {name} stops heartbeats");
                    }
                }
            })
            .expect("failed to spawn robot thread");
        handles.push(handle);
    }

    let start = Instant::now();
    for handle in handles {
        handle.join().expect("robot thread panicked");
    }
    stop_flag.store(true, Ordering::SeqCst);
    monitor_thread
        .join()
        .expect("health monitor thread panicked");

    let occupied = zones.occupied_zones();
    log_dev!("[ZONE] occupied_zones at end = {}", occupied.len());
    let offline = monitor.offline_robots();
    log_dev!(
        "[HEALTH] offline robots at end = {}",
        offline.len()
    );
    if !offline.is_empty() {
        log_dev!("[HEALTH] offline set = {:?}", offline);
    }
    log_dev!(
        "[DEMO] finished in {}ms (dev logs suppressed in release mode)",
        start.elapsed().as_millis()
    );
}

pub fn run_benchmark(
    robots: Option<usize>,
    tasks_per_robot: Option<usize>,
    zones_total: Option<u64>,
) {
    let queue = Arc::new(TaskQueue::new());
    let zones = Arc::new(ZoneAccess::new());

    let robots = robots.unwrap_or(4);
    let tasks_per_robot = tasks_per_robot.unwrap_or(25);
    let zones_total = zones_total.unwrap_or(2);
    if zones_total == 0 {
        eprintln!("benchmark error: zones must be > 0");
        return;
    }
    let total_tasks = robots * tasks_per_robot;

    for id in 0..total_tasks {
        queue.push(Task::new(id as u64, format!("bench-{id}")));
    }
    let total_tasks = queue.len();

    let zone_wait_us = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let mut handles = Vec::new();
    let start = Instant::now();
    for robot_id in 0..robots {
        let queue = Arc::clone(&queue);
        let zones = Arc::clone(&zones);
        let zone_wait_us = Arc::clone(&zone_wait_us);
        handles.push(thread::spawn(move || {
            for _ in 0..tasks_per_robot {
                let task = queue.pop_blocking();
                let zone = (task.id % zones_total) + 1;
                let wait_start = Instant::now();
                zones.acquire(zone, robot_id as u64);
                let waited = wait_start.elapsed().as_micros() as u64;
                zone_wait_us.fetch_add(waited, Ordering::SeqCst);
                thread::sleep(Duration::from_millis(5));
                let released = zones.release(zone, robot_id as u64);
                if !released {
                    log_dev!("[ZONE] bench release failed zone={zone} robot={robot_id}");
                }
            }
        }));
    }

    for handle in handles {
        handle.join().expect("benchmark thread panicked");
    }
    let mut leftover = 0usize;
    while queue.try_pop().is_some() {
        leftover += 1;
    }
    let elapsed_ms = start.elapsed().as_millis() as f64;
    let throughput = if elapsed_ms > 0.0 {
        (total_tasks as f64) / (elapsed_ms / 1000.0)
    } else {
        0.0
    };
    let avg_zone_wait = if total_tasks > 0 {
        zone_wait_us.load(Ordering::SeqCst) as f64 / total_tasks as f64
    } else {
        0.0
    };

    println!("robots,tasks_per_robot,zones,total_tasks,elapsed_ms,throughput_tasks_per_s,avg_zone_wait_us");
    println!(
        "{robots},{tasks_per_robot},{zones_total},{total_tasks},{elapsed_ms:.2},{throughput:.2},{avg_zone_wait:.2}"
    );
    if leftover > 0 {
        eprintln!("# warning,leftover_tasks,{}", leftover);
    }
}
