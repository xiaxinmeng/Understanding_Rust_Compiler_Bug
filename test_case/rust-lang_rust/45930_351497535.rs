
[01:02:57] ---- [run-make] run-make/pretty-print-path-suffix stdout ----
[01:02:57] 	
[01:02:57] error: make failed
[01:02:57] status: exit code: 2
[01:02:57] command: "make"
[01:02:57] stdout:
[01:02:57] ------------------------------------------
[01:02:57] make[1]: Entering directory '/checkout/src/test/run-make/pretty-print-path-suffix'
[01:02:57] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/pretty-print-path-suffix.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/pretty-print-path-suffix.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/pretty-print-path-suffix.stage2-x86_64-unknown-linux-gnu  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/pretty-print-path-suffix.stage2-x86_64-unknown-linux-gnu/foo.out -Z unstable-options --unpretty hir=foo input.rs
[01:02:57] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/pretty-print-path-suffix.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/pretty-print-path-suffix.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/pretty-print-path-suffix.stage2-x86_64-unknown-linux-gnu  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/pretty-print-path-suffix.stage2-x86_64-unknown-linux-gnu/nest_foo.out -Z unstable-options --unpretty hir=nest::foo input.rs
[01:02:57] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/pretty-print-path-suffix.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/pretty-print-path-suffix.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/pretty-print-path-suffix.stage2-x86_64-unknown-linux-gnu  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/pretty-print-path-suffix.stage2-x86_64-unknown-linux-gnu/foo_method.out -Z unstable-options --unpretty hir=foo_method input.rs
[01:02:57] diff -u /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/pretty-print-path-suffix.stage2-x86_64-unknown-linux-gnu/foo.out foo.pp
[01:02:57] --- /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/pretty-print-path-suffix.stage2-x86_64-unknown-linux-gnu/foo.out	2017-12-13 17:46:32.935556154 +0000
[01:02:57] +++ foo.pp	2017-12-13 16:42:30.499819848 +0000
[01:02:57] @@ -9,7 +9,7 @@
[01:02:57]  // except according to those terms.
[01:02:57]  
[01:02:57]  
[01:02:57] -pub fn foo<>() -> i32 { 45 } /* foo */
[01:02:57] +pub fn foo() -> i32 { 45 } /* foo */
[01:02:57]  
[01:02:57]  
[01:02:57] -pub fn foo<>() -> &'static str { "i am a foo." } /* nest::foo */
[01:02:57] +pub fn foo() -> &'static str { "i am a foo." } /* nest::foo */
[01:02:57] Makefile:4: recipe for target 'all' failed
[01:02:57] make[1]: Leaving directory '/checkout/src/test/run-make/pretty-print-path-suffix'
[01:02:57] 
[01:02:57] ------------------------------------------
[01:02:57] stderr:
[01:02:57] ------------------------------------------
[01:02:57] make[1]: warning: jobserver unavailable: using -j1.  Add '+' to parent make rule.
[01:02:57] warning: ignoring --out-dir flag due to -o flag.
[01:02:57] 
[01:02:57] warning: ignoring --out-dir flag due to -o flag.
[01:02:57] 
[01:02:57] warning: ignoring --out-dir flag due to -o flag.
[01:02:57] 
[01:02:57] make[1]: *** [all] Error 1
[01:02:57] 
[01:02:57] ------------------------------------------
[01:02:57] 
[01:02:57] thread '[run-make] run-make/pretty-print-path-suffix' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2774:8
[01:02:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:02:57] 
[01:02:57] 
[01:02:57] failures:
[01:02:57]     [run-make] run-make/pretty-print-path-suffix
