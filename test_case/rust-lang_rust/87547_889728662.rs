plain
.......i.....................i.....................i.....................i.....................i.... 3000/3087
.......................................................................................
failures:

---- src/ptr/non_null.rs - ptr::non_null::NonNull<T>::as_ptr (line 246) stdout ----
error[E0277]: can't compare `u32` with `i32`
   |
   |
10 | assert_eq!(x_value, 0);
   | ^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `u32 == i32`
   |
   = help: the trait `PartialEq<i32>` is not implemented for `u32`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: cannot add-assign `i32` to `u32`
   |
   |
12 | unsafe { *ptr.as_ptr() += 2; }
   |                        ^^ no implementation for `u32 += i32`
   |
   = help: the trait `AddAssign<i32>` is not implemented for `u32`

error[E0277]: can't compare `u32` with `i32`
   |
   |
14 | assert_eq!(x_value, 2);
   | ^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `u32 == i32`
   |
   = help: the trait `PartialEq<i32>` is not implemented for `u32`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:19:35
