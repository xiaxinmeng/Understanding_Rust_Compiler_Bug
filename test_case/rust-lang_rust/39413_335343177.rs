rust
error: no rules expected the token `TUPLE`
  --> src/errors.rs:3:1
   |
3  | / error_chain! {
4  | |     
5  | |     foreign_links {
6  | |         IoError(::std::io::Error);
...  |
28 | |     }
29 | | }
   | |_^
   |
   = note: this error originates in a macro outside of the current crate

error: aborting due to previous error
