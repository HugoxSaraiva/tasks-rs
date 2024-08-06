use sqlx::SqlitePool;

use crate::domain::Task;

pub async fn get_task_by_id(pool: &SqlitePool, task_id: u32) -> anyhow::Result<Option<Task>> {
    let row = sqlx::query!(
        r#"
            SELECT id as "id: u32", description, completed_at, created_at
            FROM TASKS
            WHERE id = $1
        "#,
        task_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.and_then(|r| Task::from(r.id, r.description, r.completed_at, r.created_at)))
}
