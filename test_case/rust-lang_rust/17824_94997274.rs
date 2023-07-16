 rust
#![feature(path_ext)]
use std::env;
use std::fs::{File, PathExt};

fn main() {
    let homedir = env::home_dir().expect("the user's home directory could not be determined");
    let configFile = homedir.join(".cf").join("config.json");
    if (!configFile.metadata().unwrap().is_file()) {
        println!("the config file was not present at {}", configFile.display());
    }
    let mut reader = File::open(&configFile);
    let mut reader2 = reader.unwrap();
}
