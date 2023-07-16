 rust
fn echo_foo_into_pipe(pipe: &SomePipeObject) {
    let command = Command::new("echo");
    // ...have it talk to the pipe somehow
}
