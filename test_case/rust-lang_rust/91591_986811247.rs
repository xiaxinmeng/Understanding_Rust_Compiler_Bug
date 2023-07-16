plain
.................................................................................................... 400/3681
..................i................................................................................. 500/3681
.................................................................................................... 600/3681
...ii.................iiii.......................................................................... 700/3681
...........................F................F....................................................... 800/3681
.................................................................................................... 1000/3681
.................................................................................................... 1100/3681
.................................................................................................... 1200/3681
.................................................................................................... 1300/3681
---
i.....................i.....................i.....................i.....................i........... 3600/3681
.................................................................................
failures:

---- src/mem/mod.rs - mem::replace_succ (line 853) stdout ----
error[E0658]: use of unstable library feature 'replace_succ'
  |
  |
7 | assert_eq!(replace_succ(&mut i, |i| i + 1), 0);
  |
  |
  = help: add `#![feature(replace_succ)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'replace_succ'
 --> src/mem/mod.rs:854:5
  |
4 | use std::mem::replace_succ;
4 | use std::mem::replace_succ;
  |     ^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: add `#![feature(replace_succ)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/mem/mod.rs - mem::try_replace_succ (line 871) stdout ----
error[E0658]: use of unstable library feature 'replace_succ'
  |
  |
7 | assert_eq!(try_replace_succ(&mut i, |i| i.checked_add(1)), Ok(0));
  |
  |
  = help: add `#![feature(replace_succ)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'replace_succ'
  --> src/mem/mod.rs:879:12
   |
   |
11 | assert_eq!(try_replace_succ(&mut i, |i| i.checked_add(1).ok_or(())), Err(()));
   |
   |
   = help: add `#![feature(replace_succ)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'replace_succ'
 --> src/mem/mod.rs:872:5
  |
4 | use std::mem::try_replace_succ;
4 | use std::mem::try_replace_succ;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: add `#![feature(replace_succ)]` to the crate attributes to enable

error[E0599]: no method named `checked_add` found for reference `&{integer}` in the current scope
  |
  |
7 | assert_eq!(try_replace_succ(&mut i, |i| i.checked_add(1)), Ok(0));
  |                                           ^^^^^^^^^^^ method not found in `&{integer}`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0599, E0658.
For more information about an error, try `rustc --explain E0599`.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:21:35
