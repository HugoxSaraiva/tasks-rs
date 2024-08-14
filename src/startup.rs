use std::fs;

use anyhow::Context;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

use crate::{configuration::Settings, generator::Generator, tasks::get_last_id};

pub struct Application {
    pub pool: SqlitePool,
    pub generator: Generator,
}

impl Application {
    pub async fn build(configuration: Settings) -> anyhow::Result<Self> {
        let options = SqliteConnectOptions::new().filename(configuration.location);
        let pool = SqlitePool::connect_with(options)
            .await
            .expect("Failed to connect to the database");
        let next_id = get_last_id(&pool)
            .await?
            .map_or(1, |previous_id| previous_id + 1);
        let generator = Generator::from(next_id);
        Ok(Self { pool, generator })
    }
}

pub async fn ensure_initialized(configuration: &Settings) -> anyhow::Result<()> {
    ensure_folder_created(configuration)?;
    ensure_db_created(configuration).await?;
    Ok(())
}

pub fn ensure_folder_created(configuration: &Settings) -> anyhow::Result<()> {
    let mut directory = configuration.location.to_owned();
    directory.pop();
    fs::create_dir_all(&directory)
        .with_context(|| format!("Failed to create directory at {}", &directory.display()))?;
    Ok(())
}

pub async fn ensure_db_created(configuration: &Settings) -> anyhow::Result<()> {
    let options = SqliteConnectOptions::new()
        .create_if_missing(true)
        .filename(&configuration.location);
    let db_path = options.get_filename().to_owned();
    let pool = SqlitePool::connect_with(options)
        .await
        .with_context(|| format!("Failed to connect to database at {}", &db_path.display()))?;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");
    Ok(())
}
