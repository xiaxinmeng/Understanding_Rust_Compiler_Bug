
rustc 1.17.0 (56124baa9 2017-04-24)
warning[E0122]: trait bounds are not (yet) enforced in type definitions
 --> <anon>:6:1
  |
6 | type Thing<F: Foo> = (F::Bar, F::Biff);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: the trait bound `u32: Foo` is not satisfied
 --> <anon>:9:12
  |
9 |     let v: Thing<u32> = unimplemented!();
  |            ^^^^^^^^^^ the trait `Foo` is not implemented for `u32`

error: aborting due to previous error

