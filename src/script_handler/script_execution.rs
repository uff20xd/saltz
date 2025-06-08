use core::str;
use std::process::Command;
use std::str::from_utf8;

struct SingleCommand(String, Vec<String>);

impl SingleCommand {
    pub fn execute (&self) -> String {
        let mut command = Command::new(self.0.clone());
        for i in 0..self.1.len()-1 {
            let __ = command.arg(self.1[i].clone());
        }
        let output = command.output().unwrap().stdout;
        str::from_utf8(&output).unwrap().to_owned()
    }
}

pub struct Script {
    current_path: String,
    script: Vec<SingleCommand>
}
impl Script {
    fn run_project(project_name: String) {
        
    }
    fn run_path(path: Script, script_name: String) {

    }

}
