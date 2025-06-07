use core::str;
use std::{
    env, fs::*, io::Write, path::{self, *}, process::{
        Command, Output,
    }
};
use serde::{Serialize, Deserialize};


use users::*;
use crate::error::SaltzError;

const FILE_ENDING: &[u8] = b".slz";

#[derive(Debug)]
struct Project (String, String);

impl Project {
    fn new (name: String, path: String) -> Self {
        Self(name, path)
    }
}

#[derive(Debug)]
pub struct Projects(Vec<Project>);

impl Projects {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn query() -> Result<(), SaltzError> {

        Ok(())
    }
    pub fn get_files (&mut self) -> () {
        self.0 = Projects::search_directory(get_home_directory())
    }
    fn save_projects(&mut self) -> () {
        //opens local file and writes all projects and paths into it
    }
    fn get_paths(&mut self) -> (){
        //get all the files and paths from the "database"
    }
    //searches all non-hidden files
    fn search_directory (path: String) -> Vec<Project> {
        let mut projects: Vec<Project> = Vec::new();
        let mut list_command: Command = Command::new("ls");
        #[path = "../../LICENSE"]
        let raw_output: Output = list_command.current_dir(&path).output().expect("This command should work usually");
        let output: Vec<u8> = raw_output.stdout;
        let mut slice_start: usize = 0;
        let mut slice_end: usize = 0;
        while output.len() >= slice_end + 2 {
            slice_end += 1;
            let _  = match output[slice_end] {
                b'\n' => {
                    let len = (slice_end - 1) - slice_start;
                    if len > 5 {
                        let _ = match &output[(slice_end - 4)..(slice_end)] {
                            FILE_ENDING => {
                                projects.push(Project::new(str::from_utf8(&output[slice_start..(slice_end-4)]).unwrap().to_owned(), path.clone()));
                                return projects;
                            },
                            _ => ()
                        };
                    }
                    let file_name = path.clone() + "/" + str::from_utf8(&output[slice_start..(slice_end)]).unwrap();
                    let file = Path::new(&file_name);
                    dbg!(&file);
                    if file.is_dir() {
                        let mut projects_in_dir = Projects::search_directory(file_name);
                        projects.append(&mut projects_in_dir);
                    }
                    slice_end += 1;
                    slice_start = slice_end;
                },
                _ => ()
            };
        }
    
        projects
    }
}
pub fn get_home_directory () -> String {
    let current_user_as_osstring = get_current_username()
        .expect("couldnt get username");
    let current_user = current_user_as_osstring.to_str()
        .expect("couldnt convert username to a string");
    ("/home/").to_string() + current_user
}


