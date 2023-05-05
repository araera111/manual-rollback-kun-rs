use crate::read_config;

pub fn delete_config_to_toml(config_path: &str) {
    let config: read_config::Config = read_config::read_config(&config_path);

    let config_names = config
        .sources
        .iter()
        .map(|source| source.name.clone())
        .collect::<Vec<String>>();

    /* config_namesが空のときはエラーを表示し、終了する。 */
    if config_names.is_empty() {
        println!("There are no config files to delete.");
        std::process::exit(1);
    }

    let question = requestty::Question::select("overwrite")
        .message("Conflict on `file.rs`")
        .choices(config_names)
        .build();

    let answer = requestty::prompt_one(question).unwrap();
    let answer_string = &answer.as_list_item().unwrap().text;

    println!("{}", answer_string);

    /* configからsourcesを取得し、それのnameとanswer_stringが一致していたものはfilterで削除する */
    let new_sources = config
        .sources
        .iter()
        .filter(|source| source.name != *answer_string)
        .map(|source| source.clone())
        .collect::<Vec<read_config::Source>>();

    let new_config = read_config::Config {
        sources: new_sources,
    };

    let toml_string = toml::to_string(&new_config).unwrap();
    std::fs::write(&config_path, toml_string).unwrap();

    println!("delete")
}
