
error: out of range hex escape
 --> examples/hey.rs:3:7
  |
3 | hey!("\xFF");
  |       ^^^^ must be a character in the range [\x00-\x7f]

error: non-ASCII character in byte constant
 --> examples/hey.rs:4:8
  |
4 | hey!(b"å");
  |        ^
  |        |
  |        byte constant must be ASCII
  |        help: use a \xHH escape for a non-ASCII byte: `\xE5`
