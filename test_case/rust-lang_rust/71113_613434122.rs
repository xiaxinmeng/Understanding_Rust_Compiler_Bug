
error[E0411]: cannot find type `Self` in this scope
 --> <source>:6:5
  |
6 |     Self: ToOwned<Owned=Box<Self>>
  |     ^^^^ `Self` is only available in traits and impls

error[E0411]: cannot find type `Self` in this scope
 --> <source>:6:29
  |
6 |     Self: ToOwned<Owned=Box<Self>>
  |                             ^^^^ `Self` is only available in traits and impls

warning: unused import: `Borrow`
 --> <source>:1:19
  |
1 | use std::borrow::{Borrow, Cow};
  |                   ^^^^^^
  |
  = note: #[warn(unused_imports)] on by default

warning: unused imports: `DerefMut`, `Deref`
 --> <source>:2:16
  |
2 | use std::ops::{Deref, DerefMut};
  |                ^^^^^  ^^^^^^^^

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0411`.
