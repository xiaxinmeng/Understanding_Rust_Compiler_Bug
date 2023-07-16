

error[E0277]: the trait bound `[u8; 64]: FooImpl<{N==0}>` is not satisfied
  --> src/main.rs:43:23
   |
43 |     let v: [u8; 64] = Foo::foo();
   |                       ^^^^^^^^ the trait `FooImpl<{N==0}>` is not implemented for `[u8; 64]`
   |
   = help: the following implementations were found:
             <[T; 0] as FooImpl<true>>
             <[T; _] as FooImpl<false>>
   = note: required because of the requirements on the impl of `Foo` for `[u8; 64]`
note: required by `Foo::foo`
  --> src/main.rs:4:5
   |
4  |     fn foo() -> Self;
   |     ^^^^^^^^^^^^^^^^^

