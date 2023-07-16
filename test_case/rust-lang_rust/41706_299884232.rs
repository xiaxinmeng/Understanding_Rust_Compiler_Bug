
[00:50:31] failures:
[00:50:31] 
[00:50:31] ---- [run-make] run-make/alloc-extern-crates stdout ----
[00:50:31] 	
[00:50:31] error: make failed
[00:50:31] status: exit code: 2
[00:50:31] command: "make"
[00:50:31] stdout:
[00:50:31] ------------------------------------------
[00:50:31] make[1]: Entering directory '/checkout/src/test/run-make/alloc-extern-crates'
[00:50:31] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/alloc-extern-crates.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/alloc-extern-crates.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/alloc-extern-crates.stage2-x86_64-unknown-linux-gnu  fakealloc.rs
[00:50:31] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/alloc-extern-crates.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/alloc-extern-crates.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/alloc-extern-crates.stage2-x86_64-unknown-linux-gnu  ../../../liballoc/lib.rs --cfg feature=\"external_crate\" --extern external=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/alloc-extern-crates.stage2-x86_64-unknown-linux-gnu/libfakealloc.rlib
[00:50:31] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:315
[00:50:31] Makefile:4: recipe for target 'all' failed
[00:50:31] make[1]: Leaving directory '/checkout/src/test/run-make/alloc-extern-crates'
[00:50:31] 
[00:50:31] ------------------------------------------
[00:50:31] stderr:
[00:50:31] ------------------------------------------
[00:50:31] make[1]: warning: jobserver unavailable: using -j1.  Add '+' to parent make rule.
[00:50:31] error: language item required, but not found: `panic_fmt`
[00:50:31] 
[00:50:31] error: language item required, but not found: `eh_personality`
[00:50:31] 
[00:50:31] error: aborting due to 2 previous errors
[00:50:31] 
[00:50:31] make[1]: *** [all] Error 101
[00:50:31] 
[00:50:31] ------------------------------------------
[00:50:31] 
[00:50:31] thread '[run-make] run-make/alloc-extern-crates' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2467
[00:50:31] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:31] 
[00:50:31] 
[00:50:31] failures:
[00:50:31]     [run-make] run-make/alloc-extern-crates
[00:50:31] 
[00:50:31] test result: FAILED. 149 passed; 1 failed; 0 ignored; 0 measured
