plain
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:34] 
[01:24:34] running 183 tests
[01:25:16] ......................................................F.............................................
[01:26:07] ..................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
ke-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu  ep-vec.rs
[01:27:19] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu  basic.rs --extern ep_lib=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu/libep_lib.rlib
[01:27:19] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu  shadow-mod.rs --extern ep_lib=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu/libep_lib.rlib
[01:27:19] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu  shadow-prelude.rs --extern Vec=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu/libep_vec.rlib
[01:27:19] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu  feature-gate.rs --extern ep_lib=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu/libep_lib.rlib 2>&1 | "/checkout/src/etc/cat-and-grep.sh" "access to extern crates through prelude is experimental"
[01:27:19] [[[ begin stdout ]]]
[01:27:19] error[E0433]: failed to resolve. Use of undeclared type or module `ep_lib`
[01:27:19]    |
[01:27:19]    |
[01:27:19] 12 |     let s = ep_lib::S; // Feature error
[01:27:19]    |             ^^^^^^ Use of undeclared type or module `ep_lib`
[01:27:19] error: aborting due to previous error
[01:27:19] 
[01:27:19] For more information about this error, try `rustc --explain E0433`.
[01:27:19] 
[01:27:19] 
[01:27:19] [[[ end stdout ]]]
[01:27:19] Error: cannot match: access to extern crates through prelude is experimental
[01:27:19] Makefile:4: recipe for target 'all' failed
[01:27:19] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/extern-prelude'
[01:27:19] ------------------------------------------
[01:27:19] stderr:
[01:27:19] ------------------------------------------
[01:27:19] warning: unused variable: `arg1`
[01:27:19] warning: unused variable: `arg1`
[01:27:19]   --> ep-vec.rs:13:12
[01:27:19]    |
[01:27:19] 13 | pub fn new(arg1: f32, arg2: ()) {}
[01:27:19]    |            ^^^^ help: consider using `_arg1` instead
[01:27:19]    = note: #[warn(unused_variables)] on by default
[01:27:19] 
[01:27:19] warning: unused variable: `arg2`
[01:27:19] warning: unused variable: `arg2`
[01:27:19]   --> ep-vec.rs:13:23
[01:27:19]    |
[01:27:19] 13 | pub fn new(arg1: f32, arg2: ()) {}
[01:27:19]    |                       ^^^^ help: consider using `_arg2` instead
[01:27:19] warning: unused variable: `x`
[01:27:19] warning: unused variable: `x`
[01:27:19]   --> shadow-prelude.rs:16:9
[01:27:19]    |
[01:27:19] 16 |     let x = Vec::new(0f32, ()); // OK
[01:27:19]    |         ^ help: consider using `_x` instead
[01:27:19]    = note: #[warn(unused_variables)] on by default
[01:27:19] 
[01:27:19] 
[01:27:19] make[1]: *** [all] Error 1
[01:27:19] ------------------------------------------
[01:27:19] 
[01:27:19] 
[01:27:19] thread '[run-make] run-make-fulldeps/extern-prelude' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2963:9
[01:27:19] 
[01:27:19] 
[01:27:19] failures:
[01:27:19]     [run-make] run-make-fulldeps/extern-prelude
[01:27:19]     [run-make] run-make-fulldeps/extern-prelude
[01:27:19] 
[01:27:19] test result: FAILED. 182 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:27:19] 
[01:27:19] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:27:19] 
[01:27:19] 
[01:27:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdo64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu
78548 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental
74964 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/release
70976 ./obj/build/x86_64-unknown-linux-gnu/native
70692 ./.git/modules/src/tools
---
55348 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
53568 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build
53220 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps
52148 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-33oa6nnkk1g08
52144 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-33oa6nnkk1g08/s-f0gnwugntb-mb5fty-2d2hclipyua36
47832 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
47268 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools
46824 ./src/test
46660 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
