
warning: the trait `FutureExt` cannot be made into an object
  --> foo.rs:11:5
   |
11 | /     fn poll_unpin(&mut self) where Self: Unpin {
12 | |         PinMut::new(self).poll()
13 | |     }
   | |_____^
   |
   = note: #[warn(where_clauses_object_safety)] on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #51443 <https://github.com/rust-lang/rust/issues/51443>
   = note: method `poll_unpin` references the `Self` type in where clauses
