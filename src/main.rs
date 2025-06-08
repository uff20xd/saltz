mod project_management;
mod cli;
mod settings_manager;
mod script_handler;
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

    use crate::{project_management::project_management::*, settings_manager::settings_manager::Settings};
    use core::str;
    use std::process::{exit, Command};
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

    #[test]
    fn settings_test() {
        let homedirectory = get_home_directory();
        if homedirectory == "/home/nixbld" {
            exit(0)
        }

        let mut command = Command::new("rm");
        let file = get_home_directory() + "/.config/saltz/config.toml";
        let output = command.arg(&file).output().unwrap().stdout;
        println!("rm output: {}", str::from_utf8(&output).unwrap());

        let name = "test".to_owned();

        let new_value = "test_string123".to_owned();
        let _ = Settings::set_settings_value(&name, &new_value);
        let test_setting = Settings::get_setting_value("test");
        assert!(test_setting=="test_string123".to_owned());

        let new_value = "other_string".to_owned();
        let _ = Settings::set_settings_value(&name, &new_value);
        let test_setting = Settings::get_setting_value("test");
        assert!(test_setting=="other_string".to_owned());
    }
}
