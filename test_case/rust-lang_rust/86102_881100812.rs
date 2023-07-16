
   Compiling rust-id-test v0.1.0 (/home/jeanluc/Code/rust-id-test)
error: unkown token: 🦀
 --> src/main.rs:2:9
  |
2 |     let 🦀 = "Manish";
  |         ^^
  |
  = help: character 🦀 cannot be used in identifiers

error: unkown token: ➖
 --> src/main.rs:3:30
  |
3 |     println!("Result: {}", 2 ➖ 3);
  |                              ^^
  |
help: Unicode character '➖' (Heavy Minus Sign) looks like '-' (Minus/Hyphen), but it is not
  |
3 |     println!("Result: {}", 2 - 3);
  |                              ^

error: expected pattern, found `=`
 --> src/main.rs:2:11
  |
2 |     let 🦀 = "Manish";
  |            ^ expected pattern

error: aborting due to 3 previous errors

error: could not compile `rust-id-test`

To learn more, run the command again with --verbose.
