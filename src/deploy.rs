use std::process::Command;

use crate::{
    read_config::Config,
    save,
    util::{self, select_config},
};

/* stringをspaceで区切ったvecにする関数 */
pub fn string_to_vec(string: &str) -> Vec<String> {
    let vec: Vec<String> = string.split_whitespace().map(|s| s.to_string()).collect();
    vec
}

/*
  vecのlengthが1以上のときにcommandを実行する。
  vecの0はcommandに直接渡し、残りはargsにする
*/

fn execute_command(command: &str) {
    let command_list = string_to_vec(command);
    let command = command_list[0].clone();
    let args = &command_list[1..];
    if command_list.len() > 0 {
        let output = Command::new(command).args(args).output().unwrap();
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn deploy(config: &Config) {
    /* deployするものを選択させる */
    let answer_string = select_config(&config, "Which config do you want to deploy?");
    if answer_string.is_err() {
        util::error_stderr(answer_string.unwrap_err());
        return;
    }
    let answer_string = answer_string.unwrap();

    /* configのなかから、deployするものを探す */
    let deploy_source = config
        .sources
        .iter()
        .find(|source| source.name == answer_string);

    if deploy_source.is_none() {
        println!("{} is not found.", answer_string);
        return;
    }

    /* deploy_sourceのsourceをdeploy_pathにコピーする */
    let deploy_source = deploy_source.unwrap();
    let deploy_path = deploy_source.deploy_path.clone();
    let source_path = deploy_source.source_path.clone();

    /* is_save_before_deployがtrueのときは保存する */
    let is_save_before_deploy = deploy_source.is_save_before_deploy;
    if is_save_before_deploy {
        save::save_old_data(deploy_source);
    }

    /* deployする */
    util::overwrite_copy(&source_path, &deploy_path);

    println!("{}", deploy_source.deploy_command);

    /* deploy後のcommandを実行する */
    if deploy_source.deploy_command != "" {
        execute_command(deploy_source.deploy_command.as_str());
    }
    println!("deploying {}", answer_string);
}
