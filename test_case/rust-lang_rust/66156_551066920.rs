
error[E0658]: the `#[rustc_on_unimplemented]` attribute is an experimental feature
    --> src/libstd/process.rs:1606:1
     |
1606 | / #[rustc_on_unimplemented(
1607 | |   message="`main` has invalid return type `{Self}`",
1608 | |   label="`main` can only return types that implement `{Termination}`")]
     | |_______________________________________________________________________^
     |
     = note: for more information, see https://github.com/rust-lang/rust/issues/29628
     = help: add `#![feature(on_unimplemented)]` to the crate attributes to enable
