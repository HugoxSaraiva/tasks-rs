use std::path::PathBuf;

pub struct Settings {
    pub location: PathBuf,
}
impl Settings {
    pub fn new(location: PathBuf) -> Self {
        Self { location }
    }
}

pub struct DatabaseSettings {
    pub location: String,
}
