plain
thread '<unnamed>' panicked at 'explicit panic', library/std/src/io/buffered/tests.rs:497:13
........................................................................................ 352/963
thread '<unnamed>' panicked at 'explicit panic', library/std/src/io/stdio/tests.rs:37:9
........................................................................................ 440/963
error: test failed, to rerun pass `-p std --lib`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/std-4f3b3a6236f6ed2e --quiet` (signal: 11, SIGSEGV: invalid memory reference)
..................................................................Build completed unsuccessfully in 0:01:31
