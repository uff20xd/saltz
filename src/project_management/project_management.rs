use std::{
    fs::{self, *}, io::Write, path::{*},
};
use serde_derive::{Serialize, Deserialize};
use users::*;
// use ron::ser::Error;
//
const FILE_ENDING: &[u8] = b".slz";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Project (String, PathBuf);

impl Project {
    fn new (name: String, path: PathBuf) -> Self {
        Self(name, path)
    }
    fn get (&self) -> (String, PathBuf) {
        (self.0.clone(), self.1.clone())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Projects(Vec<Project>);

impl Projects {
    pub fn query(search_hidden: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let mut projects = Vec::new();
        Self::search_directory(&mut projects, &get_home_directory(), search_hidden)?;
        let projects = Self(projects);
        let _ = projects.save_projects();
        Ok(projects)
    }
    pub fn get_all_paths() -> Result<(), Box<dyn std::error::Error>> {
        let projects = Self::load_projects()?.get_vec();
        let mut project_name: String;
        let mut project_path: PathBuf;
        for current_project in projects {
            (project_name, project_path) = current_project.get();
            println!("{:<20} {}", project_name, project_path.display());
        }
        Ok(())
    }
    pub fn get_project_path(name: String) -> Result<PathBuf, Box<dyn std::error::Error>>{
        let projects = Self::load_projects()?.get_vec();
        let mut project_name: String;
        let mut project_path: PathBuf;

        for current_project in projects {
            (project_name, project_path) = current_project.get();
            if project_name == name {
                return Ok(project_path.clone());
            }
        }
        Err(Box::new(std::fmt::Error))
    }

    fn get_vec(self) -> Vec<Project> {
        self.0
    }

    fn save_projects(&self) -> () {

        let projects = self.clone();
        let mut homedirectory = get_home_directory();
        homedirectory.as_mut_os_string().push("/.config/saltz");
        if homedirectory.exists() {
            let _ = fs::create_dir(homedirectory.clone());
        } 
        homedirectory.as_mut_os_string().push("/.projects.ron");
        let mut projects_file = File::create(homedirectory)
            .expect("new config file in load setting");
        // dbg!(&projects);
        let new_config_file_contents = match ron::to_string::<Projects>(&projects) {
            Ok(content) => content,
            //Err(ron::ser::Error::UnsupportedType(None)) => panic!("Didnt find any Projects."),
            Err(err) => {
                dbg!(err);
                eprintln!("No Projects found.");
                return
            },
        };
        write!(projects_file, "{}", new_config_file_contents).unwrap_or_else(|err| panic!("Error while caching projects: {}", err));

    }

    fn load_projects() -> Result<Self, Box<dyn std::error::Error>>{
        //get all the files and paths from the "database"
        let projects_file: String;
        let mut projects_file_path = get_home_directory();
        projects_file_path.as_mut_os_string().push("/.config/saltz/.projects.ron");
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
        Self::query(false)
    }

    fn search_directory(projects: &mut Vec<Project>, path: &Path, search_hidden: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut directory = fs::read_dir(path)?;
        while let Some(file) = directory.next() {
            let u_file = file?;
            let file_path = u_file.path();
            let file_name = u_file.file_name();
            let file_type = u_file.metadata()?;
            let true_name = match file_name.clone().into_string() {
                Ok(name) => name,
                Err(_) => continue,
            };
            let true_name_start = match true_name.chars().next() {
                Some(character) => character,
                None => continue,
            };


            // println!("{:>10}: {}", file_name.display(), file_path.display());

            if file_type.is_dir() {
                if !(true_name_start == '.') || (search_hidden && true_name.len() >= 1) {
                    // println!("{:>10}: {}", file_name.display(), file_path.display());
                    Self::search_directory(projects, &file_path, search_hidden)?;
                } else { continue }
            }
            else if file_type.is_symlink() { continue }
            // file_path.ends_with(".slz") && 
            if true_name.len() < 4 { continue }
            let slz_test = &true_name.as_bytes()[true_name.len() - 4..];
            // println!("{:>10}: {:?}, {:?}", &true_name, slz_test, FILE_ENDING);
            if FILE_ENDING == slz_test {
                let project_name = true_name[..true_name.len()-4].to_owned();
                println!("{:<20} {}", &project_name, file_path.display());
                projects.push(Project::new(project_name, u_file.path()));
            }
        }
        Ok(())
    }
}

pub fn get_home_directory () -> PathBuf{
    let current_user_as_osstring = get_current_username()
        .expect("couldnt get username");
    let current_user = current_user_as_osstring.to_str()
        .expect("couldnt convert username to a string");
    (("/home/").to_string() + current_user).into()
}
