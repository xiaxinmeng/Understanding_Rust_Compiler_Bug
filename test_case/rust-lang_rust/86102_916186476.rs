
error: expected one of `,`, `.`, `?`, or an operator, found `➖`
 --> f8.rs:3:30
  |
3 |     println!("Result: {}", 2 ➖ 3);
  |                              ^^ expected one of `,`, `.`, `?`, or an operator

error: identifiers cannot contain emojis: `➖`
 --> f8.rs:3:30
  |
3 |     println!("Result: {}", 2 ➖ 3);
  |                              ^^

error: identifiers cannot contain emojis: `🦀`
 --> f8.rs:2:9
  |
2 |     let 🦀 = "Manish";
  |         ^^
