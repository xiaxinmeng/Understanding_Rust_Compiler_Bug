plain
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:07] 
[01:06:07] running 89 tests
[01:06:30] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:06:30] ..F.............F....................................F..F...F..FF........................
[01:06:30] 
[01:06:30] ---- [incremental] incremental/change_crate_dep_kind.rs stdout ----
[01:06:30] 
[01:06:30] 
[01:06:30] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:06:30] status: exit code: 101
[01:06:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_dep_kind.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/change_crate_dep_kind.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Cpanic=unwind" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/auxiliary"
[01:06:30] ------------------------------------------
[01:06:30] 
[01:06:30] ------------------------------------------
[01:06:30] stderr:
[01:06:30] stderr:
[01:06:30] ------------------------------------------
[01:06:30] {"message":"librustc/dep_graph/graph.rs:644: DepNode Hir(4fdcb82a799f4ad8-c5f5e0c6c53fced0) should have been pre-allocated but wasn't.","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: librustc/dep_graph/graph.rs:644: DepNode Hir(4fdcb82a799f4ad8-c5f5e0c6c53fced0) should have been pre-allocated but wasn't.\n\n"}
[01:06:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:06:30] 
[01:06:30] note: the compiler unexpectedly panicked. this is a bug.
[01:06:30] 
[01:06:30] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:30] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[01:06:30] 
[01:06:30] 
[01:06:30] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C panic=unwind
[01:06:30] 
[01:06:30] ------------------------------------------
[01:06:30] 
[01:06:30] thread '[incremental] incremental/change_crate_dep_kind.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:06:30] thread '[incremental] incremental/change_crate_dep_kind.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:06:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:30] 
[01:06:30] ---- [incremental] incremental/crate_hash_reorder.rs stdout ----
[01:06:30] 
[01:06:30] error in revision `rpass3`: compilation failed!
[01:06:30] status: exit code: 101
[01:06:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/crate_hash_reorder.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/crate_hash_reorder.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/auxiliary"
[01:06:30] ------------------------------------------
[01:06:30] 
[01:06:30] ------------------------------------------
[01:06:30] stderr:
[01:06:30] stderr:
[01:06:30] ------------------------------------------
[01:06:30] {"message":"librustc/dep_graph/graph.rs:644: DepNode Hir(1a639b735888c92a-2155bcef5a4fba1e) should have been pre-allocated but wasn't.","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: librustc/dep_graph/graph.rs:644: DepNode Hir(1a639b735888c92a-2155bcef5a4fba1e) should have been pre-allocated but wasn't.\n\n"}
[01:06:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:06:30] 
[01:06:30] note: the compiler unexpectedly panicked. this is a bug.
[01:06:30] 
[01:06:30] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:30] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[01:06:30] 
[01:06:30] 
[01:06:30] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:06:30] 
[01:06:30] ------------------------------------------
[01:06:30] 
[01:06:30] thread '[incremental] incremental/crate_hash_reorder.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:06:30] thread '[incremental] incremental/crate_hash_reorder.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:06:30] 
[01:06:30] ---- [incremental] incremental/issue-39569.rs stdout ----
[01:06:30] 
[01:06:30] error in revision `rpass2`: compilation failed!
[01:06:30] status: exit code: 101
[01:06:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-39569.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39569/issue-39569.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39569/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39569/auxiliary"
[01:06:30] ------------------------------------------
[01:06:30] 
[01:06:30] ------------------------------------------
[01:06:30] stderr:
[01:06:30] stderr:
[01:06:30] ------------------------------------------
[01:06:30] {"message":"librustc/dep_graph/graph.rs:644: DepNode Hir(917416e6ae8ffa9a-60701dd2d252815a) should have been pre-allocated but wasn't.","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: librustc/dep_graph/graph.rs:644: DepNode Hir(917416e6ae8ffa9a-60701dd2d252815a) should have been pre-allocated but wasn't.\n\n"}
[01:06:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:06:30] 
[01:06:30] note: the compiler unexpectedly panicked. this is a bug.
[01:06:30] 
[01:06:30] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:30] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[01:06:30] 
[01:06:30] 
[01:06:30] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:06:30] 
[01:06:30] ------------------------------------------
[01:06:30] 
[01:06:30] thread '[incremental] incremental/issue-39569.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:06:30] thread '[incremental] incremental/issue-39569.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:06:30] 
[01:06:30] ---- [incremental] incremental/krate-inherent.rs stdout ----
[01:06:30] 
[01:06:30] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:06:30] status: exit code: 101
[01:06:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/krate-inherent.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inherent/krate-inherent.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inherent/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/o----------------------
[01:06:30] thread '[incremental] incremental/krate-inherent.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:06:30] 
[01:06:30] ---- [incremental] incremental/krate_reassign_34991/main.rs stdout ----
[01:06:30] 
[01:06:30] 
[01:06:30] error in revision `rpass2`: compilation failed!
[01:06:30] status: exit code: 101
[01:06:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/krate_reassign_34991/main.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate_reassign_34991/main/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate_reassign_34991/main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate_reassign_34991/main/auxiliary"
[01:06:30] ------------------------------------------
[01:06:30] 
[01:06:30] ------------------------------------------
[01:06:30] stderr:
[01:06:30] stderr:
[01:06:30] ------------------------------------------
[01:06:30] {"message":"librustc/dep_graph/graph.rs:644: DepNode Hir(48bbd1ae5ea14ff8-b1be713555a028aa) should have been pre-allocated but wasn't.","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: librustc/dep_graph/graph.rs:644: DepNode Hir(48bbd1ae5ea14ff8-b1be713555a028aa) should have been pre-allocated but wasn't.\n\n"}
[01:06:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:06:30] 
[01:06:30] note: the compiler unexpectedly panicked. this is a bug.
[01:06:30] 
[01:06:30] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:30] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[01:06:30] 
[01:06:30] 
[01:06:30] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[01:06:30] 
[01:06:30] ------------------------------------------
[01:06:30] 
[01:06:30] thread '[incremental] incremental/krate_reassign_34991/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:06:30] thread '[incremental] incremental/krate_reassign_34991/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:06:30] 
[01:06:30] ---- [incremental] incremental/remove-private-item-cross-crate/main.rs stdout ----
[01:06:30] 
[01:06:30] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/remove-private-item-cross-crate/auxiliary/a.rs" failed to compile: 
[01:06:30] status: exit code: 101
[01:06:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/remove-private-item-cross-crate/auxiliary/a.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/auxiliary"
[01:06:30] ------------------------------------------
[01:06:30] 
[01:06:30] ------------------------------------------
[01:06:30] stderr:
[01:06:30] stderr:
[01:06:30] ------------------------------------------
[01:06:30] {"message":"librustc/dep_graph/graph.rs:644: DepNode Hir(e4b0bbe3a744e8df-dcf1a4426be54552) should have been pre-allocated but wasn't.","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: librustc/dep_graph/graph.rs:644: DepNode Hir(e4b0bbe3a744e8df-dcf1a4426be54552) should have been pre-allocated but wasn't.\n\n"}
[01:06:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:06:30] 
[01:06:30] note: the compiler unexpectedly panicked. this is a bug.
[01:06:30] 
[01:06:30] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:30] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[01:06:30] 
[01:06:30] 
[01:06:30] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:06:30] 
[01:06:30] ------------------------------------------
[01:06:30] 
[01:06:30] thread '[incremental] incremental/remove-private-item-cross-crate/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:06:30] thread '[incremental] incremental/remove-private-item-cross-crate/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:06:30] 
[01:06:30] ---- [incremental] incremental/remove_crate/main.rs stdout ----
[01:06:30] 
[01:06:30] error in revision `rpass2`: compilation failed!
[01:06:30] status: exit code: 101
[01:06:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/remove_crate/main.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove_crate/main/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove_crate/main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove_crate/main/auxiliary"
[01:06:30] ------------------------------------------
[01:06:30] 
[01:06:30] ------------------------------------------
[01:06:30] stderr:
[01:06:30] stderr:
[01:06:30] ------------------------------------------
[01:06:30] {"message":"librustc/dep_graph/graph.rs:644: DepNode Hir(1e998d763f6a8721-5866ebd393d32511) should have been pre-allocated but wasn't.","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: librustc/dep_graph/graph.rs:644: DepNode Hir(1e998d763f6a8721-5866ebd393d32511) should have been pre-allocated but wasn't.\n\n"}
[01:06:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:06:30] 
[01:06:30] note: the compiler unexpectedly panicked. this is a bug.
[01:06:30] 
[01:06:30] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:30] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[01:06:30] 
[01:06:30] 
[01:06:30] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[01:06:30] 
[01:06:30] ------------------------------------------
[01:06:30] 
[01:06:30] thread '[incremental] incremental/remove_crate/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
---
[01:06:30] test result: FAILED. 82 passed; 7 failed; 0 ignored; 0 measured; 0 filtered out
[01:06:30] 
[01:06:30] 
[01:06:30] 
[01:06:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:30] 
[01:06:30] 
[01:06:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:30] Build completed unsuccessfully in 0:17:43
[01:06:30] Build completed unsuccessfully in 0:17:43
[01:06:30] make: *** [check] Error 1
[01:06:30] Makefile:58: recipe for target 'check' failed
