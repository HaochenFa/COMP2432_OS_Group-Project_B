use std::collections::{HashMap, HashSet};
use std::sync::{Condvar, Mutex};

use crate::types::{RobotId, ZoneId};

pub struct ZoneAccess {
    occupied: Mutex<HashMap<ZoneId, RobotId>>,
    available: Condvar,
}

impl ZoneAccess {
    pub fn new() -> Self {
        Self {
            occupied: Mutex::new(HashMap::new()),
            available: Condvar::new(),
        }
    }

    pub fn acquire(&self, zone: ZoneId, robot: RobotId) {
        let mut guard = self.occupied.lock().expect("zone mutex poisoned");
        loop {
            if !guard.contains_key(&zone) {
                guard.insert(zone, robot);
                return;
            }
            guard = self.available.wait(guard).expect("condvar wait failed");
        }
    }

    pub fn release(&self, zone: ZoneId, robot: RobotId) -> bool {
        let mut guard = self.occupied.lock().expect("zone mutex poisoned");
        match guard.get(&zone) {
            Some(owner) if *owner == robot => {
                guard.remove(&zone);
                self.available.notify_all();
                true
            }
            Some(_) => {
                if !cfg!(debug_assertions) {
                    eprintln!(
                        "[ZONE] release by non-owner: zone={zone} robot={robot}"
                    );
                }
                debug_assert!(
                    false,
                    "zone release by non-owner: zone={zone} robot={robot}"
                );
                false
            }
            None => {
                if !cfg!(debug_assertions) {
                    eprintln!(
                        "[ZONE] release on unoccupied zone: zone={zone}"
                    );
                }
                debug_assert!(false, "zone release on unoccupied zone: zone={zone}");
                false
            }
        }
    }

    pub fn occupied_zones(&self) -> HashSet<ZoneId> {
        let guard = self.occupied.lock().expect("zone mutex poisoned");
        guard.keys().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn zone_is_exclusive_under_contention() {
        let access = Arc::new(ZoneAccess::new());
        let contenders = 6;
        let barrier = Arc::new(Barrier::new(contenders));
        let occupancy = Arc::new(AtomicUsize::new(0));
        let max_occupancy = Arc::new(AtomicUsize::new(0));
        let violation = Arc::new(AtomicBool::new(false));

        let mut handles = Vec::new();
        for robot_id in 0..contenders {
            let access = Arc::clone(&access);
            let barrier = Arc::clone(&barrier);
            let occupancy = Arc::clone(&occupancy);
            let max_occupancy = Arc::clone(&max_occupancy);
            let violation = Arc::clone(&violation);
            handles.push(thread::spawn(move || {
                barrier.wait();
                access.acquire(1, robot_id as u64);
                let current = occupancy.fetch_add(1, Ordering::SeqCst) + 1;
                let mut prev = max_occupancy.load(Ordering::SeqCst);
                while current > prev {
                    match max_occupancy.compare_exchange(
                        prev,
                        current,
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                    ) {
                        Ok(_) => break,
                        Err(next) => prev = next,
                    }
                }
                if current > 1 {
                    violation.store(true, Ordering::SeqCst);
                }
                thread::sleep(Duration::from_millis(20));
                occupancy.fetch_sub(1, Ordering::SeqCst);
                assert!(access.release(1, robot_id as u64));
            }));
        }

        for handle in handles {
            handle.join().expect("zone thread panicked");
        }

        assert!(!violation.load(Ordering::SeqCst));
        assert_eq!(max_occupancy.load(Ordering::SeqCst), 1);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "zone release by non-owner")]
    fn release_by_non_owner_panics_in_debug() {
        let access = ZoneAccess::new();
        access.acquire(1, 1);
        let _ = access.release(1, 2);
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn release_by_non_owner_fails_and_keeps_zone() {
        let access = ZoneAccess::new();
        access.acquire(1, 1);
        assert!(!access.release(1, 2));
        let occupied = access.occupied_zones();
        assert!(occupied.contains(&1));
        assert!(access.release(1, 1));
    }
}
