
error: `<` is interpreted as a start of generic arguments for `u16`, not a shift
 --> src/main.rs:2:29
  |
2 |     println!("{}", x as u16 << 4);
  |                    -------- ^^ - interpreted as generic arguments
  |                    |        |
  |                    |        not interpreted as shift
  |                    help: try shifting the casted value: `(x as u16)`
