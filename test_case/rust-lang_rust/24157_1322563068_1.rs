
   Compiling playground v0.0.1 (/playground)
error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): `match` arms have incompatible types
  --> src/main.rs:9:20
   |
7  | /     match std::io::stdin().read_line(&mut input) {
8  | |         Ok(u) => u,
   | |                  - this is found to be of type `usize`
9  | |         Err(_e) => exit_wrapper(),
   | |                    ^^^^^^^^^^^^^^ expected `usize`, found `()`
10 | |     };
   | |_____- `match` arms have incompatible types

For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground` due to previous error
