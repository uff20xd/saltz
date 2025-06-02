mod finder;
mod error;
use std::{
    env::{current_exe, set_current_dir}, process::Command, str
};
use users::*;
use finder::recursive_search::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    //let test = set_current_dir("");
    println!("{:?}", current_exe());
    loop {
        print!("a");
    }
    Ok(())
}
