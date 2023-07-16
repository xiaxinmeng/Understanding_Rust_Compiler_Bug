
[00:57:26] ---- [run-make] run-make/sanitizer-leak stdout ----
[00:57:26] 	
[00:57:26] error: make failed
[00:57:26] status: exit code: 2
[00:57:26] command: "make"
[00:57:26] stdout:
[00:57:26] ------------------------------------------
[00:57:26] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-leak.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-leak.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-leak.stage2-x86_64-unknown-linux-gnu  -C opt-level=1 -g -Z sanitizer=leak -Z print-link-args leak.rs | grep -q librustc_lsan
[00:57:26] /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-leak.stage2-x86_64-unknown-linux-gnu/leak 2>&1 | grep -q 'detected memory leaks'
[00:57:26] Makefile:6: recipe for target 'all' failed
[00:57:26] 
[00:57:26] ------------------------------------------
[00:57:26] stderr:
[00:57:26] ------------------------------------------
[00:57:26] make: *** [all] Error 1
[00:57:26] 
[00:57:26] ------------------------------------------
[00:57:26] 
[00:57:26] thread '[run-make] run-make/sanitizer-leak' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2480
[00:57:26] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:26] 
[00:57:26] 
[00:57:26] failures:
[00:57:26]     [run-make] run-make/sanitizer-leak
[00:57:26] 
[00:57:26] test result: [31mFAILED(B[m. 148 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
