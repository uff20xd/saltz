use clap::{command, Parser, Subcommand};
use std::process::{exit, Command};
use crate::finder::recursive_search::Projects;

#[derive(Parser, Debug)]
#[command(name = "Saltz", version, about = "A Project Management Program", long_about = None)]
struct Cli {
    /// Enter a Command
    #[command(subcommand)]
    command: CliArgs,

    #[arg(long)]
    name: String,
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

    Get
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
            let mut nvim = Command::new("nvim");
            let mut nvim_process = nvim.current_dir(&path).arg(".").spawn().unwrap();
            let _ = nvim_process.wait();
        },
        CliArgs::Get => {
            let path = Projects::get_all_paths();
        },
    }
}
