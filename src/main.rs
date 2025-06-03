mod finder;
mod error;
use std::{
    env::{current_exe, set_current_dir}, io, process::{exit, Command}, str
};
use users::*;
use finder::recursive_search::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let mut neovim_command = Command::new("nvim");
    //let mut neovim = neovim_command.arg("src/main.rs").spawn()?;
    //let _ = neovim.wait();
    let test = get_home_directory()+"/programming/nixism";
    let mut tmux_command = Command::new("cd");
    let mut tmux= tmux_command.arg(test).spawn()?;
    let _ = tmux.wait();
    exit(69);
}
