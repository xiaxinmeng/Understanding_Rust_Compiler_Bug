
error: expected expression, found reserved keyword `try`
 --> src/main.rs:2:13
  |
2 |     let x = try!(Ok(1u8));
  |             ^^^ expected expression
  help: the `try!` macro has been removed from the language, use the `?` operator instead
  |
2 |     let x = Ok(1u8)?;
  |             ^^^^^^^^
