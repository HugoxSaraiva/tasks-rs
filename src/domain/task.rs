use ::chrono::{NaiveDateTime, TimeZone};
use chrono::{DateTime, Local};
use sqlx::types::chrono;

use crate::generator::Generator;

use super::TaskId;

pub struct Task {
    pub id: TaskId,
    pub description: String,
    pub completed_at: Option<DateTime<Local>>,
    pub created_at: DateTime<Local>,
}

impl Task {
    pub fn new(description: String, generator: &Generator) -> Self {
        Self {
            id: TaskId::new(generator),
            description,
            completed_at: None,
            created_at: Local::now(),
        }
    }

    pub fn from(
        id: u32,
        description: String,
        completed_at: Option<NaiveDateTime>,
        created_at: NaiveDateTime,
    ) -> Option<Self> {
        let created_at = try_parse_datetime(created_at)?;
        let completed_at = completed_at.and_then(|date| try_parse_datetime(date));
        Some(Self {
            id: TaskId::from(id),
            description,
            completed_at,
            created_at,
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
