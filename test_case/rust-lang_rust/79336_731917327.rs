
error[E0658]: auto traits are experimental and possibly buggy
  --> library/core/src/marker.rs:38:1
   |
38 | / pub unsafe auto trait Send {
39 | |     // empty.
40 | | }
   | |_^
   |
   = note: see issue #13231 <https://github.com/rust-lang/rust/issues/13231> for more information
   = help: add `#![feature(optin_builtin_traits)]` to the crate attributes to enable

error[E0658]: auto traits are experimental and possibly buggy
   --> library/core/src/marker.rs:469:1
    |
469 | / pub unsafe auto trait Sync {
470 | |     // FIXME(estebank): once support to add notes in `rustc_on_unimplemented`
471 | |     // lands in beta, and it has been extended to check whether a closure is
472 | |     // anywhere in the requirement chain, extend it as such (#48534):
...   |
480 | |     // Empty
481 | | }
    | |_^
    |
    = note: see issue #13231 <https://github.com/rust-lang/rust/issues/13231> for more information
    = help: add `#![feature(optin_builtin_traits)]` to the crate attributes to enable

error[E0658]: auto traits are experimental and possibly buggy
   --> library/core/src/marker.rs:716:1
    |
716 | pub(crate) unsafe auto trait Freeze {}
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: see issue #13231 <https://github.com/rust-lang/rust/issues/13231> for more information
    = help: add `#![feature(optin_builtin_traits)]` to the crate attributes to enable

error[E0658]: auto traits are experimental and possibly buggy
   --> library/core/src/marker.rs:772:1
    |
772 | pub auto trait Unpin {}
    | ^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: see issue #13231 <https://github.com/rust-lang/rust/issues/13231> for more information
    = help: add `#![feature(optin_builtin_traits)]` to the crate attributes to enable

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
error: could not compile `core`
