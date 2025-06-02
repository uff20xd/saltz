use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = ("None").to_owned() )]
    enter: String,
}

fn start_cli () -> () {
    todo!();
}
