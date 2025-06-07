mod finder;
mod error;
mod cli;
use cli::*;
use std::{
    fs::{self, *}, io::*, path::Path, process::{exit, Command, Output}, str
};
use finder::recursive_search::*;

fn main() {
    init();
    start_cli();
    exit(0);
}

fn init() {
    let homedirectory = get_home_directory();
    if !Path::new(&(homedirectory.clone() + "/.config/saltz/.projects.toml")).exists() {
        let projects = Projects::query();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_directory() {
        let homedirectory = get_home_directory();
        println!("{}",&homedirectory);
        if homedirectory == "/home/nixbld" {
            exit(0)
        }
        init();
        let mut projects = Projects::query();
        dbg!(projects);
    }

    #[test]
    fn open_project() {
        let homedirectory = get_home_directory();
        println!("{}",&homedirectory);
        if homedirectory == "/home/nixbld" {
            exit(0)
        }
        init();
        let mut projects = Projects::query();
        let mut project_path = match Projects::get_project_path("saltz".to_owned()) {
            Ok(n) => n,
            Err(_) => exit(99)
        };
        dbg!(projects);
        dbg!(project_path);
    }
}
