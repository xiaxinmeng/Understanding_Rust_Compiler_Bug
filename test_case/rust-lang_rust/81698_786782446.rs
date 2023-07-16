rust
error: unmatched angle bracket
 --> src/lib.rs:8:10
  |
8 |     foo::<(20 * 100 + 20 * 10 + 1)>(); // ok: const expression contains no generic parameters
  |          ^ help: remove extra angle bracket
