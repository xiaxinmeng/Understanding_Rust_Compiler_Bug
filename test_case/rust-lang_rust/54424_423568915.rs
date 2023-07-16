plain
[00:52:49] ....................................................................................................
[00:52:52] .......................................................i............................................
[00:52:55] ....................................................................................................
[00:52:58] ....................................................................................................
[00:53:01] ...iiiiiiiii........................................................................................
[00:53:07] ....................................................................................................
[00:53:10] .......................................................................................i............
[00:53:12] ....................................................................................................
[00:53:15] ..........................................i.i..ii...................................................
---
[00:59:30] ---- [run-pass] run-pass/rvalue-static-promotion.rs stdout ----
[00:59:30] 
[00:59:30] error: compilation failed!
[00:59:30] status: exit code: 1
[00:59:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/rvalue-static-promotion.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rvalue-static-promotion/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rvalue-static-promotion/auxiliary"
[00:59:30] ------------------------------------------
[00:59:30] 
[00:59:30] ------------------------------------------
[00:59:30] stderr:
[00:59:30] stderr:
[00:59:30] ------------------------------------------
[00:59:30] error[E0597]: borrowed value does not live long enough
[00:59:30]    |
[00:59:30]    |
[00:59:30] 25 |     let _: &'static Option<Cell<String>> = &NONE_CELL_STRING;
[00:59:30]    |                                             ^^^^^^^^^^^^^^^^ temporary value does not live long enough
[00:59:30] 26 |     let _: &'static Option<Box<()>> = &Foo::FOO;
[00:59:30] 27 | }
[00:59:30]    | - temporary value only lives until here
[00:59:30]    |
[00:59:30]    = note: borrowed value must be valid for the static lifetime...
[00:59:30] 
[00:59:30] error[E0597]: borrowed value does not live long enough
[00:59:30]    |
[00:59:30]    |
[00:59:30] 26 |     let _: &'static Option<Box<()>> = &Foo::FOO;
[00:59:30]    |                                        ^^^^^^^^ temporary value does not live long enough
[00:59:30] 27 | }
[00:59:30]    | - temporary value only lives until here
[00:59:30]    |
[00:59:30]    = note: borrowed value must be valid for the static lifetime...
[00:59:30] error: aborting due to 2 previous errors
[00:59:30] 
[00:59:30] For more information about this error, try `rustc --explain E0597`.
[00:59:30] 
---
[00:59:30] 
[00:59:30] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[00:59:30] 
[00:59:30] 
[00:59:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:30] 
[00:59:30] 
[00:59:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:30] Build completed unsuccessfully in 0:15:13
[00:59:30] Build completed unsuccessfully in 0:15:13
[00:59:30] make: *** [check] Error 1
[00:59:30] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ab30daa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
