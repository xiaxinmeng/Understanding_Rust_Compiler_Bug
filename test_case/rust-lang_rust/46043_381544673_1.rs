shell
warning: borrow of packed field requires unsafe function or block (error E0133)
 --> src/main.rs:9:13
  |
9 |     let _ = &a.y;
  |             ^^^^
  |
  = note: #[warn(safe_packed_borrows)] on by default
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: for more information, see issue #46043 <https://github.com/rust-lang/rust/issues/46043>
