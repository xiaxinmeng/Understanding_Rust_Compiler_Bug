plain
[00:52:30] ....................................................................................................
[00:52:33] .....................................................i..............................................
[00:52:36] ....................................................................................................
[00:52:39] ....................................................................................................
[00:52:41] .iiiiiiiii..........................................................................................
[00:52:47] ....................................................................................................
[00:52:50] .....................................................................................i..............
[00:52:53] ....................................................................................................
[00:52:56] ........................................i.i..ii.....................................................
---
[01:24:20] 
[01:24:20] running 6 tests
aring output.
[01:24:22] status: exit code: 101
[01:24:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:24:22] ------------------------------------------
[01:24:22] 
[01:24:22] running 2 tests
[01:24:22] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26) ... FAILED
[01:24:22] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26) ... FAILED
[01:24:22] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 20) ... FAILED
[01:24:22] failures:
[01:24:22] 
[01:24:22] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26) stdout ----
[01:24:22] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26) stdout ----
[01:24:22] error[E0425]: cannot find value `no` in this scope
[01:24:22]   |
[01:24:22] 3 | no
[01:24:22]   | ^^ not found in this scope
[01:24:22] 
[01:24:22] 
[01:24:22] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26)' panicked at 'couldn't compile the test', librustdoc/test.rs:344:13
[01:24:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:24:22] 
[01:24:22] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 20) stdout ----
[01:24:22] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 20)' panicked at 'test executable failed:
[01:24:22] 
[01:24:22] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:24:22] 
[01:24:22] ', librustdoc/test.rs:379:17
[01:24:22] 
[01:24:22] 
[01:24:22] 
[01:24:22] failures:
[01:24:22]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26)
[01:24:22]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 20)
[01:24:22] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:24:22] 
[01:24:22] 
[01:24:22] ------------------------------------------
---
[01:24:22] test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:24:22] 
[01:24:22] 
[01:24:22] 
[01:24:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-g; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0352bf40
travis_time:start:0352bf40
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6

