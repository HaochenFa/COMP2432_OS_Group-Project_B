use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::types::RobotId;

struct HealthState {
    last_seen: HashMap<RobotId, Instant>,
    offline: HashSet<RobotId>,
}

pub struct HealthMonitor {
    state: Mutex<HealthState>,
}

impl HealthMonitor {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(HealthState {
                last_seen: HashMap::new(),
                offline: HashSet::new(),
            }),
        }
    }

    pub fn register_robot(&self, robot: RobotId) {
        let mut guard = self.state.lock().expect("health monitor mutex poisoned");
        guard.last_seen.entry(robot).or_insert_with(Instant::now);
    }

    pub fn heartbeat(&self, robot: RobotId) {
        let mut guard = self.state.lock().expect("health monitor mutex poisoned");
        guard.last_seen.insert(robot, Instant::now());
        guard.offline.remove(&robot);
    }

    pub fn detect_offline(&self, timeout: Duration) -> HashSet<RobotId> {
        let mut guard = self.state.lock().expect("health monitor mutex poisoned");
        let now = Instant::now();
        let overdue: Vec<RobotId> = guard
            .last_seen
            .iter()
            .filter_map(|(&robot, &last)| {
                if now.duration_since(last) > timeout {
                    Some(robot)
                } else {
                    None
                }
            })
            .collect();
        for robot in overdue {
            guard.offline.insert(robot);
        }
        guard.offline.clone()
    }

    pub fn offline_robots(&self) -> HashSet<RobotId> {
        let guard = self.state.lock().expect("health monitor mutex poisoned");
        guard.offline.clone()
    }

    #[cfg(test)]
    fn set_last_seen_for_test(&self, robot: RobotId, instant: Instant) {
        let mut guard = self.state.lock().expect("health monitor mutex poisoned");
        guard.last_seen.insert(robot, instant);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_offline_after_timeout() {
        let monitor = HealthMonitor::new();
        let robot = 7;
        let past = Instant::now() - Duration::from_millis(50);
        monitor.set_last_seen_for_test(robot, past);
        let offline = monitor.detect_offline(Duration::from_millis(10));
        assert!(offline.contains(&robot));
    }

    #[test]
    fn marks_never_heartbeat_after_timeout() {
        let monitor = HealthMonitor::new();
        let robot = 11;
        let past = Instant::now() - Duration::from_millis(30);
        monitor.set_last_seen_for_test(robot, past);
        let offline = monitor.detect_offline(Duration::from_millis(5));
        assert!(offline.contains(&robot));
    }

    #[test]
    fn heartbeat_clears_offline() {
        let monitor = HealthMonitor::new();
        let robot = 21;
        let past = Instant::now() - Duration::from_millis(30);
        monitor.set_last_seen_for_test(robot, past);
        monitor.detect_offline(Duration::from_millis(5));
        assert!(monitor.offline_robots().contains(&robot));
        monitor.heartbeat(robot);
        assert!(!monitor.offline_robots().contains(&robot));
    }

    #[test]
    fn deterministic_offline_without_sleep() {
        let monitor = HealthMonitor::new();
        let robot = 42;
        let past = Instant::now() - Duration::from_secs(5);
        monitor.set_last_seen_for_test(robot, past);
        let offline = monitor.detect_offline(Duration::from_secs(1));
        assert!(offline.contains(&robot));
    }
}
