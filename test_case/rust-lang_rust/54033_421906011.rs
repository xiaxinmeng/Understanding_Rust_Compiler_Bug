plain
[00:57:55] ....................................................................................................
[00:57:58] .......................................................i............................................
[00:58:01] ....................................................................................................
[00:58:04] ....................................................................................................
[00:58:07] ...iiiiiiiii........................................................................................
[00:58:13] ....................................................................................................
[00:58:16] ....................................................................................i...............
[00:58:19] ....................................................................................................
[00:58:22] ......................................i.i..ii.......................................................
---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:43] 
[01:11:43] running 260 tests
[01:12:13] ...........F...........i............................................................................
[01:12:49] ............................................................
[01:12:49] failures:
[01:12:49] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:12:49] 
[01:12:49] 
[01:12:49] ---- [rustdoc] rustdoc/const-evalutation-ice.rs stdout ----
[01:12:49] 
[01:12:49] error: rustdoc failed!
[01:12:49] status: exit code: 1
[01:12:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-evalutation-ice/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-evalutation-ice" "/checkout/src/test/rustdoc/const-evalutation-ice.rs"
[01:12:49] ------------------------------------------
[01:12:49] 
[01:12:49] ------------------------------------------
[01:12:49] stderr:
[01:12:49] stderr:
[01:12:49] -------------------------------------4-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:49] 
[01:12:49] 
[01:12:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:49] Build completed unsuccessfully in 0:24:54
[01:12:49] Build completed unsuccessfully in 0:24:54
[01:12:49] Makefile:58: recipe for target 'check' failed
[01:12:49] make: *** [check] Error 1
2977752 ./obj
2949468 ./obj/build
2329692 ./obj/build/x86_64-unknown-linux-gnu
1187536 ./.git
---
151412 ./src/tools/clang
149128 ./src/llvm-emscripten/test
149068 ./obj/build/bootstrap/debug/incremental
134624 ./obj/build/bootstrap/debug/incremental/bootstrap-14ucxxomeo8gg
134620 ./obj/build/bootstrap/debug/incremental/bootstrap-14ucxxomeo8gg/s-f4vms60yix-19zbptk-2f6sgdh2xj16g
131252 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
129780 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
129776 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
128332 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
