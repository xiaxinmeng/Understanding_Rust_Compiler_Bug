plain
.................................................................................................... 3200/3607
........................................i...............................................i........... 3300/3607
.....i......................i....................i.....................i.....................i...... 3400/3607
..........................i.....................i.....................i.....................i....... 3500/3607
..............i......................................................................F........FF.... 3600/3607
.....F.

---- src/time.rs - time::Duration::div_f32 (line 949) stdout ----
Test executable failed (exit code 101).


stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `859.872579ms`,
 right: `859.872576ms`', src/time.rs:9:1


---- src/time.rs - time::Duration::from_secs_f32 (line 798) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `2.700000047s`,
 right: `2.7s`', src/time.rs:7:1


---- src/time.rs - time::Duration::mul_f32 (line 902) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `847800s`,
 right: `847799.969120256s`', src/time.rs:10:1


---- src/time.rs - time::Duration::try_from_secs_f32 (line 822) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `Ok(2.700000047s)`,
 right: `Ok(2.7s)`', src/time.rs:9:1



failures:
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:20:29
