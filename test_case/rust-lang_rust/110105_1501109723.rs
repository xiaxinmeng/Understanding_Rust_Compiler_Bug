plain
---- src/primitive_docs.rs - prim_never (line 134) stdout ----
error[E0005]: refutable pattern in local binding
 --> src/primitive_docs.rs:136:5
  |
5 | let Ok(s) = String::from_str("hello");
  |     ^^^^^ pattern `Err(_)` not covered
  |
  = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
  = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
  = note: the matched value is of type `Result<String, Infallible>`
help: you might want to use `let else` to handle the variant that isn't matched
  |
5 | let Ok(s) = String::from_str("hello") else { todo!() };

error: aborting due to previous error

For more information about this error, try `rustc --explain E0005`.
For more information about this error, try `rustc --explain E0005`.
Couldn't compile the test.

failures:
    src/primitive_docs.rs - prim_never (line 134)

test result: FAILED. 4073 passed; 1 failed; 37 ignored; 0 measured; 0 filtered out; finished in 47.85s

error: doctest failed, to rerun pass `-p core --doc`
