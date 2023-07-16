
[01:12:34] failures:
[01:12:34] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:331:21
[01:12:34] 
[01:12:34] ---- [run-make] run-make/sanitizer-memory stdout ----
[01:12:34]      
[01:12:34] error: make failed
[01:12:34] status: exit code: 2
[01:12:34] command: "make"
[01:12:34] stdout:
[01:12:34] ------------------------------------------
[01:12:34] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-memory.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-memory.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-memory.stage2-x86_64-unknown-linux-gnu  -g -Z sanitizer=memory -Z print-link-args uninit.rs | grep -q librustc_msan
[01:12:34] /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-memory.stage2-x86_64-unknown-linux-gnu/uninit 2>&1 | grep -q use-of-uninitialized-value
[01:12:34] Makefile:6: recipe for target 'all' failed
[01:12:34] 
[01:12:34] ------------------------------------------
[01:12:34] stderr:
[01:12:34] ------------------------------------------
[01:12:34] warning: unused variable: `y`
[01:12:34]   --> uninit.rs:15:9
[01:12:34]    |
[01:12:34] 15 |     let y = xs[0] + xs[1];
[01:12:34]    |         ^
[01:12:34]    |
[01:12:34]    = note: #[warn(unused_variables)] on by default
[01:12:34]    = note: to avoid this warning, consider using `_y` instead
[01:12:34] 
[01:12:34] make: *** [all] Error 1
[01:12:34] 
[01:12:34] ------------------------------------------
[01:12:34] 
[01:12:34] thread '[run-make] run-make/sanitizer-memory' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2505:8
[01:12:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:12:34] 
[01:12:34] 
[01:12:34] failures:
[01:12:34]     [run-make] run-make/sanitizer-memory
[01:12:34] 
[01:12:34] test result: FAILED. 163 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
