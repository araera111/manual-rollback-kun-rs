use std::path;

use crate::read_config;

pub fn rollback(config_path: &str) {
    let config: read_config::Config = read_config::read_config(&config_path);
    let config_name_list = config
        .sources
        .iter()
        .map(|source| source.name.clone())
        .collect::<Vec<String>>();

    /* listが空のときはエラーで終了 */
    if config_name_list.is_empty() {
        println!("There are no config files to rollback.");
        std::process::exit(1);
    }

    let question = requestty::Question::select("rollback_target")
        .message("Which project do you want to rollback?")
        .choices(config_name_list)
        .build();

    let answer = requestty::prompt_one(question).unwrap();
    let answer_string = &answer.as_list_item().unwrap().text;

    let selected_config = config
        .sources
        .iter()
        .find(|source| source.name == *answer_string)
        .unwrap();

    /* 選択されたnameのリストを表示する。save_pathを読み込み、フォルダ名をvecにする。そのvecでもう一度質問する */

    /*
       selected_configにあるsavedirに、answer_stringのdirがある。
       そのdirにあるdir一覧をvecにする
    */
    /* path joinする。selected_configのsaveとanswer_stringをjoin */
    let save_dir_path = path::Path::new(&selected_config.save_path);
    let answer_name_path = path::Path::new(&answer_string);
    let save_path = path::Path::new(&save_dir_path).join(answer_name_path);
    let saved_list = std::fs::read_dir(save_path).unwrap();
    let saved_vec = saved_list
        .map(|path| path.unwrap().path())
        .collect::<Vec<std::path::PathBuf>>();

    let question = requestty::Question::select("rollback_target_2")
        .message("Which saved data do you want to rollback?")
        .choices(
            saved_vec
                .iter()
                .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
                .collect::<Vec<String>>(),
        )
        .build();

    let answer = requestty::prompt_one(question).unwrap();
    let answer_string = &answer.as_list_item().unwrap().text;

    let rollback_path = saved_vec
        .iter()
        .find(|path| path.file_name().unwrap().to_str().unwrap().to_string() == *answer_string)
        .unwrap();

    let deploy_path = &selected_config.deploy_path;
    let options = fs_extra::dir::CopyOptions {
        overwrite: true,
        copy_inside: true,
        content_only: true,
        skip_exist: true,
        ..fs_extra::dir::CopyOptions::new() // 残りのフィールドはデフォルト値を使う
    };
    fs_extra::dir::copy(rollback_path, deploy_path, &options).unwrap();
}
