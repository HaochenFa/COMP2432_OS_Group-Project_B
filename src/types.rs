pub type TaskId = u64;
pub type RobotId = u64;
pub type ZoneId = u64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task {
    pub id: TaskId,
    pub description: String,
}

impl Task {
    pub fn new(id: TaskId, description: impl Into<String>) -> Self {
        Self {
            id,
            description: description.into(),
        }
    }
}
