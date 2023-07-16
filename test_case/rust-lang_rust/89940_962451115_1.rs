rust
error[E0277]: the trait bound `[Foo]: ToOwned` is not satisfied in `Foo`
 --> src/lib.rs:5:23
  |
5 |     foo: Cow<'static, [Foo]>,
  |                       ^^^^^ within `Foo`, the trait `ToOwned` is not implemented for `[Foo]`
  |
  = help: the following implementations were found:
            <[T] as ToOwned>
note: required because it appears within the type `Foo`
 --> src/lib.rs:4:8
  |
4 | struct Foo {
  |        ^^^
  = note: slice and array elements must have `Sized` type

error[E0277]: the trait bound `[Foo]: ToOwned` is not satisfied in `Foo`
   --> src/lib.rs:3:10
    |
3   | #[derive(Clone)]
    |          ^^^^^ within `Foo`, the trait `ToOwned` is not implemented for `[Foo]`
    |
    = help: the following implementations were found:
              <[T] as ToOwned>
note: required because it appears within the type `Foo`
   --> src/lib.rs:4:8
    |
4   | struct Foo {
    |        ^^^
note: required by a bound in `Clone`
    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
