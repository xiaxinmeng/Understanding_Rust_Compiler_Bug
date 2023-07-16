 rust
let output = Command::new("sh")
                     .arg("-c")
                     .arg("echo hello")
                     .output()
                     .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
