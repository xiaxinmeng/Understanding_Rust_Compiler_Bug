rust
extern crate reqwest;

fn main() {
    let response_text = reqwest::get("https://www.rust-lang.org")
        .expect("Couldn't make request")
        .text().expect("Could not read response text!")
}

