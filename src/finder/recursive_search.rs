use std::{
    path::{
        PathBuf
    },
    process::{
        Command,
    },
};
use crate::error::SaltzError;

struct Project (String, PathBuf);

fn search_directory (path: PathBuf, hidden_files: bool) -> Vec<PathBuf> {

    let temp = vec![PathBuf::new()];
    temp
}
pub fn get_files () -> () {
    ()
}
fn format (paths: Vec<PathBuf>) -> Result<Vec<Project>, SaltzError> {
    let mut projects = Vec::new();
    for i in paths {

    }
    Ok(projects)
}
pub fn get_home_directory () -> String {
    let current_user_as_osstring = get_current_username()
        .expect("couldnt get username");
    let current_user = current_user_as_osstring.to_str()
        .expect("couldnt convert username to a string");
    ("/home/").to_string() + current_user
}
