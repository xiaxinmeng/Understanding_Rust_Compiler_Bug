
error[E0637]: `'_` cannot be used here
 --> src/lib.rs:4:8
  |
4 | where &'_ T: Trait<'_> {
  |        ^^ `'_` is a reserved lifetime name

error[E0637]: `'_` cannot be used here
 --> src/lib.rs:4:20
  |
4 | where &'_ T: Trait<'_> {
  |                    ^^ `'_` is a reserved lifetime name

error[E0275]: overflow evaluating the requirement `&'static &'static &'static &'static &'static &'static &'static ...: Trait<'static>`
 --> src/lib.rs:4:14
  |
4 | where &'_ T: Trait<'_> {
  |              ^^^^^^^^^
  |
  = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`playground`)
note: required for `&'static &'static &'static &'static &'static &'static &'static &'static &'static &'static &'static ...` to implement `Trait<'static>`
 --> src/lib.rs:3:9
  |
3 | impl<T> Trait<'_> for T
  |         ^^^^^^^^^     ^
  = note: the full type name has been written to '/playground/target/debug/deps/playground-3896d5d561595de5.long-type-16081939563578455512.txt'
  = note: 127 redundant requirements hidden
  = note: required for `&'static T` to implement `Trait<'static>`
