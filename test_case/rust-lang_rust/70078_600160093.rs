
error: entry symbol `main` declared multiple times
 --> src/main.rs:7:1
  |
7 | / fn main() {
8 | |     unsafe { internal::main() }
9 | | }
  | |_^
  |
  = help: did you use `#[no_mangle]` on `fn main`? Use `#[start]` instead
