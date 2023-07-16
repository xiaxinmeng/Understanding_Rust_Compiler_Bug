plain
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:14:53] 
[01:14:53] running 190 tests
[01:15:23] ......................F.............................................................................
[01:16:17] .........................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:17:06] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:17:06] failures:
[01:17:06] 
[01:17:06] 
[01:17:06] ---- [run-make] run-make-fulldeps/cross-lang-lto-upstream-rlibs stdout ----
[01:17:06] error: make failed
[01:17:06] status: exit code: 2
[01:17:06] command: "make"
[01:17:06] stdout:
[01:17:06] stdout:
[01:17:06] ------------------------------------------
[01:17:06] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/cross-lang-lto-upstream-rlibs'
[01:17:06] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-upstream-rlibs/cross-lang-lto-upstream-rlibs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-upstream-rlibs/cross-lang-lto-upstream-rlibs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-upstream-rlibs/cross-lang-lto-upstream-rlibs  upstream.rs -Z cross-lang-lto -Ccodegen-units=1
[01:17:06] # Check No LTO
[01:17:06] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-upstream-rlibs/cross-lang-lto-upstream-rlibs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-upstream-rlibs/cross-lang-lto-upstream-rlibs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-upstream-rlibs/cross-lang-lto-upstream-rlibs  staticlib.rs -Z cross-lang-lto -Ccodegen-units=1 -L. -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-upstream-rlibs/cross-lang-lto-upstream-rlibs/staticlib.a
[01:17:06] (cd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-upstream-rlibs/cross-lang-lto-upstream-rlibs; llvm-ar x ./staticlib.a)
[01:17:06] # Make sure the upstream object file was included
[01:17:06] ls upstream.*.rcgu.o
[01:17:06] Makefile:8: recipe for target 'all' failed
[01:17:06] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/cross-lang-lto-upstream-rlibs'
[01:17:06] ------------------------------------------
[01:17:06] stderr:
[01:17:06] ------------------------------------------
[01:17:06] ------------------------------------------
[01:17:06] warning: ignoring --out-dir flag due to -o flag
[01:17:06] 
[01:17:06] ls: cannot access 'upstream.*.rcgu.o': No such file or directory
[01:17:06] make[1]: *** [all] Error 2
[01:17:06] ------------------------------------------
[01:17:06] 
[01:17:06] 
[01:17:06] thread '[run-make] run-make-fulldeps/cross-lang-lto-upstream-rlibs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:17:06] 
[01:17:06] 
