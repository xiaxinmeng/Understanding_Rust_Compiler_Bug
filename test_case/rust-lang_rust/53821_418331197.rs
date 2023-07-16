plain
[00:47:47] ....................................................................................................
[00:47:50] ....................................................................................................
[00:47:52] ............................i.......................................................................
[00:47:55] ....................................................................................................
[00:47:58] .............................................................................iiiiiiiii..............
[00:48:03] i...................................................................................................
[00:48:07] ....................................................................................................
[00:48:10] ..........................................................i.........................................
[00:48:12] ....................................................................................................
---
[00:55:13] ---- [codegen-units] codegen-units/partitioning/vtable-through-const.rs stdout ----
[00:55:13] 
[00:55:13] These items were contained but should not have been:
[00:55:13] 
[00:55:13] MONO_ITEM fn vtable_through_const::mod1[0]::Trait2[0]::do_something[0]<u32> @@ vtable_through_const.7rcbfp3g-mod1.volatile[Internal]
[00:55:13] MONO_ITEM fn vtable_through_const::mod1[0]::Trait2[0]::do_something_else[0]<u32> @@ vtable_through_const.7rcbfp3g-mod1.volatile[Internal]
[00:55:13] MONO_ITEM fn vtable_through_const::mod1[0]::id[0]<i64> @@ vtable_through_const.7rcbfp3g-mod1.volatile[Internal]
[00:55:13] MONO_ITEM fn vtable_through_const::mod1[0]::{{impl}}[3]::do_something[0]<u8> @@ vtable_through_const.7rcbfp3g-mod1.volatile[Internal]
[00:55:13] MONO_ITEM fn vtable_through_const::mod1[0]::{{impl}}[3]::do_something_else[0]<u8> @@ vtable_through_const.7rcbfp3g-mod1.volatile[Internal]
[00:55:13] 
[00:55:13] thread '[codegen-units] codegen-units/partitioning/vtable-through-const.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:55:13] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:55:13] 
---
[00:55:13] 
[00:55:13] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:55:13] 
[00:55:13] 
[00:55:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:13] 
[00:55:13] 
[00:55:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:55:13] Build completed unsuccessfully in 0:11:22
[00:55:13] Build completed unsuccessfully in 0:11:22
[00:55:13] Makefile:58: recipe for target 'check' failed
[00:55:13] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:010aedcf
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:02a9ca38:start=1536060213691494067,finish=1536060213781784126,duration=90290059
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1fc55462
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:15969168
$ dmesg | grep -i kill
