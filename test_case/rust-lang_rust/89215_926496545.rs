plain
............................................................................................F....... 500/598
..................................................................................................
failures:

---- src/sync.rs - sync::Arc<[T]>::try_unwrap_as_vec (line 703) stdout ----
error[E0658]: use of unstable library feature 'vec_from_reference_counted': recently added
  |
  |
7 | assert!(matches!(Arc::try_unwrap_as_vec(x), Ok(vec) if vec.len() == 4));
  |
  = help: add `#![feature(vec_from_reference_counted)]` to the crate attributes to enable

error[E0308]: mismatched types
error[E0308]: mismatched types
 --> src/sync.rs:707:41
  |
7 | assert!(matches!(Arc::try_unwrap_as_vec(x), Ok(vec) if vec.len() == 4));
  |                                         ^ expected slice, found `&[{integer}; 4]`
  |
  = note: expected struct `Arc<[_]>`
             found struct `Arc<&[{integer}; 4]>`

error[E0658]: use of unstable library feature 'vec_from_reference_counted': recently added
   |
   |
11 | assert_eq!(Arc::try_unwrap_as_vec(x).unwrap_err()[3], 4);
   |
   = help: add `#![feature(vec_from_reference_counted)]` to the crate attributes to enable

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> src/sync.rs:711:35
   |
11 | assert_eq!(Arc::try_unwrap_as_vec(x).unwrap_err()[3], 4);
   |                                   ^ expected slice, found `&[{integer}; 4]`
   |
   = note: expected struct `Arc<[_]>`
              found struct `Arc<&[{integer}; 4]>`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0308, E0658.
For more information about an error, try `rustc --explain E0308`.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:17:12
