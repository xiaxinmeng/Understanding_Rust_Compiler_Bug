plain
[01:22:19] failures:
[01:22:19] 
[01:22:19] ---- [mir-opt] mir-opt/generator-storage-dead-unwind.rs stdout ----
[01:22:19] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:22:19] thread '[mir-opt] mir-opt/generator-storage-dead-unwind.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:22:19] Expected Line: "    _1 = suspend(move _5) -> [resume: bb2, drop: bb4];"
[01:22:19] Test Name: rustc.main-{{closure}}.StateTransform.before.mir
[01:22:19] ... (elided)
[01:22:19] ... (elided)
[01:22:19] let _2: Foo;
[01:22:19] ... (elided)
[01:22:19] ... (elided)
[01:22:19] let mut _7: Foo;
[01:22:19] ... (elided)
[01:22:19] let mut _9: Bar;
[01:22:19] scope 1 {
[01:22:19]     let _3: Bar;
[01:22:19]     scope 2 {
[01:22:19]     }
[01:22:19] }
[01:22:19] bb0: {
[01:22:19]     StorageLive(_2);
[01:22:19]     _2 = Foo(const 5i32,);
[01:22:19]     StorageLive(_3);
[01:22:19]     _3 = Bar(const 6i32,);
[01:22:19] ... (elided)
[01:22:19]     _1 = suspend(move _5) -> [resume: bb2, drop: bb4];
[01:22:19] }
[01:22:19] bb1 (cleanup): {
[01:22:19]     resume;
[01:22:19] }
[01:22:19] bb2: {
[01:22:19] ... (elided)
[01:22:19]     StorageLive(_7);
[01:22:19]     _7 = move _2;
[01:22:19]     _6 = const take::<Foo>(move _7) -> [return: bb9, unwind: bb8];
[01:22:19] }
[01:22:19] bb3 (cleanup): {
[01:22:19]     StorageDead(_2);
[01:22:19]     drop(_1) -> bb1;
[01:22:19] }
[01:22:19] bb4: {
[01:22:19] ... (elided)
[01:22:19]     StorageDead(_3);
[01:22:19]     drop(_2) -> [return: bb5, unwind: bb3];
[01:22:19] }
[01:22:19] bb5: {
[01:22:19]     StorageDead(_2);
[01:22:19]     drop(_1) -> [return: bb6, unwind: bb1];
[01:22:19] }
[01:22:19] bb6: {
[01:22:19]     generator_drop;
[01:22:19] }
[01:22:19] bb7 (cleanup): {
[01:22:19]     StorageDead(_3);
[01:22:19]     StorageDead(_2);
[01:22:19]     drop(_1) -> bb1;
[01:22:19] }
[01:22:19] bb8 (cleanup): {
[01:22:19]     StorageDead(_7);
[01:22:19]     goto -> bb7;
[01:22:19] }
[01:22:19] bb9: {
[01:22:19]     StorageDead(_7);
[01:22:19]     StorageLive(_9);
[01:22:19]     _9 = move _3;
[01:22:19]     _8 = const take::<Bar>(move _9) -> [return: bb10, unwind: bb11];
[01:22:19] }
[01:22:19] bb10: {
[01:22:19]     StorageDead(_9);
[01:22:19] ... (elided)
[01:22:19]     StorageDead(_3);
[01:22:19]     StorageDead(_2);
[01:22:19]     drop(_1) -> [return: bb12, unwind: bb1];
[01:22:19] }
[01:22:19] bb11 (cleanup): {
[01:22:19]     StorageDead(_9);
[01:22:19]     goto -> bb7;
[01:22:19] }
[01:22:19] bb12: {
[01:22:19]     return;
[01:22:19] }
[01:22:19] Actual:
[01:22:19] fn  main::{{closure}}#0(_1: [generator@/checkout/src/test/mir-opt/generator-storage-dead-unwind.rs:19:16: 25:6 {Foo, Bar, ()}]) -> ()
[01:22:19] yields ()
[01:22:19]  {
[01:22:19]     let mut _0: ();
[01:22:19]     let _2: Foo;
[01:22:19]     let mut _4: ();
[01:22:19]     let mut _5: ();
[01:22:19]     let mut _6: ();
[01:22:19]     let mut _7: Foo;
[01:22:19]     let mut _8: ();
[01:22:19]     let mut _9: Bar;
[01:22:19]     scope 1 {
[01:22:19]         let _3: Bar;
[01:22:19]         scope 2 {
[01:22:19]     }
[01:22:19]     bb0: {
[01:22:19]     bb0: {
[01:22:19]         StorageLive(_2);
[01:22:19]         _2 = Foo(const 5i32,);
[01:22:19]         StorageLive(_3);
[01:22:19]         _3 = Bar(const 6i32,);
[01:22:19]         StorageLive(_5);
[01:22:19]         _5 = ();
[01:22:19]         _1 = suspend(move _5) -> [resume: bb1, drop: bb2];
[01:22:19]     bb1: {
[01:22:19]         _4 = ();
[01:22:19]         _4 = ();
[01:22:19]         StorageDead(_5);
[01:22:19]         StorageLive(_7);
[01:22:19]         _7 = move _2;
[01:22:19]         _6 = const take::<Foo>(move _7) -> bb5;
[01:22:19]     bb2: {
[01:22:19]     bb2: {
[01:22:19]         StorageDead(_5);
[01:22:19]         StorageDead(_3);
[01:22:19]         drop(_2) -> bb3;
[01:22:19]     bb3: {
[01:22:19]     bb3: {
[01:22:19]         StorageDead(_2);
[01:22:19]         drop(_1) -> bb4;
[01:22:19]     bb4: {
[01:22:19]         generator_drop;
[01:22:19]     }
[01:22:19]     bb5: {
[01:22:19]     bb5: {
[01:22:19]         StorageDead(_7);
[01:22:19]         StorageLive(_9);
[01:22:19]         _9 = move _3;
[01:22:19]         _8 = const take::<Bar>(move _9) -> bb6;
[01:22:19]     bb6: {
[01:22:19]     bb6: {
[01:22:19]         StorageDead(_9);
[01:22:19]         _0 = ();
[01:22:19]         StorageDead(_3);
[01:22:19]         StorageDead(_2);
[01:22:19]         drop(_1) -> bb7;
[01:22:19]     bb7: {
[01:22:19]         return;
[01:22:19]     }
[01:22:19]     }
[01:22:19]     bb8 (cleanup): {
[01:22:19]         resume;
[01:22:19] }', src/tools/compiletest/src/runtest.rs:3196:13
[01:22:19] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:22:19] 
[01:22:19] 
[01:22:19] 
[01:22:19] failures:
[01:22:19]     [mir-opt] mir-opt/generator-storage-dead-unwind.rs
[01:22:19] 
[01:22:19] test result: FAILED. 44 passed; 1 failed; 6 ignored; 0 measured; 0 filtered out
[01:22:19] 
[01:22:19] 
[01:22:19] 
[01:22:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "mir-opt" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.37.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:22:19] 
[01:22:19] 
[01:22:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
[01:22:19] Build completed unsuccessfully in 1:11:27
---
travis_time:end:1e9b975a:start=1559615223197757109,finish=1559615223207392949,duration=9635840
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10633660
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0966c898
travis_time:start:0966c898
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:005e47a5
$ dmesg | grep -i kill
