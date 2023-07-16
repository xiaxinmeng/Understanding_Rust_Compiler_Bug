
   Compiling foo v0.1.0 (/tmp/2022-01-10-16-11-09/foo)
error[E0277]: the trait bound `[Foo<'a>]: ToOwned` is not satisfied
   --> src/lib.rs:4:15
    |
4   |     children: Cow<'a, [Self]>,
    |               ^^^^^^^^^^^^^^^ the trait `ToOwned` is not implemented for `[Foo<'a>]`
    |
    = help: the following implementations were found:
              <[T] as ToOwned>
note: required by a bound in `Cow`
   --> /home/maarten/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/borrow.rs:183:8
    |
183 |     B: ToOwned,
    |        ^^^^^^^ required by this bound in `Cow`

error[E0277]: the trait bound `[Foo<'a>]: ToOwned` is not satisfied in `Foo<'a>`
   --> src/lib.rs:7:10
    |
7   | impl<'a> Clone for Foo<'a> {
    |          ^^^^^ within `Foo<'a>`, the trait `ToOwned` is not implemented for `[Foo<'a>]`
    |
    = help: the following implementations were found:
              <[T] as ToOwned>
note: required because it appears within the type `Foo<'a>`
   --> src/lib.rs:3:8
    |
3   | struct Foo<'a> {
    |        ^^^
note: required by a bound in `Clone`
   --> /home/maarten/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/clone.rs:108:18
    |
108 | pub trait Clone: Sized {
    |                  ^^^^^ required by this bound in `Clone`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `foo` due to 2 previous errors
