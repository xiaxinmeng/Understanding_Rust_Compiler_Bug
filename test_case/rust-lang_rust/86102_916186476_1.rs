
error: unknown start of token: \u{2796}
 --> f8.rs:3:30
  |
3 |     println!("Result: {}", 2 ➖ 3);
  |                              ^^
  |
help: Unicode character '➖' (Heavy Minus Sign) looks like '-' (Minus/Hyphen), but it is not
  |
3 |     println!("Result: {}", 2 - 3);
  |                              ~

error: identifiers cannot contain emojis: `🦀`
 --> f8.rs:2:9
  |
2 |     let 🦀 = "Manish";
  |         ^^
