plain
---- src/option.rs - option::Option<T>::reduce (line 1061) stdout ----
error[E0599]: no function or associated item named `add` found for type `i32` in the current scope
  --> src/option.rs:1069:29
   |
11 | assert_eq!(x.reduce(y, i32::add), Some(12));
   |                             ^^^ function or associated item not found in `i32`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
5  | use std::ops::Add;
5  | use std::ops::Add;
   |

error[E0599]: no function or associated item named `add` found for type `i32` in the current scope
  --> src/option.rs:1070:32
   |
12 | assert_eq!(None.reduce(y, i32::add), Some(7));
   |                                ^^^ function or associated item not found in `i32`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
5  | use std::ops::Add;
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:19:37
