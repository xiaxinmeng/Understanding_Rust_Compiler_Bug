rust
use std::env;
use std::process::{self, Command};

fn main() {
    let status = Command::new("cmd")
        .arg("/c")
        .arg("emcc.bat")
        .args(&env::args().skip(1).collect::<Vec<_>>())
        .status()
        .expect("failed to spawn `cmd`");
    process::exit(status.code().unwrap());
}
