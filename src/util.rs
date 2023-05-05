use std::path::PathBuf;

use fs_extra::dir::{copy, CopyOptions};

use crate::{read_config::Config, ConfigPathArgs};

pub fn get_config_path(add: &ConfigPathArgs) -> String {
    let home_dir: PathBuf = dirs::home_dir().unwrap();
    let file_path: PathBuf = home_dir.join(".config/manual_rollback_kun/rollback.toml");
    let config_path = match &add.config {
        Some(config_path) => config_path,
        None => file_path.to_str().unwrap(),
    };
    config_path.to_string()
}

pub fn get_config_name(config: &Config) -> Result<Vec<String>, &'static str> {
    let config_names = config
        .sources
        .iter()
        .map(|source| source.name.clone())
        .collect::<Vec<String>>();

    match config_names.is_empty() {
        true => Err("There are no config files to rollback."),
        false => Ok(config_names),
    }
}

pub fn error_stderr(error: &str) {
    eprintln!("{}", error);
    std::process::exit(1);
}

/* 成功していたらanswer_string, 失敗のときは失敗のstringを返すResultの関数 */
pub fn select_config<'a>(config: &'a Config, message: &str) -> Result<String, &'a str> {
    let config_names = get_config_name(&config);

    /* config_namesがErrのときは、それをそのまま返しましょう */
    let config_names = match config_names {
        Ok(config_names) => config_names,
        Err(error) => return Err(error),
    };

    let question = requestty::Question::select("select")
        .message(message)
        .choices(config_names)
        .build();

    let answer = requestty::prompt_one(question).unwrap();
    let answer_string = &answer.as_list_item().unwrap().text;

    Ok(answer_string.clone())
}

pub fn overwrite_copy(from_path: &str, to_path: &str) {
    let options = CopyOptions {
        overwrite: true,
        copy_inside: true,
        content_only: true,
        skip_exist: true,
        ..CopyOptions::new() // 残りのフィールドはデフォルト値を使う
    };
    let r = copy(from_path, to_path, &options);
    match r {
        Ok(_) => println!("copy success!"),
        Err(e) => println!("copy error: {}", e),
    }
}
