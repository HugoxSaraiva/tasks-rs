use anyhow::Ok;
use sqlx::SqlitePool;

pub async fn get_last_id(pool: &SqlitePool) -> anyhow::Result<Option<u32>> {
    let last_id = sqlx::query!(r#"SELECT id as "id: u32" FROM TASKS ORDER BY id DESC"#)
        .fetch_optional(pool)
        .await?
        .map(|r| r.id);
    Ok(last_id)
}
