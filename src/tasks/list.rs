use anyhow::{anyhow, Context};
use sqlx::SqlitePool;

use crate::domain::{Scope, Task};

pub async fn list_tasks(
    pool: &SqlitePool,
    scope: Option<Scope>,
) -> anyhow::Result<Vec<Result<Task, anyhow::Error>>> {
    let tasks = sqlx::query!(
        r#"
            SELECT id as "id: u32", description, completed_at, created_at, scope
            FROM tasks
            WHERE (scope = $1) OR ($1 is null)
            ORDER BY id DESC
        "#,
        scope
    )
    .fetch_all(pool)
    .await
    .context("Failed fetching tasks")?
    .into_iter()
    .map(|r| Task::from(r.id, r.description, r.completed_at, r.created_at, r.scope))
    .map(|t| t.ok_or(anyhow!("Failed to parse task")))
    .collect();
    Ok(tasks)
}
