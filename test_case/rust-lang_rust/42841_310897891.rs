
[01:01:22] failures:
[01:01:22]
[01:01:22] ---- [run-make] run-make/sanitizer-leak stdout ----
[01:01:22]
[01:01:22] error: make failed
[01:01:22] status: exit code: 2
[01:01:22] command: "make"
[01:01:22] stdout:
[01:01:22] ------------------------------------------
[01:01:22] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-leak.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu
[01:01:22] /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-leak.stage2-x86_64-unknown-linux-gnu/leak 2>&1 | grep -q 'detected memory leaks'
[01:01:22] Makefile:6: recipe for target 'all' failed
[01:01:22]
[01:01:22] ------------------------------------------
[01:01:22] stderr:
[01:01:22] ------------------------------------------
[01:01:22] make: *** [all] Error 1
[01:01:22]
[01:01:22] ------------------------------------------
[01:01:22]
[01:01:22] thread '[run-make] run-make/sanitizer-leak' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2480
[01:01:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:22]
[01:01:22]
[01:01:22] failures:
[01:01:22]     [run-make] run-make/sanitizer-leak
[01:01:22]
[01:01:22] test result: FAILED. 148 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
