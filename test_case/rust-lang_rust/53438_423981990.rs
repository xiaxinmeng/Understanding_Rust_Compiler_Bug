plain
[01:10:11] test [mir-opt] mir-opt/loop_test.rs ... ok
[01:10:11] test [mir-opt] mir-opt/nll/named-lifetimes-basic.rs ... ok
[01:10:11] test [mir-opt] mir-opt/packed-struct-drop-aligned.rs ... ignored
[01:10:11] test [mir-opt] mir-opt/nll/region-subtyping-basic.rs ... ok
[01:10:12] ERROR 2018-09-24T13:50:47Z: compiletest::runtest: Some("bb10: {")
[01:10:12] test [mir-opt] mir-opt/remove_fake_borrows.rs ... FAILED
[01:10:12] test [mir-opt] mir-opt/simplify_if.rs ... ok
[01:10:13] test [mir-opt] mir-opt/storage_ranges.rs ... ok
[01:10:13] test [mir-opt] mir-opt/match_false_edges.rs ... ok
[01:10:13] test [mir-opt] mir-opt/validate_1.rs ... ok
---
[01:10:16] test [mir-opt] mir-opt/storage_live_dead_in_statics.rs ... ok
[01:10:16] 
[01:10:16] failures:
[01:10:16] 
[01:10:16] ---- [mir-opt] mir-opt/remove_fake_borrows.rs stdout ----
[01:10:16] thread '[mir-opt] mir-opt/remove_fake_borrows.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:10:16] Current block: bb10: {
[01:10:16] Actual Line: "}"
[01:10:16] Expected Line: "bb10: {"
[01:10:16] Test Name: rustc.match_guard.CleanFakeReadsAndBorrows.before.mir
[01:10:16] ... (elided)
[01:10:16] bb0: {
[01:10:16] bb0: {
[01:10:16]     FakeRead(ForMatchedPlace, _1);
[01:10:16]     _2 = discriminant(_1);
[01:10:16]     _3 = &shallow _1;
[01:10:16]     _4 = &shallow ((_1 as Some).0: &'<empty> &'<empty> i32);
[01:10:16]     _5 = &shallow (*((_1 as Some).0: &'<empty> &'<empty> i32));
[01:10:16]     _6 = &shallow (*(*((_1 as Some).0: &'<empty> &'<empty> i32)));
[01:10:16]     switchInt(move _2) -> [1isize: bb6, otherwise: bb4];
[01:10:16] }
[01:10:16] bb1: {
[01:10:16]     _0 = const 0i32;
[01:10:16]     goto -> bb9;
[01:10:16] }
[01:10:16] bb2: {
[01:10:16]     _0 = const 1i32;
[01:10:16]     goto -> bb9;
[01:10:16] }
[01:10:16] bb3: {
[01:10:16]     FakeRead(ForMatchGuard, _3);
[01:10:16]     FakeRead(ForMatchGuard, _4);
[01:10:16]     FakeRead(ForMatchGuard, _5);
[01:10:16]     FakeRead(ForMatchGuard, _6);
[01:10:16]     goto -> bb7;
[01:10:16] }
[01:10:16] bb4: {
[01:10:16]     FakeRead(ForMatchGuard, _3);
[01:10:16]     FakeRead(ForMatchGuard, _4);
[01:10:16]     FakeRead(ForMatchGuard, _5);
[01:10:16]     FakeRead(ForMatchGuard, _6);
[01:10:16]     goto -> bb2;
[01:10:16] }
[01:10:16] bb5: {
[01:10:16] }
[01:10:16] bb6: {
[01:10:16] bb6: {
[01:10:16]     switchInt((*(*((_1 as Some).0: &'<empty> &'<empty> i32)))) -> [0i32: bb3, otherwise: bb4];
[01:10:16] }
[01:10:16] bb7: {
[01:10:16]     goto -> bb1;
[01:10:16] }
[01:10:16] bb8: {
[01:10:16]     goto -> bb4;
[01:10:16] }
[01:10:16] bb9: {
[01:10:16]     return;
[01:10:16] }
[01:10:16] bb10: {
[01:10:16]     resume;
[01:10:16] }
[01:10:16] Actual:
[01:10:16] fn match_guard(_1: std::option::Option<&&i32>) -> i32{
[01:10:16]     let mut _0: i32;
[01:10:16]     let mut _2: isize;
[01:10:16]     let mut _3: &'<empty> std::option::Option<&&i32>;
[01:10:16]     let mut _4: &'<empty> &'<empty> &'<empty> i32;
[01:10:16]     let mut _5: &'<empty> &'<empty> i32;
[01:10:16]     let mut _6: &'<empty> i32;
[01:10:16]     bb0: {                              
[01:10:16]         FakeRead(ForMatchedPlace, _1);
[01:10:16]         _2 = discriminant(_1);
[01:10:16]         _3 = &shallow _1;
[01:10:16]         _4 = &shallow ((_1 as Some).0: &'<empty> &'<empty> i32);
[01:10:16]         _5 = &shallow (*((_1 as Some).0: &'<empty> &'<empty> i32));
[01:10:16]         _6 = &shallow (*(*((_1 as Some).0: &'<empty> &'<empty> i32)));
[01:10:16]         switchInt(move _2) -> [1isize: bb6, otherwise: bb4];
[01:10:16]     }
[01:10:16]     bb1: {                              
[01:10:16]         _0 = const 0i32;
[01:10:16]         goto -> bb9;
[01:10:16]     }
[01:10:16]     bb2: {                              
[01:10:16]         _0 = const 1i32;
[01:10:16]         goto -> bb9;
[01:10:16]     }
[01:10:16]     bb3: {                              
[01:10:16]         FakeRead(ForMatchGuard, _3);
[01:10:16]         FakeRead(ForMatchGuard, _4);
[01:10:16]         FakeRead(ForMatchGuard, _5);
[01:10:16]         FakeRead(ForMatchGuard, _6);
[01:10:16]         goto -> bb7;
[01:10:16]     }
[01:10:16]     bb4: {                              
[01:10:16]         FakeRead(ForMatchGuard, _3);
[01:10:16]         FakeRead(ForMatchGuard, _4);
[01:10:16]         FakeRead(ForMatchGuard, _5);
[01:10:16]         FakeRead(ForMatchGuard, _6);
[01:10:16]         goto -> bb2;
[01:10:16]     }
[01:10:16]     bb5: {                              
[01:10:16]     }
[01:10:16]     }
[01:10:16]     bb6: {                              
[01:10:16]         switchInt((*(*((_1 as Some).0: &'<empty> &'<empty> i32)))) -> [0i32: bb3, otherwise: bb4];
[01:10:16]     }
[01:10:16]     bb7: {                              
[01:10:16]         goto -> bb1;
[01:10:16]     }
[01:10:16]     bb8: {                              
[01:10:16]         goto -> bb4;
[01:10:16]     }
[01:10:16]     bb9: {                              
[01:10:16]         return;
[01:10:16] }', tools/compiletest/src/runtest.rs:2893:13
[01:10:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:10:16] 
[01:10:16] 
[01:10:16] 
[01:10:16] failures:
[01:10:16]     [mir-opt] mir-opt/remove_fake_borrows.rs
[01:10:16] 
[01:10:16] test result: FAILED. 39 passed; 1 failed; 6 ignored; 0 measured; 0 filtered out
[01:10:16] 
[01:10:16] 
[01:10:16] 
[01:10:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "mir-opt" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:16] 
[01:10:16] 
[01:10:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/parse-fail src/test/mir-opt src/test/codegen-units src/libcore
[01:10:16] Build completed unsuccessfully in 1:06:38
---
travis_time:end:18707448:start=1537797054323369773,finish=1537797054328644283,duration=5274510
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1e513cd5
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06a5c8ca
travis_time:start:06a5c8ca
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01518870
$ dmesg | grep -i kill
