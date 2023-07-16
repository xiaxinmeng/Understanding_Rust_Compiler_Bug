rust
let mut cmd = Command::new("proc").args(["-a", "-b"]);

if some_condition {
    cmd.arg("-c");
}

// return here without using the command
// #[must_use] doesn't trigger because we "used" the `Command` to add an extra arg
