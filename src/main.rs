mod add;
mod delete;
mod deploy;
mod read_config;
mod rollback;
mod save;
mod util;

use clap::{Parser, Subcommand};

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
            let config: read_config::Config = read_config::read_config(&config_path);
            deploy::deploy(&config);
        }
        None => {
            println!("Please enter a command.");
        }
    }
}
