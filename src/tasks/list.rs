use anyhow::{anyhow, Context};
use sqlx::SqlitePool;

use crate::domain::Task;

pub async fn list_tasks(pool: &SqlitePool) -> anyhow::Result<Vec<Result<Task, anyhow::Error>>> {
    let tasks = sqlx::query!(
        r#"
            SELECT id as "id: u32", description, completed_at, created_at
            FROM tasks
            ORDER BY id DESC
        "#
    )
    .fetch_all(pool)
    .await
    .context("Failed fetching tasks")?
    .into_iter()
    .map(|r| Task::from(r.id, r.description, r.completed_at, r.created_at))
    .map(|t| t.ok_or(anyhow!("Failed to parse task")))
    .collect();
    Ok(tasks)
}
