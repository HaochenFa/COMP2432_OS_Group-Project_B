use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

use crate::types::Task;

pub struct TaskQueue {
    inner: Mutex<VecDeque<Task>>,
    available: Condvar,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(VecDeque::new()),
            available: Condvar::new(),
        }
    }

    pub fn push(&self, task: Task) {
        let mut guard = self.inner.lock().expect("task queue mutex poisoned");
        guard.push_back(task);
        self.available.notify_one();
    }

    pub fn try_pop(&self) -> Option<Task> {
        let mut guard = self.inner.lock().expect("task queue mutex poisoned");
        guard.pop_front()
    }

    pub fn pop_blocking(&self) -> Task {
        let mut guard = self.inner.lock().expect("task queue mutex poisoned");
        loop {
            if let Some(task) = guard.pop_front() {
                return task;
            }
            guard = self.available.wait(guard).expect("condvar wait failed");
        }
    }

    pub fn len(&self) -> usize {
        let guard = self.inner.lock().expect("task queue mutex poisoned");
        guard.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::sync::{Arc, Barrier, Mutex};
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn tasks_are_consumed_once() {
        let queue = Arc::new(TaskQueue::new());
        let total_tasks = 100;
        for id in 0..total_tasks {
            queue.push(Task::new(id, format!("task-{id}")));
        }

        let consumers = 4;
        let barrier = Arc::new(Barrier::new(consumers));
        let seen: Arc<Mutex<HashSet<u64>>> = Arc::new(Mutex::new(HashSet::new()));

        let mut handles = Vec::new();
        for _ in 0..consumers {
            let queue = Arc::clone(&queue);
            let barrier = Arc::clone(&barrier);
            let seen = Arc::clone(&seen);
            handles.push(thread::spawn(move || {
                barrier.wait();
                loop {
                    match queue.try_pop() {
                        Some(task) => {
                            let mut guard = seen.lock().expect("seen mutex poisoned");
                            assert!(guard.insert(task.id));
                        }
                        None => break,
                    }
                }
            }));
        }

        for handle in handles {
            handle.join().expect("consumer thread panicked");
        }

        let guard = seen.lock().expect("seen mutex poisoned");
        assert_eq!(guard.len(), total_tasks as usize);
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn pop_blocking_wakes_on_push() {
        let queue = Arc::new(TaskQueue::new());
        let (tx, rx) = mpsc::channel();

        let queue_clone = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            let task = queue_clone.pop_blocking();
            tx.send(task.id).expect("send task id");
        });

        thread::sleep(Duration::from_millis(20));
        queue.push(Task::new(99, "wake"));

        let received = rx.recv_timeout(Duration::from_secs(1)).expect("receive task id");
        assert_eq!(received, 99);
        handle.join().expect("blocking pop thread panicked");
    }
}
