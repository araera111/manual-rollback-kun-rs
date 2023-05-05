use std::process::Command;

/* stringをspaceで区切ったvecにする関数 */
pub fn string_to_vec(string: &str) -> Vec<String> {
    let vec: Vec<String> = string.split_whitespace().map(|s| s.to_string()).collect();
    vec
}

/*
  vecのlengthが1以上のときにcommandを実行する。
  vecの0はcommandに直接渡し、残りはargsにする
*/

pub fn execute_command(command: &str) {
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
