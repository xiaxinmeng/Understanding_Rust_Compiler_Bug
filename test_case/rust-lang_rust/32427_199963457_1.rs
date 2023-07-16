 rust
use std::io::prelude::*;
use std::fs::File;

fn main(){
    println!("hello world!");
    let mut f = File::open("file-ÿ.rs").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    println!("{}",s);
}

