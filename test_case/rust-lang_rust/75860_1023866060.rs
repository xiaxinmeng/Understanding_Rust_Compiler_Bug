
error[E0275]: overflow evaluating the requirement `Option<_>: Sized`
   --> issue-75860.rs:9:14
    |
9   |    let left: fn(Option<_>) -> _ = move |o_a| match o_a {
    |              ^^^^^^^^^^^^^^^^^^
    |
    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_75860`)
note: required by a bound in `Option`
   --> /Users/pnkfelix/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:514:17
    |
514 | pub enum Option<T> {
    |                 ^ required by this bound in `Option`

error: aborting due to previous error
