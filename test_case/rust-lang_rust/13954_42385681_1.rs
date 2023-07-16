
let mut cmd = Command::new("echo");
cmd.arg("hello");
cmd.arg("world");
cmd.spawn();
