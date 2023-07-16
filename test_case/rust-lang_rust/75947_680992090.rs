plain
   Compiling addr2line v0.13.0
[RUSTC-TIMING] addr2line test:false 0.362
[RUSTC-TIMING] gimli test:false 4.050
[RUSTC-TIMING] object test:false 7.975
error[E0658]: `#[doc(spotlight)]` is experimental
    |
    |
488 | #[doc(spotlight)]
    |
    |
    = note: see issue #45040 <https://github.com/rust-lang/rust/issues/45040> for more information
    = help: add `#![feature(doc_spotlight)]` to the crate attributes to enable

error[E0658]: `#[doc(spotlight)]` is experimental
     |
     |
1241 | #[doc(spotlight)]
     |
     |
     = note: see issue #45040 <https://github.com/rust-lang/rust/issues/45040> for more information
     = help: add `#![feature(doc_spotlight)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
[RUSTC-TIMING] std test:false 1.720
---
== clock drift check ==
  local time: Wed Aug 26 16:37:25 UTC 2020
  network time: Wed, 26 Aug 2020 16:37:25 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (2759) (node)
Terminate orphan process: pid (2787) (python)
