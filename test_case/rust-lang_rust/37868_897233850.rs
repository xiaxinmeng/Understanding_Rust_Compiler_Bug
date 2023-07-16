rust
let mut my_command = Command::new("./run.sh");
my_command.status().unwrap();  // the exe in this directory
my_command.current_dir("foo");
my_command.status().unwrap();  // the same exe on Windows, or the other one on Unix
