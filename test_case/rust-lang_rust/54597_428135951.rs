rust
#![feature(nll)]

extern crate serde_json;

fn _foo(val: serde_json::Value) {
    let _reviewers_original: Vec<serde_json::Value> = match val["reviewers"].as_array() {
        Some(array) => {
            println!("Reviewers: {:?}", array);
            *array
        }
        None => vec![]
    };
}

fn main() {
    println!("Hello, world!");
}
