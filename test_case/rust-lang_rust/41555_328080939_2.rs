rust
// main.rs
extern crate reqwest;

fn main() {
    let x = reqwest::get("https://google.com");
}
