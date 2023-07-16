plain
---- src/iter/traits/iterator.rs - iter::traits::iterator::Iterator::cloned (line 3021) stdout ----
error[E0282]: type annotations needed
 --> src/iter/traits/iterator.rs:3025:18
  |
7 | assert_eq!(&[".".into()], &bad[..]);
  |              |   |
  |              |   |
  |              |   cannot infer type for type parameter `T` declared on the trait `Into`
  |              this method call resolves to `T`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:23:47
