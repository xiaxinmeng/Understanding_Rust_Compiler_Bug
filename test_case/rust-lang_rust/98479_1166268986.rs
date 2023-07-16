plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0658]: use of unstable library feature 'atomic_bool_fetch_not'
  --> library/core/tests/atomic.rs:36:18
   |
36 |     assert_eq!(a.fetch_not(SeqCst), false);
   |
   = note: see issue #98485 <https://github.com/rust-lang/rust/issues/98485> for more information
   = note: see issue #98485 <https://github.com/rust-lang/rust/issues/98485> for more information
   = help: add `#![feature(atomic_bool_fetch_not)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'atomic_bool_fetch_not'
  --> library/core/tests/atomic.rs:38:18
   |
   |
38 |     assert_eq!(a.fetch_not(SeqCst), true);
   |
   = note: see issue #98485 <https://github.com/rust-lang/rust/issues/98485> for more information
   = note: see issue #98485 <https://github.com/rust-lang/rust/issues/98485> for more information
   = help: add `#![feature(atomic_bool_fetch_not)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'atomic_bool_fetch_not'
  --> library/core/tests/atomic.rs:40:18
   |
   |
40 |     assert_eq!(a.fetch_not(SeqCst), false);
   |
   = note: see issue #98485 <https://github.com/rust-lang/rust/issues/98485> for more information
   = note: see issue #98485 <https://github.com/rust-lang/rust/issues/98485> for more information
   = help: add `#![feature(atomic_bool_fetch_not)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'atomic_bool_fetch_not'
  --> library/core/tests/atomic.rs:42:18
   |
   |
42 |     assert_eq!(a.fetch_not(SeqCst), true);
   |
   = note: see issue #98485 <https://github.com/rust-lang/rust/issues/98485> for more information
   = note: see issue #98485 <https://github.com/rust-lang/rust/issues/98485> for more information
   = help: add `#![feature(atomic_bool_fetch_not)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'atomic_bool_fetch_not'
   --> library/core/tests/atomic.rs:173:16
    |
    |
173 |         ATOMIC.fetch_not(SeqCst);
    |
    = note: see issue #98485 <https://github.com/rust-lang/rust/issues/98485> for more information
    = note: see issue #98485 <https://github.com/rust-lang/rust/issues/98485> for more information
    = help: add `#![feature(atomic_bool_fetch_not)]` to the crate attributes to enable
For more information about this error, try `rustc --explain E0658`.
error: could not compile `core` due to 5 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:02:28
