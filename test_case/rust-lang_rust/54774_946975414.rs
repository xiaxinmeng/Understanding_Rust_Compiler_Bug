text
error: use of deprecated `try` macro
 --> src/main.rs:2:13
  |
2 |     let x = try!(Ok(1u8));
  |             ^^^^^^^^^^^^^
  |
  = note: in the 2018 edition `try` is a reserved keyword, and the `try!()` macro is deprecated
help: you can use the `?` operator instead
  |
2 |     let x = Ok(1u8)?;
  |            --      ^
help: alternatively, you can still access the deprecated `try!()` macro using the "raw identifier" syntax
  |
2 |     let x = r#try!(Ok(1u8));
  |             ^^
