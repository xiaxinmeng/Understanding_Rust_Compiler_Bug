
error: `<` is interpreted as a start of generic arguments for `usize`, not a comparison
 --> test.rs:6:26
  |
6 |     } else if x as usize < pids.len() {
  |               ---------- ^ ----- interpreted as generic arguments
  |               |          |
  |               |          not interpreted as comparison
  |               help: try comparing the casted value: `(x as usize)`

error: aborting due to previous error
