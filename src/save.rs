use fs_extra::dir::{copy, CopyOptions};

use crate::{
    read_config,
    util::{error_stderr, select_config},
};

pub fn save_source(source: &read_config::Source) {
    // 今のデータを保存する
    let source_path = &source.source_path;
    let save_path = &source.save_path;
    let directory_name = chrono::Local::now().format("%Y-%m-%d-%H-%M-%S").to_string();
    let save_path = format!("{}/{}/{}", save_path, source.name, directory_name);

    /*
       fs-extraを使用し、source_pathにあるすべてのファイルを
       save_path/YYYY-MM-DD-HH-MM-SSにコピーする
    */
    let options = CopyOptions {
        overwrite: true,
        copy_inside: true,
        content_only: true,
        skip_exist: true,
        ..CopyOptions::new() // 残りのフィールドはデフォルト値を使う
    };
    let r = copy(source_path, save_path, &options);
    match r {
        Ok(_) => println!("copy success!"),
        Err(e) => println!("copy error: {}", e),
    }
}

pub fn save_old_data(config: &read_config::Config) {
    let answer_string = select_config(&config, "Which config do you want to delete?");

    if answer_string.is_err() {
        error_stderr(answer_string.unwrap_err());
        return;
    }

    let answer_string = answer_string.unwrap();

    /* configから今回のsourceを確定する */
    let source = config
        .sources
        .iter()
        .find(|source| source.name == answer_string)
        .unwrap();
    save_source(&source)
}
