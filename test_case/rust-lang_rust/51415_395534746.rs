
error[E0658]: non-reference pattern used to match a reference (see issue #42640)
 --> src/main.rs:6:37
  |
6 |     let _ = match junk.iter().find(|(_, path)| path.is_none()) {
  |                                     ^^^^^^^^^^^^^ help: consider using a reference: `&(_, path)`
