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

fn search_directory (path: PathBuf, hidden_files: bool) -> Vec<PathBuf> {
    let mut directories = Vec::new();
    let mut list_command = Command::new("ls");
    let raw_output = list_command.output().expect("This command should work usually");
    //let raw_string_of_files = str::from_utf8(&output.stdout[0..output.stdout.len()]);
    let output = raw_output.stdout;
    let mut slice_start: usize = 0;
    let mut slice_end: usize = 0;
    while output.len()-1 >= slice_end {
        slice_end += 1;
        let _  = match output[slice_end] {
            b' ' => (),
            _ => ()
        };
    }

    directories
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
