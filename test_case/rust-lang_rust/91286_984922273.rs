plain
test src/unit.rs - unit::() (line 8) ... ok

failures:

---- src/iter/traits/iterator.rs - iter::traits::iterator::Iterator::try_find (line 2444) stdout ----
error: the feature `nonzero_is_power_of_two` has been stable since 1.59.0 and no longer requires an attribute to enable
  |
  |
4 | #![feature(nonzero_is_power_of_two)]
  |
note: the lint level is defined here
 --> src/iter/traits/iterator.rs:2442:9
  |
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--"


Build completed unsuccessfully in 0:22:07
