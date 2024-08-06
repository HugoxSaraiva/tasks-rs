use anyhow::Context;
use sqlx::SqlitePool;

use crate::{domain::Task, generator::Generator};

pub async fn add_task(
    pool: &SqlitePool,
    generator: &Generator,
    message: String,
) -> anyhow::Result<Task> {
    let task = Task::new(message, generator);
    let task_id: u32 = task.id.clone().into();
    sqlx::query!(
        r#"
            INSERT INTO tasks (id, description, completed_at, created_at)
            VALUES ($1, $2, $3, $4)
        "#,
        task_id,
        task.description,
        task.completed_at,
        task.created_at
    )
    .execute(pool)
    .await
    .context("Failed to save task")?;
    Ok(task)
}
