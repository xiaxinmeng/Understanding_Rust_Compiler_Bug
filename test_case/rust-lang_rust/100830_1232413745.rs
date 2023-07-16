console
error[E0308]: mismatched types
   --> /home/cat/rust/library/alloc/src/macros.rs:117:9
    |
114 | macro_rules! format {
    | ------------------- in this expansion of `format!`
...
117 |         res
    |         ^^^
    |         |
    |         expected `&str`, found struct `String`
    |         help: consider borrowing here: `&res`
    |
   ::: a.rs:5:15
    |
5   |     takes_str(format!("e"));
    |               ------------ in this macro invocation

error: aborting due to previous error
