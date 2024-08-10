use sqlx::SqlitePool;

use crate::domain::Scope;

pub async fn list(pool: &SqlitePool) -> anyhow::Result<Vec<Scope>> {
    let scopes = sqlx::query!(r#"SELECT DISTINCT scope FROM tasks"#)
        .fetch_all(pool)
        .await?
        .into_iter()
        .flat_map(|r| r.scope.map(Scope::new))
        .collect();
    Ok(scopes)
}
