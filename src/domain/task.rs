use ::chrono::{NaiveDateTime, TimeZone};
use chrono::{DateTime, Local};
use sqlx::types::chrono;

use crate::generator::Generator;

use super::{NewTask, Scope, TaskId};

#[derive(Debug)]
pub struct Task {
    pub id: TaskId,
    pub description: String,
    pub completed_at: Option<DateTime<Local>>,
    pub created_at: DateTime<Local>,
    pub scope: Option<Scope>,
}

impl Task {
    pub fn new(input: NewTask, generator: &Generator) -> Self {
        Self {
            id: TaskId::new(generator),
            description: input.description,
            completed_at: None,
            created_at: Local::now(),
            scope: input.scope,
        }
    }

    pub fn from(
        id: u32,
        description: String,
        completed_at: Option<NaiveDateTime>,
        created_at: NaiveDateTime,
        scope: Option<String>,
    ) -> Option<Self> {
        let created_at = try_parse_datetime(created_at)?;
        let completed_at = completed_at.and_then(try_parse_datetime);
        Some(Self {
            id: TaskId::from(id),
            description,
            completed_at,
            created_at,
            scope: scope.map(Scope::new),
        })
    }

    pub fn toggle_complete(&mut self) {
        self.completed_at = self
            .completed_at
            .map_or(Some(chrono::Local::now()), |_| None);
    }
}

fn try_parse_datetime(date: NaiveDateTime) -> Option<DateTime<Local>> {
    match Local.from_local_datetime(&date) {
        ::chrono::offset::LocalResult::Single(dt) => Some(dt),
        _ => None,
    }
}
