plain
[01:27:16] test [pretty] pretty/empty-impl.rs ... ok
[01:27:16] test [pretty] pretty/example1.rs ... ok
[01:27:16] test [pretty] pretty/example2.rs ... ok
[01:27:16] test [pretty] pretty/empty-lines.rs ... ok
[01:27:16] ERROR 2019-03-03T02:59:23Z: compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n// Check that `fn foo(x: i32, ...)` does not print as `fn foo(x: i32, ..., ...)`.\n// See issue #58853.\n\n// pp-exact\n#![feature(c_variadic)]\n\nextern \"C\" {\n    pub fn foo(x: i32, ...);\n}\n\npub unsafe extern \"C\" fn bar(_: i32, mut ap: ...) -> usize {\n    ap.arg::<usize>()\n}\n\nfn main() {}\n\n------------------------------------------\nactual:\n------------------------------------------\n// Check that `fn foo(x: i32, ...)` does not print as `fn foo(x: i32, ..., ...)`.\n// See issue #58853.\n\n// pp-exact\n#![feature(c_variadic)]\n\nextern \"C\" {\n    pub fn foo(x: i32, ...);\n}\n\npub unsafe extern \"C\" fn bar(_: i32, mut ap: ...) -> usize {\n    ap.arg::<usize>()\n}\n\nfn main() { }\n\n------------------------------------------\n\n"
[01:27:16] test [pretty] pretty/fn-return.rs ... ok
[01:27:16] test [pretty] pretty/fn-types.rs ... ok
[01:27:16] test [pretty] pretty/import-renames.rs ... ok
[01:27:16] test [pretty] pretty/for-comment.rs ... ok
---
[01:27:17] 
[01:27:17] error: pretty-printed source does not match expected source
[01:27:17] expected:
[01:27:17] ------------------------------------------
[01:27:17] // Check that `fn foo(x: i32, ...)` does not print as `fn foo(x: i32, ..., ...)`.
[01:27:17] // See issue #58853.
[01:27:17] 
[01:27:17] // pp-exact
[01:27:17] #![feature(c_variadic)]
[01:27:17] extern "C" {
[01:27:17]     pub fn foo(x: i32, ...);
[01:27:17] }
[01:27:17] 
[01:27:17] 
[01:27:17] pub unsafe extern "C" fn bar(_: i32, mut ap: ...) -> usize {
[01:27:17]     ap.arg::<usize>()
[01:27:17] }
[01:27:17] fn main() {}
[01:27:17] 
[01:27:17] ------------------------------------------
[01:27:17] actual:
[01:27:17] actual:
[01:27:17] ------------------------------------------
[01:27:17] // Check that `fn foo(x: i32, ...)` does not print as `fn foo(x: i32, ..., ...)`.
[01:27:17] // See issue #58853.
[01:27:17] 
[01:27:17] // pp-exact
[01:27:17] #![feature(c_variadic)]
[01:27:17] extern "C" {
[01:27:17]     pub fn foo(x: i32, ...);
[01:27:17] }
[01:27:17] 
[01:27:17] 
[01:27:17] pub unsafe extern "C" fn bar(_: i32, mut ap: ...) -> usize {
[01:27:17]     ap.arg::<usize>()
[01:27:17] }
[01:27:17] fn main() { }
[01:27:17] 
[01:27:17] ------------------------------------------
[01:27:17] 
[01:27:17] 
[01:27:17] 
[01:27:17] thread '[pretty] pretty/fn-variadic.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2040:9
[01:27:17] 
[01:27:17] 
[01:27:17] failures:
[01:27:17]     [pretty] pretty/fn-variadic.rs
[01:27:17]     [pretty] pretty/fn-variadic.rs
[01:27:17] 
[01:27:17] test result: FAILED. 52 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:27:17] 
[01:27:17] 
[01:27:17] 
[01:27:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/pretty" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "pretty" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:27:17] 
[01:27:17] 
[01:27:17] 
[01:27:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:27:17] Build completed unsuccessfully in 0:00:57
[01:27:17] Makefile:50: recipe for target 'check-aux' failed
[01:27:17] make: *** [check-aux] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0fe422bf
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar  3 02:59:25 UTC 2019
---
travis_time:end:10585105:start=1551581966658032182,finish=1551581966690589918,duration=32557736
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07fa2440
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:329aeca0
travis_time:start:329aeca0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0862b368
$ dmesg | grep -i kill
