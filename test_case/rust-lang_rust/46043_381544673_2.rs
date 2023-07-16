shell
warning: borrow of packed field is unsafe (error E0133)
 --> src/main.rs:9:13
  |
9 |     let _ = &a.y;
  |             ^^^^
  |
  = note: #[warn(safe_packed_borrows)] on by default
  = warning: fields of packed structs might be misaligned: dereferencing a misaligned reference is undefined behavior. 
  = note: this is warning will turn into a hard error in the next Rust 1.2x.0 release. Taking references to packed struct fields allows undefined behavior to happen in safe Rust. Fix this with great care - for more information, see issue #46043 <https://github.com/rust-lang/rust/issues/46043>
