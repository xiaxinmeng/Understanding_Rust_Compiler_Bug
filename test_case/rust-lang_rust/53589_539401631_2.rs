ruby
error[E0506]: cannot assign to `self.value` because it is borrowed
  --> src/cache/mod.rs:27:17
   |
20 |     fn value<U>(&mut self, arg: U) -> &R
   |                 - let's call the lifetime of this reference `'1`
...
23 |         match self.value.as_ref() {
   |               ---------- borrow of `self.value` occurs here
24 |             Some(v) => v,
   |                        - returning this value requires that `self.value` is borrowed for `'1`
...
27 |                 self.value = Some(v);
   |                 ^^^^^^^^^^ assignment to borrowed `self.value` occurs here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0506`.
error: Could not compile `minigrep`.
