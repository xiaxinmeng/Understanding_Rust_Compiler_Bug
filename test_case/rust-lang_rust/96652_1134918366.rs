plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0658]: `crate` visibility modifier is experimental
    --> src/librustdoc/clean/types.rs:1413:5
     |
1413 |     crate fn is_empty(&self) -> bool {
     |
     = note: see issue #53120 <https://github.com/rust-lang/rust/issues/53120> for more information
     = note: see issue #53120 <https://github.com/rust-lang/rust/issues/53120> for more information
     = help: add `#![feature(crate_visibility_modifier)]` to the crate attributes to enable
For more information about this error, try `rustc --explain E0658`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:46
