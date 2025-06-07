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
    let test = get_home_directory()+"/programming/selfanl";
    let mut test2 = Projects::new();
    test2.get_files();
    println!("{:?}", test2);
    exit(69);
}
