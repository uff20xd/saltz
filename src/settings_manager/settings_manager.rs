use crate::project_management::project_management::*;
use std:: {
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
            test: "ballball".to_owned(),
        }
    }
    pub fn set_settings_value(&mut self, setting: &String, new_value: &String) -> () {
        let _ = match setting as &str {
            "editor" => {
                self.editor = new_value.clone();
            },
            "test" => {
                self.test = new_value.clone();
            },
            _ => {
                println!("Setting \"{}\" not found.", setting.clone());
            }
        };
    }
    
    pub fn get_setting_value(&self, setting: &str) -> String {
        match setting {
            "editor" => self.editor.clone(),
            "test" => self.test.clone(),
            _ => "".to_owned()
        }
    }

    pub fn load_settings () -> Result<Self, Box<dyn std::error::Error>> {
        //get all the files and paths from the "database"
        let projects_file: String;
        let mut projects_file_path = get_home_directory();
        projects_file_path.as_mut_os_string().push("/.config/saltz/config.ron");
        // if the file exists it just reads from it
        if projects_file_path.exists() {
            projects_file = fs::read_to_string(projects_file_path)
                .expect("Couldnt read the settings file");

            match ron::from_str(&projects_file) {
                Ok(this) => { return Ok(this) },
                Err(err) => { return Err(err.into()) }
            }
        }
        // If the File doesnt exist yet it will be created 
        else {
            Ok(Self::new())
        }
    }
    fn save_settings (&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut projects_file_path = get_home_directory();
        projects_file_path.as_mut_os_string().push("/.config/saltz/config.ron");
        if projects_file_path.exists() {
            let _ = fs::create_dir(&projects_file_path);
        } 
        let mut projects_file = File::create(projects_file_path)?;
        let new_config_file_contents = ron::to_string::<Self>(&self);
        write!(projects_file, "{}", new_config_file_contents.unwrap())?;
        Ok(())
    }
}

impl Drop for Settings {
    fn drop(&mut self) {
        _ = self.save_settings();
    }
}
