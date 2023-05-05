use crate::{
    read_config,
    util::{error_stderr, select_config},
};

pub fn delete_config_to_toml(config_path: &str) {
    let config: read_config::Config = read_config::read_config(&config_path);

    let answer_string = select_config(&config, "Which config do you want to delete?");

    if answer_string.is_err() {
        error_stderr(answer_string.unwrap_err());
        return;
    }

    let answer_string = answer_string.unwrap();

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
