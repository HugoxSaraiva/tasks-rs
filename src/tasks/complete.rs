use anyhow::Ok;
use sqlx::SqlitePool;

use super::get_task_by_id;

pub async fn complete_task(pool: &SqlitePool, task_id: u32) -> anyhow::Result<bool> {
    let task = get_task_by_id(pool, task_id).await?;
    if task.is_none() {
        return Ok(false);
    }

    let mut task = task.unwrap();
    task.toggle_complete();

    let rows_affected: u32 = sqlx::query!(
        r#"
            UPDATE tasks 
            SET completed_at = $1
            WHERE id = $2
        "#,
        task.completed_at,
        task_id
    )
    .execute(pool)
    .await?
    .rows_affected()
    .try_into()
    .unwrap();
    Ok(rows_affected > 0)
}
