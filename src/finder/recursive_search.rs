use core::str;
use std::{
    path::PathBuf,
    process::{
        Command, Output,
    },
};
use users::*;
use crate::error::SaltzError;

struct Project (String, PathBuf);

pub struct Projects(Vec<Project>);

impl Projects {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn get_files (&mut self) -> () {
    }
    fn format (paths: Vec<PathBuf>) -> Result<Vec<Project>, SaltzError> {
        let mut projects: Vec<Project> = Vec::new();
        for i in paths {
        }
        Ok(projects)
    }
    pub fn save_paths(&mut self) -> () {
        //opens local file and writes all projects and paths into it
    }
    pub fn get_paths(&mut self) -> (){
        //get all the files and paths from the "database"
    }
}
fn search_directory (path: PathBuf, hidden_files: bool) -> Vec<PathBuf> {
    let mut directories: Vec<PathBuf> = Vec::new();
    let mut list_command: Command = Command::new("ls");
    let raw_output: Output = list_command.output().expect("This command should work usually");
    let output: Vec<u8> = raw_output.stdout;
    let mut slice_start: usize = 0;
    let mut slice_end: usize = 0;
    let mut files: Vec<String> = Vec::new();
    while output.len()-1 >= slice_end {
        slice_end += 1;
        let _  = match output[slice_end] {
            b' ' => {
                let file = str::from_utf8(&output[slice_start..slice_end]).expect("should be a string in string in search directory").to_owned();
                files.push(file);
                slice_start = slice_end;
            },
            _ => ()
        };
    }

    directories
}
pub fn get_home_directory () -> String {
    let current_user_as_osstring = get_current_username()
        .expect("couldnt get username");
    let current_user = current_user_as_osstring.to_str()
        .expect("couldnt convert username to a string");
    ("/home/").to_string() + current_user
}



