
test.rs:3:1: 5:2 error: entry symbol `main` defined multiple times
test.rs:3 fn main() {
test.rs:4     println!("hello world");
test.rs:5 }
help: did you use #[no_mangle] on `fn main`? Use #[start] instead
error: aborting due to previous error
