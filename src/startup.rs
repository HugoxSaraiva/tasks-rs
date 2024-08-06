use std::str::FromStr;

use anyhow::Context;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

use crate::{configuration::Settings, generator::Generator, tasks::get_last_id};

pub struct Application {
    pub pool: SqlitePool,
    pub generator: Generator,
}

impl Application {
    pub async fn build(configuration: Settings) -> anyhow::Result<Self> {
        let pool = SqlitePool::connect(&configuration.database.location)
            .await
            .expect("Failed to connect to the database");
        let next_id = get_last_id(&pool)
            .await?
            .map_or(1, |previous_id| previous_id + 1);
        let generator = Generator::from(next_id);
        Ok(Self { pool, generator })
    }
}

pub async fn ensure_db_created(configuration: &Settings) -> anyhow::Result<()> {
    let options =
        SqliteConnectOptions::from_str(&configuration.database.location)?.create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await.with_context(|| {
        format!(
            "Failed to connect to the database located at {}",
            configuration.database.location
        )
    })?;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");
    Ok(())
}
