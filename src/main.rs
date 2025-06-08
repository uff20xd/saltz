mod finder;
mod error;
mod cli;
use cli::*;
use std::{
    process::{exit}, 
};

fn main() {
    start_cli();
    exit(0);
}

#[cfg(test)]
mod tests {
    use crate::finder::recursive_search::*;
    use std::process::exit;
    #[test]
    fn search_directory() {
        let homedirectory = get_home_directory();
        println!("{}",&homedirectory);
        if homedirectory == "/home/nixbld" {
            exit(0)
        }
        let _ = Projects::query();
    }

    #[test]
    fn open_project() {
        let homedirectory = get_home_directory();
        println!("{}",&homedirectory);
        if homedirectory == "/home/nixbld" {
            exit(0)
        }
        let _ = Projects::query();
        let _ = match Projects::get_project_path("saltz".to_owned()) {
            Ok(n) => n,
            Err(_) => exit(99)
        };
    }
}
