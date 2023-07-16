
$ rustc fulfill.rs
error[E0277]: the trait bound `Struct1<T>: Trait2` is not satisfied
  --> fulfill.rs:35:14
   |
35 |     pub ins: <<Vec<T> as Trait3>::Type3 as Trait2>::Type2,
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Trait2` is not implemented for `Struct1<T>`
   |
   = note: required because of the requirements on the impl of `Trait2` for `Vec<Struct1<T>>`
$ rustdoc fulfill.rs
$
