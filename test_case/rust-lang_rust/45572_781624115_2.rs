rust 
// src/main.rs

fn main() {
    let mut command = std::process::Command::new("./target/debug/printer");
    let _res = command
        .stdout(std::process::Stdio::piped())
        .status()
        .unwrap();
}
