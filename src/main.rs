mod finder;
mod error;
mod cli;
use cli::*;
use std::{
    env::{current_exe, set_current_dir}, io, process::{exit, Command}, str
};
use users::*;
use finder::recursive_search::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let mut neovim_command = Command::new("nvim");
    //let mut neovim = neovim_command.arg("src/main.rs").spawn()?;
    //let _ = neovim.wait();
    exit(69);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_directory() {
        let mut projects = Projects::new();
        projects.get_files();
    }
}
