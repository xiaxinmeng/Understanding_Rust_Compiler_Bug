
error[E0277]: the trait bound `T: Clone` is not satisfied
   --> f26.rs:15:5
    |
13  | #[derive(Clone)]
    |          ----- in this derive macro expansion
14  | struct Baz<T> {
15  |     x: Foo<T>
    |     ^^^^^^^^^ expected an implementor of trait `Clone`
    |
note: required because of the requirements on the impl of `Clone` for `Foo<T>`
   --> f26.rs:9:14
    |
9   | impl<T: Bar> Clone for Foo<T> {
    |              ^^^^^     ^^^^^^
note: required by `clone`
   --> /Users/ekuber/workspace/rust/library/core/src/clone.rs:121:5
    |
121 |     fn clone(&self) -> Self;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^
    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
