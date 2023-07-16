
test.rs:19:1: 20:2 error: entry symbol `main` defined multiple times
test.rs:19 pub fn main() {
test.rs:20 }
help: did you use #[no_mangle] on `fn main`? Use #[start] instead
error: aborting due to previous error
