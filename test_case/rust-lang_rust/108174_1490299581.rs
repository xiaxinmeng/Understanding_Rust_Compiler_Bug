
error[E0277]: the trait bound `Foo: Clone` is not satisfied
   --> ./t.rs:5:15
    |
5   | impl Copy for Foo {}
    |               ^^^ the trait `Clone` is not implemented for `Foo`
    |
note: required by a bound in `Copy`
   --> /home/waffle/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/marker.rs:392:17
    |
392 | pub trait Copy: Clone {
    |                 ^^^^^ required by this bound in `Copy`
help: consider annotating `Foo` with `#[derive(Clone)]`
    |
1   | #[derive(Clone)]
    |
