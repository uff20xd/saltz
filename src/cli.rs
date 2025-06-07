use clap::{command, Parser, Subcommand};

use crate::finder::recursive_search::Projects;

#[derive(Parser, Debug)]
#[command(name = "Saltz", version, about = "A Project Management Program", long_about = None)]
struct Cli {
    /// Enter a Command
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Enter a Project by its Name
    Enter {
        /// Name of the Project
        name: String
    },
    /// Searches all Projects in Home
    Search 
}

fn start_cli () -> () {
    let cli = Cli::parse();
    match &cli.command {
        Command::Search => {
            let output = Projects::query();
        },
        Command::Enter { name } => {

        }
    }
}
