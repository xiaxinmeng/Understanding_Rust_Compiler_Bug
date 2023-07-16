
error[E0277]: the trait bound `&Foo: From<u64>` is not satisfied
  --> f40.rs:10:15
   |
10 |     foo(42u64.into());
   |               ^^^^ the trait `From<u64>` is not implemented for `&Foo`
   |
   = help: the trait `From<u64>` is implemented for `Foo`
   = note: required because of the requirements on the impl of `Into<&Foo>` for `u64`
