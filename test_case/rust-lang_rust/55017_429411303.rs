plain
[00:53:33] .................................................................................................... 2200/4594
[00:53:37] ..............i..................................................................................... 2300/4594
[00:53:41] .................................................................................................... 2400/4594
[00:53:44] .................................................................................................... 2500/4594
[00:53:48] ...........................iiiiiiiii................................................................ 2600/4594
[00:53:51] ........................................................F.........ii................................ 2700/4594
[00:53:58] .................................................................................................... 2900/4594
[00:54:01] .................................................i.................................................. 3000/4594
[00:54:04] .................................................................................................... 3100/4594
[00:54:07] .........i.i..ii.................................................................................... 3200/4594
---
[00:54:52] 
[00:54:52] ---- [ui (nll)] ui/issues/issue-52240.rs stdout ----
[00:54:52] diff of stderr:
[00:54:52] 
[00:54:52] - error[E0596]: cannot borrow field of immutable binding as mutable
[00:54:52] + error[E0596]: cannot borrow data in a `&` reference as mutable
[00:54:52] 3    |
[00:54:52] 3    |
[00:54:52] 4 LL |     if let (Some(Foo::Bar(ref mut val)), _) = (&arr.get(0), 0) {
[00:54:52] -    |                           ^^^^^^^^^s/compiletest/src/runtest.rs:3277:9
[00:54:52] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:54:52] 
[00:54:52] 
---
[00:54:52] 
[00:54:52] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:501:22
[00:54:52] 
[00:54:52] 
[00:54:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:54:52] 
[00:54:52] 
[00:54:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:54:52] Build completed unsuccessfully in 0:06:08
[00:54:52] Build completed unsuccessfully in 0:06:08
[00:54:52] make: *** [check] Error 1
[00:54:52] Makefile:58: recipe for target 'check' failed
2562528 ./obj
2562492 ./obj/build
1929952 ./obj/build/x86_64-unknown-linux-gnu
1069844 ./src
