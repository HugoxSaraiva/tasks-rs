use std::path::PathBuf;

pub struct Settings {
    pub location: String,
    pub database: DatabaseSettings,
}

impl Settings {
    pub fn new(location: String) -> Self {
        let path = PathBuf::from(location)
            .canonicalize()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
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
