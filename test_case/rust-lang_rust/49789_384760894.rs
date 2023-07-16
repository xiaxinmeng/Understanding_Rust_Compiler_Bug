plain
[00:57:35] ......i....................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:57:39] .........
[00:58:13] ....................................................................................................
[00:58:42] .......................................................................ii...........................
[00:59:35] ...................................i....................................................i.ii..test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[01:00:25] ................................................................................................iiii
[01:00:52] iii.................................................................................................
[01:01:22] ....................................................................................................
[01:01:50] ....................................................................................................
---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:36:39] 
[01:36:39] running 183 tests
[01:37:27] .................................................F..................................................
[01:38:16] ..................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:39:26] failures:
[01:39:26] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:39:26] 
[01:39:26] ---- [run-make] run-make-fulldeps/extern-prelude stdout ----
[01:39:26] ---- [run-make] run-make-fulldeps/extern-prelude stdout ----
[01:39:26]  
[01:39:26] error: make failed
[01:39:26] status: exit code: 2
[01:39:26] command: "make"
[01:39:26] stdout:
[01:39:26] ------------------------------------------
[01:39:26] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/extern-prelude'
[01:39:26] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu  ep-lib.rs
[01:39:26] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknow4-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu  inner-mod.rs --extern S=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu/libep_vec.rlib --extern ep_lib=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu/libep_lib.rlib
[01:39:26] Makefile:4: recipe for target 'all' failed
[01:39:26] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/extern-prelude'
[01:39:26] ------------------------------------------
[01:39:26] stderr:
[01:39:26] ------------------------------------------
[01:39:26] warning: unused variable: `arg1`
[01:39:26] warning: unused variable: `arg1`
[01:39:26]   --> ep-vec.rs:13:12
[01:39:26]    |
[01:39:26] 13 | pub fn new(arg1: f32, arg2: ()) {}
[01:39:26]    |            ^^^^ help: consider using `_arg1` instead
[01:39:26]    = note: #[warn(unused_variables)] on by default
[01:39:26] 
[01:39:26] warning: unused variable: `arg2`
[01:39:26] warning: unused variable: `arg2`
[01:39:26]   --> ep-vec.rs:13:23
[01:39:26]    |
[01:39:26] 13 | pub fn new(arg1: f32, arg2: ()) {}
[01:39:26]    |                       ^^^^ help: consider using `_arg2` instead
[01:39:26] warning: unused variable: `x`
[01:39:26] warning: unused variable: `x`
[01:39:26]   --> shadow-prelude.rs:16:9
[01:39:26]    |
[01:39:26] 16 |     let x = Vec::new(0f32, ()); // OK
[01:39:26]    |         ^ help: consider using `_x` instead
[01:39:26]    = note: #[warn(unused_variables)] on by default
[01:39:26] 
[01:39:26] 
[01:39:26] error[E0433]: failed to resolve. Use of undeclared type or module `ep_lib`
[01:39:26]   --> inner-mod.rs:15:13
[01:39:26]    |
[01:39:26] 15 |     let s = ep_lib::S;
[01:39:26]    |             ^^^^^^ Use of undeclared type or module `ep_lib`
[01:39:26] error: aborting due to previous error
[01:39:26] 
[01:39:26] For more information about this error, try `rustc --explain E0433`.
[01:39:26] For more information about this error, try `rustc --explain E0433`.
[01:39:26] make[1]: *** [all] Error 101
[01:39:26] ------------------------------------------
[01:39:26] 
[01:39:26] 
[01:39:26] thread '[run-make] run-make-fulldeps/extern-prelude' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2963:9
[01:39:26] 
[01:39:26] 
[01:39:26] failures:
[01:39:26]     [run-make] run-make-fulldeps/extern-prelude
[01:39:26]     [run-make] run-make-fulldeps/extern-prelude
[01:39:26] 
[01:39:26] test result: FAILED. 182 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:39:26] 
[01:39:26] 
[01:39:26] 
[01:39:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_6tion orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-3.9/include -std=c++0x -gsplit-dwarf -Wl,-fuse-ld=gold -fPIC -fvisibility-inlines-hidden -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -Werror=date-time -std=c++11 -ffunction-sections -fdata-sections -O2 -g -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:39:26] 
[01:39:26] 
[01:39:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:39:26] Build completed unsuccessfully in 0:52:47
[01:39:26] Build completed unsuccessfully in 0:52:47
[01:39:26] make: *** [check] Error 1
[01:39:26] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1562ebb7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
