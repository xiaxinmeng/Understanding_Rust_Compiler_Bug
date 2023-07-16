
   Compiling playground v0.0.1 (/playground)
error[E0005]: refutable pattern in local binding: `&i32::MIN..=0_i32` and `&2_i32..=i32::MAX` not covered
 --> src/main.rs:4:9
  |
4 |     let const { 1 } = &1;
  |         ^^^^^^^^^^^ patterns `&i32::MIN..=0_i32` and `&2_i32..=i32::MAX` not covered
  |
  = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
  = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
  = note: the matched value is of type `&i32`
help: you might want to use `if let` to ignore the variant that isn't matched
  |
4 |     if let const { 1 } = &1 { /* */ }
  |

error: aborting due to previous error

For more information about this error, try `rustc --explain E0005`.
error: could not compile `playground`

To learn more, run the command again with --verbose.
