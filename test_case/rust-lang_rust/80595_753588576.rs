console
error[E0210]: type parameter `T` must be covered by another type when it appears before the first local type (`Ident`)
 --> src/main.rs:9:6
  |
9 | impl<T> PartialEq<Ident> for T where T: ?Sized + AsRef<str> {
  |      ^ type parameter `T` must be covered by another type when it appears before the first local type (`Ident`)
  |
  = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local, and no uncovered type parameters appear before that first local type
  = note: in this case, 'before' refers to the following order: `impl<..> ForeignTrait<T1, ..., Tn> for T0`, where `T0` is the first and `Tn` is the last
