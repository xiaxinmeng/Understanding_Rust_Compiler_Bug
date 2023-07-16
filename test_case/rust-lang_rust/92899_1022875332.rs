plain
...........................................................................ii....................... 300/3745
.................................................................................................... 400/3745
..............................i..................................................................... 500/3745
.................................................................................................... 600/3745
.................ii....F...F..........iiii.......................................................... 700/3745
.................................................................................................... 900/3745
.................................................................................................... 1000/3745
.................................................................................................... 1100/3745
.................................................................................................... 1200/3745
---
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `Some((4, 6))`,
 right: `Some((1, 2))`', src/iter/traits/iterator.rs:13:1


---- src/iter/traits/iterator.rs - iter::traits::iterator::Iterator::zip (line 534) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `Some((4, 6))`,
 right: `Some((1, 2))`', src/iter/traits/iterator.rs:13:1



failures:
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:21:52
