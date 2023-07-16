
error: expected identifier, found keyword `let`
 --> src/main.rs:3:5
  |
3 |     let x = 1;
  |     ^^^ expected identifier, found keyword

error: expected type, found keyword `let`
 --> src/main.rs:3:5
  |
2 |     println!("x"): // note the accidental colon
  |                  - help: try using a semicolon: `;`
3 |     let x = 1;
  |     ^^^ expecting a type here because of type ascription
