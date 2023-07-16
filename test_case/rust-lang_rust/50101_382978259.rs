plain
[00:44:34] .........................i..........................................................................
[00:44:38] ....................................................................................................
[00:44:42] ....................................................................................................
[00:44:46] ....................................................................................................
[00:44:50] ...........................................................F...FF.F.................................
[00:45:02] ....................................................................................................
[00:45:08] ....................................................................................................
[00:45:15] .......................i...........................................................................i
[00:45:21] ....................................................................................................
[00:45:21] ....................................................................................................
[00:45:27] .............ii...................................................................................F.
[00:45:35] ..............................................................................................i.....
   mod inner { #![derive(Debug)] }
[00:45:38] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:45:38] -    |                 ^^^^^^^^^^^^^^^^^ help: try an outer attribute: `#[derive(Debug)]`
[00:45:38] - 
[00:45:38] - error: `derive` may only be applied to structs, enums and unions
[00:45:38] -   --> $DIR/issue-43106-gating-of-derive.rs:23:5
[00:45:38] -    |
[00:45:38] - LL |     #[derive(Debug)]
[00:45:38] - 
[00:45:38] - 
[00:45:38] - error: `derive` may only be applied to structs, enums and unions
[00:45:38] -   --> $DIR/issue-43106-gating-of-derive.rs:36:5
[00:45:38] -    |
[00:45:38] - LL |     #[derive(Debug)]
[00:45:38] - 
[00:45:38] - 
[00:45:38] - error: `derive` may only be applied to structs, enums and unions
[00:45:38] -   --> $DIR/issue-43106-gating-of-derive.rs:40:5
[00:45:38] -    |
[00:45:38] - LL |     #[derive(Debug)]
[00:45:38] - 
[00:45:38] - error: aborting due to 6 previous errors
[00:45:38] - 
[00:45:38] - 
[00:45:38] - 
[00:45:38] 
[00:45:38] 
[00:45:38] error: failed to write stderr to `/checkout/src/test/ui/feature-gate/issue-43106-gating-of-derive.stderr`: Read-only file system (os error 30)
[00:45:38] thread '[ui] ui/feature-gate/issue-43106-gating-of-derive.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1889:9
[00:45:38] 
[00:45:38] ---- [ui] ui/feature-gate/issue-43106-gating-of-macro_use.rs stdout ----
[00:45:38]  diff of stderr:
[00:45:38] 
[00:45:38] 
[00:45:38] 1 error: argumeest/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:45:38] 
[00:45:38] 
[00:45:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:38] Build completed unsuccessfully in 0:02:27
[00:45:38] Build completed unsuccessfully in 0:02:27
[00:45:38] Makefile:58: recipe for target 'check' failed
[00:45:38] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0bdf48a0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
56092 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin
55340 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
53564 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build
52060 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-33oa6nnkk1g08
52056 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-33oa6nnkk1g08/s-f0a85r4gez-1iutfhd-2h2c6e0haira
47824 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
47496 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps
46652 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
46648 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
