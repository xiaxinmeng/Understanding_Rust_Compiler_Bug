plain
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:18] 
[01:04:18] running 89 tests
[01:04:40] .........................F...............................................................
[01:04:40] 
[01:04:40] ---- [incremental] incremental/hashes/function_interfaces.rs stdout ----
[01:04:40] 
[01:04:40] 
[01:04:40] error in revision `cfail1`: test compilation failed although it shouldn't!
[01:04:40] status: exit code: 101
[01:04:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
[01:04:40] ------------------------------------------
[01:04:40] 
[01:04:40] ------------------------------------------
[01:04:40] stderr:
[01:04:40] stderr:
[01:04:40] ------------------------------------------
[01:04:40] thread 'main' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', librustc/dep_graph/graph.rs:1074:9
[01:04:40] 
[01:04:40] error: internal compiler error: unexpected panic
[01:04:40] 
[01:04:40] 
[01:04:40] note: the compiler unexpectedly panicked. this is a bug.
[01:04:40] 
[01:04:40] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:40] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[01:04:40] 
[01:04:40] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:04:40] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:04:40] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:04:40] 
[01:04:40] 
[0onents" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:40] 
[01:04:40] 
[01:04:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:40] Build completed unsuccessfully in 0:17:07
[01:04:40] Build completed unsuccessfully in 0:17:07
[01:04:40] make: *** [check] Error 1
[01:04:40] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2507981d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
