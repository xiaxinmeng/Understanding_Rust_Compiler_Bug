
[01:38:51] ---- [run-make] run-make/issue-26092 stdout ----
[01:38:51] 	
[01:38:51] error: make failed
[01:38:51] status: exit code: 2
[01:38:51] command: "make"
[01:38:51] stdout:
[01:38:51] ------------------------------------------
[01:38:51] make[1]: Entering directory '/checkout/src/test/run-make/issue-26092'
[01:38:51] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:323:21
[01:38:51] LD_LIBRARY_PATH="/checkout/obj/build/i686-unknown-linux-gnu/test/run-make/issue-26092.stage2-i686-unknown-linux-gnu:/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib:/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools/i686-unknown-linux-gnu/release/deps:/checkout/obj/build/i686-unknown-linux-gnu/stage0-sysroot/lib/rustlib/i686-unknown-linux-gnu/lib:" '/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/i686-unknown-linux-gnu/test/run-make/issue-26092.stage2-i686-unknown-linux-gnu -L /checkout/obj/build/i686-unknown-linux-gnu/test/run-make/issue-26092.stage2-i686-unknown-linux-gnu  -o "" blank.rs 2>&1 | \
[01:38:51] 		grep -i 'No such file or directory'
[01:38:51] Makefile:4: recipe for target 'all' failed
[01:38:51] make[1]: Leaving directory '/checkout/src/test/run-make/issue-26092'
[01:38:51] 
[01:38:51] ------------------------------------------
[01:38:51] stderr:
[01:38:51] ------------------------------------------
[01:38:51] make[1]: warning: jobserver unavailable: using -j1.  Add '+' to parent make rule.
[01:38:51] make[1]: *** [all] Error 1
[01:38:51] 
[01:38:51] ------------------------------------------
[01:38:51] 
[01:38:51] thread '[run-make] run-make/issue-26092' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2433:8
[01:38:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:38:51] 
[01:38:51] 
[01:38:51] failures:
[01:38:51]     [run-make] run-make/issue-26092
[01:38:51] 
[01:38:51] test result: [31mFAILED(B[m. 160 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
