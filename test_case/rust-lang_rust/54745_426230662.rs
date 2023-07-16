plain
[00:04:58]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:59]    Compiling alloc_system v0.0.0 (/checkout/src/liballoc_system)
[00:04:59]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:05:03]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:05:08] error: call to unsafe function is unsafe and unsafe operations are not allowed in const fn
[00:05:08]      |
[00:05:08]      |
[00:05:08] 1044 |         mem::transmute(bytes)
[00:05:08]      |         ^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
[00:05:08]      |
[00:05:08]      = note: consult the function's documentation for information on how to avoid undefined behavior
[00:05:08] error: aborting due to previous error
[00:05:08] 
[00:05:08] error: Could not compile `std`.
[00:05:08] 
[00:05:08] 
[00:05:08] To learn more, run the command again with --verbose.
[00:05:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:05:08] expected success, got: exit code: 101
[00:05:08] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:05:08] travis_fold:end:stage0-std

[00:05:08] travis_time:end:stage0-std:start=1538477459977074430,finish=1538477497042180674,duration=37065106244

