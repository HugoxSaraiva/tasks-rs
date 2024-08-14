use std::{env, path::PathBuf};

pub enum Folder {
    Local,
    LocalSettings,
    Temp,
}

pub fn get_folder_path(folder: Folder) -> PathBuf {
    let app_folder_name = "task-rs";
    let mut folder_name = match folder {
        Folder::Local => dirs::data_local_dir().expect("Failed to retrieve Local folder path"),
        Folder::LocalSettings => {
            dirs::config_local_dir().expect("Failed to retrieve LocalSettings folder path")
        }
        Folder::Temp => env::temp_dir(),
    };
    folder_name.push(app_folder_name);
    folder_name
}
