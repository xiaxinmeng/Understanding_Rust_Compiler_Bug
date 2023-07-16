
error: non-ASCII character in byte constant
 --> src/main.rs:2:15
  |
2 |     let x = b"µ";
  |               ^
  |               |
  |               byte constant must be ASCII
  |               help: use a \xHH escape for a non-ASCII byte: `\xB5`
