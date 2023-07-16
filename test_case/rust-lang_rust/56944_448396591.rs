plain
travis_time:end:0bc29572:start=1545167080480896013,finish=1545167189172278866,duration=108691382853
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:48] 
[00:54:48] running 119 tests
[00:55:11] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[00:55:15] i......iii.i.....ii
[00:55:15] 
[00:55:15]  finished in 26.905
[00:55:15] travis_fold:end:test_debuginfo

---
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:45] 
[01:21:45] running 193 tests
[01:22:10] .................................................................................................... 100/193
[01:23:08] ..F.........................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
d-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -c llvm-module-pass.so.cc -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/llvm-pass/llvm-pass/libllvm-module-pass.o
[01:25:48] ar crus /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/llvm-pass/llvm-pass/libllvm-module-pass.a /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/llvm-pass/llvm-pass/libllvm-module-pass.o
[01:25:48] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/llvm-pass/llvm-pass:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/llvm-pass/llvm-pass -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/llvm-pass/llvm-pass  plugin.rs -C prefer-dynamic 
[01:25:48] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/llvm-pass/llvm-pass:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/llvm-pass/llvm-pass -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/llvm-pass/llvm-pass  main.rs
[01:25:48] Makefile:18: recipe for target 'all' failed
[01:25:48] 
[01:25:48] ------------------------------------------
[01:25:48] stderr:
[01:25:48] ------------------------------------------
[01:25:48] ------------------------------------------
[01:25:48] ar: `u' modifier ignored since `D' is the default (see `U')
[01:25:48] ar: `u' modifier ignored since `D' is the default (see `U')
[01:25:48] error: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/llvm-pass/llvm-pass/libsome_plugin.so: undefined symbol: _ZTVN10__cxxabiv120__si_class_type_infoE
[01:25:48]   --> main.rs:12:11
[01:25:48]    |
[01:25:48] 12 | #![plugin(some_plugin)]
[01:25:48] 
[01:25:48] error: aborting due to previous error
[01:25:48] 
[01:25:48] 
[01:25:48] make[1]: *** [all] Error 1
[01:25:48] ------------------------------------------
[01:25:48] 
[01:25:48] thread '[run-make] run-make-fulldeps/llvm-pass' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:25:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
travis_time:end:0a174afd:start=1545172349018592907,finish=1545172349026639655,duration=8046748
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04d5f55d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_f
