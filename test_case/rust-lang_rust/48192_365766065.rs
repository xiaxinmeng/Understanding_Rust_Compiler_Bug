rust
use std::process::Command;

fn main() {
    let s = Command::new("sh")
        .arg("-c")
        .arg("exec 3>&1; exec 2>&-; sleep 600 &")
        .status()
        .unwrap();
    println!("{}", s);
}
