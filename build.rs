use std::{
    env,
    path::*,
    fs::*,
    io::Write,
};
fn main () -> () {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    let mut f = File::create(&dest_path).unwrap();
    println!("{}", out_dir);
    f.write_all(b"fn main() {println!(\"Unerwartetes hallo\")}").unwrap()
}
