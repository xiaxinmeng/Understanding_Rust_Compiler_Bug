
error: reached the type-length limit while instantiating `<std::vec::IntoIter<i32> as std:...<std::vec::InPlaceDrop<i32>, !>>`
  --> workspace/rust/src/libcore/iter/traits/iterator.rs:1870:5
   |
LL | /     fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
LL | |     where
LL | |         Self: Sized,
LL | |         F: FnMut(B, Self::Item) -> R,
...  |
LL | |         Try::from_ok(accum)
LL | |     }
   | |_____^
   |
   = note: consider adding a `#![type_length_limit="1327046"]` attribute to your crate

error: aborting due to previous error
