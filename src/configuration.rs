use std::path::{self};

pub struct Settings {
    pub location: String,
    pub database: DatabaseSettings,
}
impl Settings {
    pub fn new(location: String) -> Self {
        let path = path::absolute(location)
            .expect("Invalid file location")
            .into_os_string()
            .into_string()
            .expect("Failed to convert location to absolute path");
        Self {
            database: DatabaseSettings::new(&path),
            location: path,
        }
    }
}

pub struct DatabaseSettings {
    pub location: String,
}

impl DatabaseSettings {
    pub fn new(location: &str) -> Self {
        Self {
            location: format!("sqlite:{location}"),
        }
    }
}
