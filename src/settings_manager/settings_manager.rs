use crate::project_management::project_management::*;
use std:: {
    path::*,
    fs::{self, *},
    io::Write,
};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    editor: String,
    test: String
}
impl Settings {
    fn new () -> Self {
        Self {
            editor: "nvim".to_owned(),
            test: "".to_owned(),
        }
    }
    pub fn set_settings_value(setting: &String, new_value: &String) -> () {
        let mut settings = Self::load_settings();
        let _ = match setting as &str {
            "editor" => {
                settings.editor = new_value.clone();
            },
            "test" => {
                settings.test = new_value.clone();
            },
            _ => {
                println!("Setting \"{}\" not found.", setting.clone());
            }
        };
        let _ = settings.save_settings();
    }
    
    pub fn get_setting_value(setting: &str) -> String {
        let settings = Self::load_settings();
        match setting {
            "editor" => settings.editor,
            "test" => settings.test,
            _ => "".to_owned()
        }
    }

    fn load_settings () -> Self {
        //get all the files and paths from the "database"
        let homedirectory = get_home_directory();
        let projects_file: String;

        // if the file exists it just reads from it
        if Path::new(&(homedirectory.clone() + "/.config/saltz/config.toml")).exists() {
            projects_file = fs::read_to_string( homedirectory.clone() + "/.config/saltz/config.toml")
                .expect("Couldnt read the settings file");

            toml::from_str(&projects_file)
                .expect("couldnt Deserialize the Config object")
        }
        // If the File doesnt exist yet it will be created 
        else {
            Self::new()
        }
    }
    fn save_settings (self) -> () {
        let homedirectory = get_home_directory();
        if !Path::new(&(homedirectory.clone() + "/.config/saltz")).exists() {
            let _ = fs::create_dir(homedirectory.clone() + "/.config/saltz");
        } 
        let mut projects_file = File::create(homedirectory.clone() + "/.config/saltz/config.toml")
            .expect("new config file in load setting");
        let new_config_file_contents = toml::to_string::<Self>(&self);
        let _ = write!(projects_file, "{}", new_config_file_contents.unwrap());
    }
    
}
