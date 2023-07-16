
rustc 1.15.1 (021bd294c 2017-02-08)
error[E0277]: the trait bound `Foo + 'static: std::marker::Sized` is not satisfied
 --> <anon>:3:8
  |
3 | fn foo(arg: Foo) {} //Foo is a trait
  |        ^^^ the trait `std::marker::Sized` is not implemented for `Foo + 'static`
  |
  = note: `Foo + 'static` does not have a constant size known at compile-time
  = note: all local variables must have a statically known size

error[E0277]: the trait bound `[i32]: std::marker::Sized` is not satisfied
 --> <anon>:4:9
  |
4 | fn bar (arg: [i32]) {}
  |         ^^^ the trait `std::marker::Sized` is not implemented for `[i32]`
  |
  = note: `[i32]` does not have a constant size known at compile-time
  = note: all local variables must have a statically known size

error[E0277]: the trait bound `str: std::marker::Sized` is not satisfied
 --> <anon>:5:9
  |
5 | fn quz (arg: str) {}
  |         ^^^ the trait `std::marker::Sized` is not implemented for `str`
  |
  = note: `str` does not have a constant size known at compile-time
  = note: all local variables must have a statically known size

