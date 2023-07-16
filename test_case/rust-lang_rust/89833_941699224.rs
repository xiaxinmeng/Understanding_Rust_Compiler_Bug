plain
.................................................................................................... 500/599
...................................................................................................
failures:

---- src/rc.rs - rc (line 40) stdout ----
error: unused return value of `Rc::<T>::downgrade` that must be used
  |
  |
7 | Rc::downgrade(&my_rc);
  |
note: the lint level is defined here
 --> src/rc.rs:39:9
  |
  |
2 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(unused_must_use)]` implied by `#[deny(warnings)]`
  = note: this returns a new `Weak` pointer, without modifying the original `Rc`
error: aborting due to previous error

Couldn't compile the test.

---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:14:29
