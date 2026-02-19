use std::process::Command;
use crate::project_management::project_management::Projects;

pub struct Script {
    project_path: String,
    script: Vec<Command>
}

impl Script {
    fn exec(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!("TODO: ")
    }
    fn get(script_name: String) -> Result<Self, Box<dyn std::error::Error>> {
        let call = Self {
            project_path: Projects::get_project_path(script_name)?,
            script: Vec::new(),
        };
        Ok(call)
    }
}
