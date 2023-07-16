
[00:45:34] ---- [run-make] run-make/alloc-extern-crates stdout ----
[00:45:34] 	
[00:45:34] error: make failed
[00:45:34] status: exit code: 2
[00:45:34] command: "make"
[00:45:34] stdout:
[00:45:34] ------------------------------------------
[00:45:34] make[1]: Entering directory '/checkout/src/test/run-make/alloc-extern-crates'
[00:45:34] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/alloc-extern-crates.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/alloc-extern-crates.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/alloc-extern-crates.stage2-x86_64-unknown-linux-gnu  fakealloc.rs
[00:45:34] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/alloc-extern-crates.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/alloc-extern-crates.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/alloc-extern-crates.stage2-x86_64-unknown-linux-gnu  ../../../liballoc/lib.rs --cfg feature=\"external_crate\" --extern external=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/alloc-extern-crates.stage2-x86_64-unknown-linux-gnu/libfakealloc.rlib
[00:45:34] Makefile:4: recipe for target 'all' failed
[00:45:34] make[1]: Leaving directory '/checkout/src/test/run-make/alloc-extern-crates'
[00:45:34] 
[00:45:34] ------------------------------------------
[00:45:34] stderr:
[00:45:34] ------------------------------------------
[00:45:34] make[1]: warning: jobserver unavailable: using -j1.  Add '+' to parent make rule.
[00:45:34] error: language item required, but not found: `panic_fmt`
[00:45:34] 
[00:45:34] error: language item required, but not found: `eh_personality`
[00:45:34] 
[00:45:34] error: aborting due to previous error(s)
[00:45:34] 
[00:45:34] make[1]: *** [all] Error 101
[00:45:34] 
[00:45:34] ------------------------------------------
[00:45:34] 
[00:45:34] thread '[run-make] run-make/alloc-extern-crates' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2480
[00:45:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:34] 
[00:45:34] 
[00:45:34] failures:
[00:45:34]     [run-make] run-make/alloc-extern-crates
