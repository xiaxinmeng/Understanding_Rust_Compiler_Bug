
error[E0277]: the trait bound `Vec<_>: Copy` is not satisfied
  --> f16.rs:12:23
   |
12 | fn lacks_span() { Foo.method() }
   |                       ^^^^^^ the trait `Copy` is not implemented for `Vec<_>`
   |
note: required by `Foo::method`
  --> f16.rs:3:5
   |
3  | /     fn method<T>(self)
4  | |         // bound that never holds, `Vec<T>` instead of `T` to bypass `_: Trait`.
5  | |         where
6  | |             Vec<T>: Copy // this line doesn't show up in lacks_span error output
   | |________________________^
