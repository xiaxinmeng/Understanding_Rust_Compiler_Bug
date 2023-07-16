
error: lifetime bound not satisfied
  --> src/lib.rs:42:8
   |
42 |     fn fails1<F, T>(&self, f: F) -> T
   |        ^^^^^^
   |
note: the lifetime `'b` defined here...
  --> src/lib.rs:44:20
   |
44 |         F: for<'a, 'b> FnOnce(&'b Self::Bar<'a, 'b>) -> T,
   |                    ^^
note: ...must outlive the lifetime `'a` defined here
  --> src/lib.rs:44:16
   |
44 |         F: for<'a, 'b> FnOnce(&'b Self::Bar<'a, 'b>) -> T,
   |                ^^
   = note: this is a known limitation that will be removed in the future (see issue #100013 <https://github.com/rust-lang/rust/issues/100013> for more information)

error: lifetime bound not satisfied
  --> src/lib.rs:49:8
   |
49 |     fn fails2(&self)
   |        ^^^^^^
   |
note: the lifetime `'b` defined here...
  --> src/lib.rs:51:17
   |
51 |         for<'a, 'b> Self::Bar<'a, 'b>: Default,
   |                 ^^
note: ...must outlive the lifetime `'a` defined here
  --> src/lib.rs:51:13
   |
51 |         for<'a, 'b> Self::Bar<'a, 'b>: Default,
   |             ^^
   = note: this is a known limitation that will be removed in the future (see issue #100013 <https://github.com/rust-lang/rust/issues/100013> for more information)

error: could not compile `gat-lifetime-error` due to 2 previous errors
