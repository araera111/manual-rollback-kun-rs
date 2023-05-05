use std::{
    fs::{create_dir_all, File},
    path::Path,
};

use fs_extra::file::write_all;

use crate::read_config;

fn make_input_question<'a>(message: &'a str, error: &'a str) -> requestty::Question<'a> {
    requestty::Question::input("")
        .message(message)
        .validate(move |input, _| {
            if input.is_empty() {
                Err(error.into())
            } else {
                Ok(())
            }
        })
        /*
           さらにpathが合法で、かつ該当のディレクトリが存在しているかどうかをみたいか？
        */
        .build()
}

fn get_answer_string(answer: requestty::Answer) -> String {
    answer.as_string().unwrap().to_string()
}

fn make_config_from_questions() -> read_config::Config {
    let question_project_name = make_input_question(
        "What is the name of the project to be saved?",
        "Please enter the project name.",
    );
    let answer_project_name =
        get_answer_string(requestty::prompt_one(question_project_name).unwrap());

    let question_source_path = make_input_question(
        "Where is the source directory of the project?",
        "Please enter the project's source path",
    );

    let answer_source_path =
        get_answer_string(requestty::prompt_one(question_source_path).unwrap());

    let question_deploy_path = make_input_question(
        "What is the path to the project to be deployed?",
        "Please enter the project's deploy path.",
    );

    let answer_deploy_path =
        get_answer_string(requestty::prompt_one(question_deploy_path).unwrap());

    let question_save_path = make_input_question(
        "What is the path to the project to be saved?",
        "Please enter the project's save path.",
    );

    let answer_save_path = get_answer_string(requestty::prompt_one(question_save_path).unwrap());

    let question_after_deploy_command = requestty::Question::input("")
        .message("What is the command to run after deployment?")
        .build();

    let answer_after_deploy_command = requestty::prompt_one(question_after_deploy_command)
        .unwrap()
        .as_string()
        .unwrap()
        .to_string();

    let is_save_before_deploy = requestty::Question::confirm("")
        .message("Do you want to save the project before deployment?")
        .default(true)
        .build();

    let answer_save_before_deploy = requestty::prompt_one(is_save_before_deploy)
        .unwrap()
        .as_bool()
        .unwrap();

    let source = read_config::Source {
        deploy_path: answer_deploy_path,
        name: answer_project_name,
        save_path: answer_save_path,
        source_path: answer_source_path,
        deploy_command: answer_after_deploy_command,
        is_save_before_deploy: answer_save_before_deploy,
    };
    println!("{:?}", source);

    let config = read_config::Config {
        sources: vec![source],
    };
    config
}

fn config_to_toml(config: &read_config::Config) -> String {
    let toml_string = toml::to_string(&config).unwrap();
    toml_string
}

fn create_config_file(config_path: &str, toml_string: &str) {
    let config_path = Path::new(config_path);
    let dir_path = config_path.parent().unwrap();
    create_dir_all(dir_path).unwrap();
    let result = write_all(config_path, &toml_string);
    match result {
        Ok(_) => println!("File created."),
        Err(e) => println!("File creation failed.{}", e),
    }
}

pub fn add_config_to_toml(config_path: &str) {
    /*
      現在のtomlファイルの状態を確認する。
      存在しなかった場合はファイルを作成し、そこに書き込む。
    */
    let path = Path::new(config_path);
    let config = make_config_from_questions();
    let toml_string = config_to_toml(&config);

    /* ファイルが存在しなかった場合は作成する */
    if !path.exists() {
        create_config_file(config_path, &toml_string);
        return;
    }

    /* ファイルは存在したが、ファイルサイズが0のとき */
    if path.exists() {
        let file = File::open(path).unwrap();
        let size = file.metadata().unwrap().len();
        if size == 0 {
            create_config_file(config_path, &toml_string);
            return;
        }

        /* ファイルが存在し、ファイルサイズが0ではないとき */
        /* configからsourcesを取り出す */
        let sources = config.sources;
        let mut config = read_config::read_config(config_path);
        let mut config_sources = config.sources;

        /* sourcesに新しいものをpushする */
        for source in sources {
            config_sources.push(source);
        }

        /* configをtomlに上書きする */
        config.sources = config_sources;
        let toml_string = config_to_toml(&config);
        let result = write_all(config_path, &toml_string);
        match result {
            Ok(_) => println!("File created."),
            Err(e) => println!("File creation failed.{}", e),
        }
    }
}
