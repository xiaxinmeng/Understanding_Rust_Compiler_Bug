plain
[00:53:46] ....................................................................................................
[00:53:49] .......................................................i............................................
[00:53:52] ....................................................................................................
[00:53:55] ....................................................................................................
[00:53:58] ...iiiiiiiii........................................................................................
[00:54:04] ....................................................................................................
[00:54:07] ....................................................................................i...............
[00:54:10] ....................................................................................................
[00:54:13] ......................................i.i..ii.......................................................
---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:13] 
[01:06:13] running 260 tests
[01:06:39] F...........F..........i............................................................................
[01:07:12] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:07:12] ............................................................
[01:07:12] failures:
[01:07:12] 
[01:07:12] 
[01:07:12] ---- [rustdoc] rustdoc/assoc-item-cast.rs stdout ----
[01:07:12] 
[01:07:12] error: rustdoc failed!
[01:07:12] status: exit code: 1
[01:07:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-item-cast/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-item-cast" "/checkout/src/test/rustdoc/assoc-item-cast.rs"
[01:07:12] ------------------------------------------
[01:07:12] 
[01:07:12] ------------------------------------------
[01:07:12] stderr:
[01:07:12] stderr:
[01:07:12] ------------------------------------------
[01:07:12] error[nux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:12] 
[01:07:12] 
[01:07:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:12] Build completed unsuccessfully in 0:21:58
[01:07:12] Build completed unsuccessfully in 0:21:58
[01:07:12] make: *** [check] Error 1
[01:07:12] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:028d30e0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
