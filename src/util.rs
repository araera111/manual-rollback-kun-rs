use std::path::PathBuf;

use crate::ConfigPathArgs;

pub fn get_config_path(add: &ConfigPathArgs) -> String {
    let home_dir: PathBuf = dirs::home_dir().unwrap();
    let file_path: PathBuf = home_dir.join(".config/manual_rollback_kun/rollback.toml");
    let config_path = match &add.config {
        Some(config_path) => config_path,
        None => file_path.to_str().unwrap(),
    };
    config_path.to_string()
}
