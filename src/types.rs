//! Shared identifiers and task model used across the system.

/// Unique identifier for a task in the queue.
pub type TaskId = u64;
/// Unique identifier for a robot thread.
pub type RobotId = u64;
/// Unique identifier for a physical zone.
pub type ZoneId = u64;

/// Unit of work assigned to robots.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task {
    /// Stable task identifier for logging and validation.
    pub id: TaskId,
    /// Human-readable description for demo output.
    pub description: String,
}

impl Task {
    /// Construct a new task with the provided id and description.
    pub fn new(id: TaskId, description: impl Into<String>) -> Self {
        Self {
            id,
            description: description.into(),
        }
    }
}
