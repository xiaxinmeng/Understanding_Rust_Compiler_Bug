
error[E0277]: the trait bound `Vec<_>: Copy` is not satisfied
  --> src/lib.rs:10:17
   |
3  |     fn method<T>(self)
   |        ------ required by a bound in this
...
6  |             Vec<T>: Copy // this line doesn't show up in lacks_span error output
   |                     ---- required by this bound in `Foo::method`
...
10 | fn has_span() { Foo::method(Foo) }
   |                 ^^^^^^^^^^^ the trait `Copy` is not implemented for `Vec<_>`

error[E0277]: the trait bound `Vec<_>: Copy` is not satisfied
  --> src/lib.rs:12:23
   |
12 | fn lacks_span() { Foo.method() }
   |                       ^^^^^^ the trait `Copy` is not implemented for `Vec<_>`
