
[00:56:50] ---- [run-make] run-make/sanitizer-staticlib-link stdout ----
[00:56:50]
[00:56:50] error: make failed
[00:56:50] status: exit code: 2
[00:56:50] command: "make"
[00:56:50] stdout:
[00:56:50] ------------------------------------------
[00:56:50] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-staticlib-link.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-staticlib-link.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-staticlib-link.stage2-x86_64-unknown-linux-gnu  -g -Z sanitizer=address --crate-type staticlib --target x86_64-unknown-linux-gnu library.rs
[00:56:50] ------------------------------------------
[00:56:50] stderr:
[00:56:50] ------------------------------------------
[00:56:50] warning: unused variable: `y`
[00:56:50]   --> library.rs:14:9
[00:56:50]    |
[00:56:50] 14 |     let y = unsafe { *xs.as_ptr().offset(4) };
[00:56:50]    |         ^
[00:56:50]    |
[00:56:50]    = note: #[warn(unused_variables)] on by default
[00:56:50]
[00:56:50] note: link against the following native artifacts when linking against this static library
[00:56:50]
[00:56:50] note: the order and any duplication can be significant on some platforms, and so may need to be preserved
[00:56:50]
[00:56:50] note: library: dl
[00:56:50]
[00:56:50] note: library: rt
[00:56:50]
[00:56:50] note: library: pthread
[00:56:50]
[00:56:50] note: library: gcc_s
[00:56:50]
[00:56:50] note: library: c
[00:56:50]
[00:56:50] note: library: m
[00:56:50]
[00:56:50]
[00:56:50] thread '[run-make] run-make/sanitizer-staticlib-link' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2473
[00:56:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:50]
[00:56:50]
[00:56:50] failures:
[00:56:50]     [run-make] run-make/sanitizer-staticlib-link
