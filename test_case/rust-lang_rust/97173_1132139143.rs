
error[E0599]: no method named `read_line` found for fn item `fn() -> Stdin {stdin}` in the current scope
  --> src/main.rs:5:24
   |
5  |         std::io::stdin.read_line(&mut inner).unwrap();
   |         -------------- ^^^^^^^^^ method not found in `fn() -> Stdin {stdin}`
   |         |
   |         this is a function, perhaps you wish to call it
...
15 |     read_vec!(v as u32);
   |     ------------------- in this macro invocation
   |
   = note: this error originates in the macro `read_vec` (in Nightly builds, run with -Z macro-backtrace for more info)
