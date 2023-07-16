
[00:50:13] failures:
[00:50:13] 
[00:50:13] ---- [run-make] run-make/target-specs stdout ----
[00:50:13] 	
[00:50:13] error: make failed
[00:50:13] status: exit code: 2
[00:50:13] command: "make"
[00:50:13] stdout:
[00:50:13] ------------------------------------------
[00:50:13] make[1]: Entering directory '/checkout/src/test/run-make/target-specs'
[00:50:13] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/target-specs.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/target-specs.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/target-specs.stage2-x86_64-unknown-linux-gnu  foo.rs --target=my-awesome-platform.json --crate-type=lib --emit=asm
[00:50:13] Makefile:3: recipe for target 'all' failed
[00:50:13] make[1]: Leaving directory '/checkout/src/test/run-make/target-specs'
[00:50:13] 
[00:50:13] ------------------------------------------
[00:50:13] stderr:
[00:50:13] ------------------------------------------
[00:50:13] make[1]: warning: jobserver unavailable: using -j1.  Add '+' to parent make rule.
[00:50:13] error: Error loading target specification: Field target-c-int-width in target specification is required
[00:50:13]   |
[00:50:13]   = help: Use `--print target-list` for a list of built-in targets
[00:50:13] 
[00:50:13] make[1]: *** [all] Error 101
[00:50:13] 
[00:50:13] ------------------------------------------
[00:50:13] 
[00:50:13] thread '[run-make] run-make/target-specs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2433:8
[00:50:13] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:13] 
[00:50:13] 
[00:50:13] failures:
[00:50:13]     [run-make] run-make/target-specs
