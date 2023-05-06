use std::fs;

use serde_derive::{Deserialize, Serialize};

// debugも対応して
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Source {
    pub deploy_path: String,
    pub name: String,
    pub save_path: String,
    pub source_path: String,
    /* deploy後のcommand. optionのstring */
    pub deploy_command: String,
    /* deploy前にsaveするかどうか。optionのboolean */
    pub is_save_before_deploy: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(rename = "sources")]
    pub sources: Vec<Source>,
}

/* 最初は同じフォルダ。今後は設定を保存できるようにする。./config/manual_rollback */
pub fn read_config(config_path: &str) -> Config {
    let toml_string = fs::read_to_string(config_path);

    let toml_string = match toml_string {
        Ok(toml_string) => toml_string,
        Err(_) => {
            println!("Failed to parse the selected toml file.");
            std::process::exit(1);
        }
    };
    let sources_table = toml::from_str(&toml_string);

    let sources_table = match sources_table {
        Ok(sources_table) => sources_table,
        Err(_) => {
            println!("Failed to parse the selected toml file.");
            std::process::exit(1);
        }
    };
    sources_table
}
