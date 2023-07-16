
error[E0277]: the trait bound `B: AsRef<u32>` is not satisfied
  --> src/main.rs:14:5
   |
14 |     fn foo<T: AsRef<Self::Foo>>(&self, t: T) {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `AsRef<u32>` is not implemented for `B`
   |
   = note: required because of the requirements on the impl of `Example` for `(A, B)`
help: consider further restricting this bound
   |
10 |     B: AsRef<A::Item> + AsRef<u32>,
   |                       ^^^^^^^^^^^^

error[E0277]: the trait bound `B: AsRef<u32>` is not satisfied
  --> src/main.rs:14:8
   |
14 |     fn foo<T: AsRef<Self::Foo>>(&self, t: T) {}
   |        ^^^ the trait `AsRef<u32>` is not implemented for `B`
   |
   = note: required because of the requirements on the impl of `Example` for `(A, B)`
help: consider further restricting this bound
   |
10 |     B: AsRef<A::Item> + AsRef<u32>,
   |                       ^^^^^^^^^^^^

error[E0277]: the trait bound `T: AsRef<()>` is not satisfied
   --> src/main.rs:14:15
    |
14  |       fn foo<T: AsRef<Self::Foo>>(&self, t: T) {}
    |                 ^^^^^^^^^^^^^^^^ the trait `AsRef<()>` is not implemented for `T`
    |
help: consider further restricting this bound
    |
14  |     fn foo<T: AsRef<Self::Foo> + AsRef<()>>(&self, t: T) {}
    |                                ^^^^^^^^^^^
