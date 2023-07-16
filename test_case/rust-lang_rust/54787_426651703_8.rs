\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-20605.rs","byte_start":544,"byte_end":551,"line_start":12,"line_end":12,"column_start":17,"column_end":24,"is_primary":true,"text":[{"text":"    for item in *things { *item = ndered":"error[E0277]: the size for values of type `dyn std::iter::Iterator<Item=&mut u8>` cannot be known at compilation time\n  --> /checkout/src/test/ui/issues/issue-20605.rs:12:17\n   |\nLL |     for item in *things { *item = 0 }\n   |                 ^^^^^^^ doesn't have a size known at compile-time\n   |\n   = help: the trait `std::marker::Sized` is not implemented for `dyn std::iter::Iterator<Item=&mut u8>`\n   = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>\n   = note: all local variables must have a statically known size\n   = help: unsized locals are gated as an unstable feature\n\n"}
[00:51:50] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:51:50] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[00:51:50] ------------------------------------------
[00:51:50] 
[00:51:50] thread '[ui] ui/issues/issue-20605.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:51:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:51:50] 
[00:51:50] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:499:22
[00:51:50] 
[00:51:50] 
[00:51:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:50] 
[00:51:50] 
[00:51:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:50] Build completed unsuccessfully in 0:03:36
[00:51:50] Build completed unsuccessfully in 0:03:36
[00:51:50] make: *** [check] Error 1
[00:51:50] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:211c9274
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
