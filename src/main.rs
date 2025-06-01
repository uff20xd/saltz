mod finder;
mod error;
use std::{
    str,
    process::{
        Command
    },
};
use users::*;
use finder::recursive_search::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut test = Command::new("echo");

    Ok(())
}
