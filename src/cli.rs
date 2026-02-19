use clap::{Parser, Subcommand};
use std::process::{exit, Command};
use crate::project_management::project_management::Projects;
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
    Search {
        #[arg(long, short = 'd', default_value_t = true)]
        hidden: bool
    },
    Get {
        #[arg(long, default_value_t = ("").to_owned() )]
        name: String,
    },
    Config {
        setting: String,
        new_value: String
    },
    Run {
        script: String,

        #[arg(long, default_value_t = (".").to_owned() )]
        path: String,
    },
    Test,
}

pub fn start_cli () -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut settings = Settings::load_settings()?;
    match &cli.command {
        CliArgs::Search { hidden } => {
            let _output = Projects::query(*hidden);
        },
        CliArgs::Enter { name } => {
            let path = match Projects::get_project_path(name.clone()) {
                Ok(heh) => heh,
                Err(_) => {
                    println!("No Project with the name: {}.", name);
                    exit(99)
                }
            };
            let editor = settings.get_setting_value("editor");
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
                print!("{}", path.display());
            }
        },
        CliArgs::Config { setting, new_value } => {
            settings.set_settings_value(setting, new_value);
        },
        CliArgs::Run { script, path } => {
            print!("{}{}", script, path);
        }
        CliArgs::Test => {
            let directory = std::fs::read_dir("/home/uff20xd")?;
            let _: Vec<_> = directory
                .map(|file| {
                    let u_file = file.unwrap();
                    println!("file: {}", u_file.path().display())
                })
                .collect();

        }
    }
    Ok(())
}
