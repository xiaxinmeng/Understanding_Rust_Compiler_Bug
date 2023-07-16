
error[E0277]: `&T` needs to be a pointer-sized type
 --> /home/gh-compiler-errors/test.rs:9:13
  |
9 |     let _ = t as dyn* Debug;
  |             ^ `&T` needs to be a pointer-sized type
  |
  = help: the trait `PointerSized` is not implemented for `&T`
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
  |
8 | fn polymorphic<T: Debug + ?Sized + Thin>(t: &T) where &T: PointerSized {
  |                                                 ++++++++++++++++++++++
