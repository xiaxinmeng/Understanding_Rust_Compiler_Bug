plain
Resolving deltas: 100% (613114/613114), completed with 4852 local objects.
---
[00:00:42] configure: rust.quiet-tests     := True
---
[00:40:59] .........................................................................i..........................
[00:41:05] ................i...................................................................................
[00:41:09] ....................................................................................................
[00:41:13] ....................................................................................................
[00:41:17] ....................................................................................................
[00:41:22] ....................................................................................................
[00:41:28] ..........................................................................F.........................
[00:41:34] ...................F................................................................................
[00:41:41] ............................................................................................i.......
[00:41:48] ................................................................i...................................
---
[00:42:10] - error[E0391]: cyclic dependency detected
[00:42:10] -   --> $DIR/issue-23302-1.rs:14:9
[00:42:10] -    |
[00:42:10] - LL |     A = X::A as isize, //~ ERROR E0391
[00:42:10] -    |         ^^^^^^^^^^^^^ cyclic reference
[00:42:10] -    |
[00:42:10] - note: the cycle begins when const-evaluating `X::A::{{initializer}}`...
[00:42:10] -   --> $DIR/issue-23302-1.rs:14:9
[00:42:10] -    |
[00:42:10] - LL |     A = X::A as isize, //~ ERROR E0391
[00:42:10] -    |         ^^^^^^^^^^^^^
[00:42:10] - note: ...which then requires computing layout of `X`...
[00:42:10] -   --> $DIR/issue-23302-1.rs:14:9
[00:42:10] -    |
[00:42:10] - LL |     A = X::A as isize, //~ ERROR E0391
[00:42:10] -    |         ^^^^
[00:42:10] -    = note: ...which then again requires const-evaluating `X::A::{{initializer}}`, completing the cycle.
[00:42:10] + error: internal compiler error: librustc_mir/interpret/eval_context.rs:1243: primitive read failed for type: X
---
[00:42:10] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'issue-23302-1.rs'
[00:42:10]
[00:42:10] error: 1 errors occurred comparing output.
[00:42:10] status: exit code: 101
[00:42:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-23302-1.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-23302-1.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-23302-1.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:42:10] {"message":"librustc_mir/interpret/eval_context.rs:1243: primitive read failed for type: X","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: librustc_mir/interpret/eval_context.rs:1243: primitive read failed for type: X\n\n"}
[00:42:10] thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:543:9
[00:42:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:42:10] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:42:10]
[00:42:10] note: the compiler unexpectedly panicked. this is a bug.
[00:42:10]
[00:42:10] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:42:10]
[00:42:10] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:42:10]
[00:42:10] note: compiler flags: -Z ui-testing -Z miri -Z unstable-options -C prefer-dynamic -C rpath
---
[00:42:10] - error[E0391]: cyclic dependency detected
[00:42:10] -   --> $DIR/issue-36163.rs:14:9
[00:42:10] -    |
[00:42:10] - LL |     B = A, //~ ERROR E0391
[00:42:10] -    |         ^ cyclic reference
[00:42:10] -    |
[00:42:10] - note: the cycle begins when const-evaluating `Foo::B::{{initializer}}`...
[00:42:10] -   --> $DIR/issue-36163.rs:14:9
[00:42:10] -    |
[00:42:10] - LL |     B = A, //~ ERROR E0391
[00:42:10] -    |         ^
[00:42:10] - note: ...which then requires processing `Foo::B::{{initializer}}`...
[00:42:10] -   --> $DIR/issue-36163.rs:14:9
[00:42:10] -    |
[00:42:10] - LL |     B = A, //~ ERROR E0391
[00:42:10] -    |  ic -C rpath
---
[00:42:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:42:10] expected success, got: exit code: 101
[00:42:10]
[00:42:10]
[00:42:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:42:10] Build completed unsuccessfully in 0:02:04
[00:42:10] Makefile:58: recipe for target 'check' failed
[00:42:10] make: *** [check] Error 1
