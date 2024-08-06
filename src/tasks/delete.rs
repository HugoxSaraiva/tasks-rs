use sqlx::SqlitePool;

pub async fn delete_task(pool: &SqlitePool, task_id: u32) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
        DELETE FROM tasks
        WHERE id = $1
    "#,
        task_id
    )
    .execute(pool)
    .await?
    .rows_affected();
    Ok(rows_affected > 0)
}
