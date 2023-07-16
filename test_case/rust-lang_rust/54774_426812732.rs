
error: expected expression, found reserved keyword `try`
 --> src/main.rs:2:13
  |
2 |     let x = try!(Ok(1u8));
  |             ^^^ expected expression
  help: `try` is a reserved keyword in the 2018 edition, use the `?` operator instead
  |
2 |     let x = Ok(1u8)?;
  |             ^^^^^^^^
  help: alternatively, you can still access the `try!` macro using the raw identifier format
  |
2 |     let x = r#try!(Ok(1u8));
  |             ^^
