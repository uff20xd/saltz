use clap::{command, Parser, Subcommand};
use std::process::{exit, Command};
use crate::finder::recursive_search::Projects;
use crate::settings_manager::settings_manager::Settings;

#[derive(Parser, Debug)]
#[command(name = "Saltz", version, about = "A Project Management Program", long_about = None)]
struct Cli {
    /// Enter a Command
    #[command(subcommand)]
    command: CliArgs,

}

#[derive(Subcommand, Debug)]
enum CliArgs{
    /// Enter a Project by its Name
    Enter {
        /// Name of the Project
        name: String
    },
    /// Searches all Projects in Home
    Search,

    Get {
        #[arg(long, default_value_t = ("").to_owned() )]
        name: String,
    },
    Config {
        setting: String,
        new_value: String
    }
}

pub fn start_cli () -> () {
    let cli = Cli::parse();
    match &cli.command {
        CliArgs::Search => {
            let _output = Projects::query();
        },
        CliArgs::Enter { name } => {
            let path = match Projects::get_project_path(name.clone()) {
                Ok(heh) => heh,
                Err(_) => {
                    println!("No Project with the name: {}.", name);
                    exit(99)
                }
            };
            let editor = Settings::get_setting_value("editor");
            let mut nvim = Command::new(editor);
            let mut nvim_process = nvim.current_dir(&path).arg(".").spawn().unwrap();
            let _ = nvim_process.wait();
        },
        CliArgs::Get {name} => {
            if name == "" {
                let _ = Projects::get_all_paths();
            } else {
                let name = name.clone();
                let path = match Projects::get_project_path(name){
                    Ok(heh ) => heh,
                    Err(_) => {
                        exit(99)
                    }
                };
                print!("{}", path);
            }
        },
        CliArgs::Config { setting, new_value } => {
            Settings::set_settings_value(setting, new_value);
        }
    }
}
