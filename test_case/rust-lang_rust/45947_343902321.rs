Rust
error: non-reference pattern used to match a reference (see issue #42640)
   --> src/coord.rs:194:23
    |
194 |             .filter(|&(ref k, _)| self.hash(k) == server)
    |                      ^^^^^^^^^^^ help: consider using: `&&(ref k, _)`
    |
    = help: add #![feature(match_default_bindings)] to the crate attributes to enable

error: aborting due to previous error

error: Could not compile `kv_2pc`.

To learn more, run the command again with --verbose.

