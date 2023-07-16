plain
.............................................................iii.................................... 3000/3747
.................................................................................................... 3100/3747
.................................................................................................... 3200/3747
.................................................................................................... 3300/3747
........F..F..................................................................i..................... 3400/3747
...........i.....................i................................i.....................i........... 3600/3747
..........i.....................i.....................i............................................. 3700/3747
...............................................
failures:
failures:

---- src/slice/mod.rs - slice::[f32]::sort_floats (line 4001) stdout ----
Test executable failed (exit code 101).
stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `[-inf, -1.0, -5e-8, -0.0, 0.0, 2.6, 8.29, inf]`,
 right: `[-inf, -5e-8, -1.0, -0.0, 0.0, 2.6, 8.29, inf]`', src/slice/mod.rs:9:1



---- src/slice/mod.rs - slice::[f64]::sort_floats (line 4032) stdout ----
Test executable failed (exit code 101).
stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `[-inf, -1.0, -5e-8, -0.0, 0.0, 2.6, 8.29, inf]`,
 right: `[-inf, -5e-8, -1.0, -0.0, 0.0, 2.6, 8.29, inf]`', src/slice/mod.rs:9:1



failures:
failures:
    src/slice/mod.rs - slice::[f32]::sort_floats (line 4001)
    src/slice/mod.rs - slice::[f64]::sort_floats (line 4032)
test result: FAILED. 3713 passed; 2 failed; 32 ignored; 0 measured; 0 filtered out; finished in 57.09s

error: test failed, to rerun pass '--doc'



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:18:57
