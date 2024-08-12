use anyhow::Context;
use sqlx::SqlitePool;

use crate::{
    domain::{NewTask, Task},
    generator::Generator,
};

pub async fn add_task(
    pool: &SqlitePool,
    generator: &Generator,
    input: NewTask,
) -> anyhow::Result<Task> {
    let task = Task::new(input, generator);
    sqlx::query!(
        r#"
            INSERT INTO tasks (id, description, completed_at, created_at, scope)
            VALUES ($1, $2, $3, $4, $5)
        "#,
        task.id,
        task.description,
        task.completed_at,
        task.created_at,
        task.scope
    )
    .execute(pool)
    .await
    .context("Failed to save task")?;
    Ok(task)
}
