mod add;
mod delete;
mod deploy;
mod read_config;
mod rollback;
mod save;
mod util;

use clap::{Parser, Subcommand};

use crate::{deploy::execute_command, util::select_config};
/*
    -c --config -> config.tomlを定義する。
*/

#[derive(Parser)]
#[command(name = "Manual Rollback Kun")]
#[command(author = "Ou Fumio <araera111@gmail.com>")]
#[command(version = "v1.0.0")]
#[command(about = "This tool allows you to manually backup folders. It can also use data that has been backed up in the past and perform a rollback.", long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
pub struct ConfigPathArgs {
    #[clap(short = 'c', long = "config")]
    pub config: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "save")]
    Save(ConfigPathArgs),
    #[command(name = "rollback")]
    Rollback(ConfigPathArgs),
    #[command(name = "add")]
    Add(ConfigPathArgs),
    #[command(name = "delete")]
    Delete(ConfigPathArgs),
    #[command(name = "deploy")]
    Deploy(ConfigPathArgs),
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Save(config_path)) => {
            let config_path = util::get_config_path(&config_path);
            let config: read_config::Config = read_config::read_config(&config_path);
            config.sources.iter().for_each(|source| {
                save::save_old_data(source);
            });
        }
        Some(Commands::Rollback(config_path)) => {
            let config_path = util::get_config_path(&config_path);
            rollback::rollback(&config_path);
        }
        Some(Commands::Add(config_path)) => {
            let config_path = util::get_config_path(&config_path);
            add::add_config_to_toml(&config_path);
        }
        Some(Commands::Delete(config_path)) => {
            let config_path = util::get_config_path(&config_path);
            delete::delete_config_to_toml(&config_path);
        }
        Some(Commands::Deploy(config_path)) => {
            let config_path = util::get_config_path(&config_path);
            /*
                deployとは
                source dirからdeploy dirにファイルをコピーする。
                また、最後にcommandがあればそれを実行する。
                ex pm2 start ./dist/main.jsなど
                deploy時に保存するかどうかのoption booleanがあっても良い
            */
            let config: read_config::Config = read_config::read_config(&config_path);
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
        None => {
            println!("Please enter a command.");
        }
    }
}
