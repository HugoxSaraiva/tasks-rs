use std::fmt::Display;

use crate::generator::Generator;

#[derive(Debug, PartialEq, Clone)]
pub struct TaskId(u32);

impl TaskId {
    pub fn new(generator: &Generator) -> Self {
        Self(generator.poll())
    }

    pub fn from(id: u32) -> Self {
        Self(id)
    }
}

impl From<TaskId> for u32 {
    fn from(value: TaskId) -> Self {
        value.0
    }
}

impl Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::generator::Generator;

    use super::TaskId;

    #[test]
    fn ids_are_incrementing() {
        let generator = Generator::new();
        let id1 = TaskId::new(&generator);
        let id2 = TaskId::new(&generator);

        assert_eq!(TaskId(1), id1);
        assert_eq!(TaskId(2), id2);
    }

    #[test]
    fn counter_can_be_set() {
        let generator = Generator::from(42);
        let id42 = TaskId::new(&generator);

        assert_eq!(TaskId(42), id42);
    }
}
