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
