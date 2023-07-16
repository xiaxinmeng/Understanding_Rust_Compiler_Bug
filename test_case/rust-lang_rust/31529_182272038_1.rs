
# src/compiletest/runtest.rs
fn split_maybe_args(argstr: &Option<String>) -> Vec<String> {
    match *argstr {
        Some(ref s) => {
            s
             .split(' ')
