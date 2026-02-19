use core::str;
use std::{
    fs::{self, *}, io::Write, path::{*}, process::{
        Command, Output,
    },
};
use serde_derive::{Serialize, Deserialize};
use users::*;
use std::fmt;

const FILE_ENDING: &[u8] = b".slz";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Project (String, String);

impl Project {
    fn new (name: String, path: String) -> Self {
        Self(name, path)
    }
    fn get (&self) -> [String; 2] {
        [self.0.clone(), self.1.clone()]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Projects(Vec<Project>);

impl Projects {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn query() -> Self {
        let mut projects = Self::new();
        let _ = projects.set_projects(Self::search_directory(get_home_directory()));
        let _ = projects.save_projects();
        projects
    }
    pub fn get_all_paths() -> () {
        let projects = Self::load_projects().get_vec();
        let mut current_project: [String; 2];
        for current_name in projects {
            current_project = current_name.get();
            println!("{} ; {}", current_project[0], current_project[1]);
        }
    }
    pub fn get_project_path(name: String) -> Result<String, Box<dyn std::error::Error>>{
        let projects = Self::load_projects().get_vec();
        let mut current_project: [String; 2];

        for current_name in projects {
            current_project = current_name.get();
            if current_project[0] == name {
                return Ok(current_project[1].clone());
            }
        }
        Err(Box::new(std::fmt::Error))
    }

    fn set_projects(&mut self, p: Vec<Project>) -> () {
        self.0 = p;
    }

    fn get_vec(self) -> Vec<Project> {
        self.0
    }

    fn save_projects(&mut self) -> () {

        let projects = self.clone();
        let homedirectory = get_home_directory();
        if !Path::new(&(homedirectory.clone() + "/.config/saltz/.projects.ron")).exists() {
            let _ = fs::create_dir(homedirectory.clone() + "/.config/saltz");
        } 
        let mut projects_file = File::create(homedirectory.clone() + "/.config/saltz/.projects.ron")
            .expect("new config file in load setting");
        let new_config_file_contents = ron::to_string::<Projects>(&projects);
        let _ = write!(projects_file, "{}", new_config_file_contents.unwrap());

    }

    fn load_projects() -> Self {
        //get all the files and paths from the "database"
        let homedirectory = get_home_directory();
        let projects_file: String;

        // if the file exists it just reads from it
        if Path::new(&(homedirectory.clone() + "/.config/saltz/.projects.ron")).exists() {
            projects_file = fs::read_to_string( homedirectory.clone() + "/.config/saltz/.projects.ron")
                .expect("Couldnt read the settings file");

            ron::from_str(&projects_file)
                .expect("couldnt Deserialize the Config object")
        }
        // If the File doesnt exist yet it will be created 
        else {
            Self::query()
        }
    }
    //searches all non-hidden files
    fn search_directory (path: String) -> Vec<Project> {
        let mut projects: Vec<Project> = Vec::new();
        let mut list_command: Command = Command::new("ls");
        //println!("{:?}", &path);
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
                                let name = str::from_utf8(&output[slice_start..(slice_end-4)]).unwrap().to_owned();
                                projects.push(Project::new(name.clone(), path.clone()));
                                println!("{} ; {}", name, path.clone());
                                return projects;
                            },
                            _ => ()
                        };
                    }
                    let file_name = path.clone() + "/" + str::from_utf8(&output[slice_start..(slice_end)]).unwrap();
                    let file = Path::new(&file_name);
                    //dbg!(&file);
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
