plain
[00:53:20] 
[00:53:20] ---- [ui] ui/issues/issue-44415.rs stdout ----
[00:53:20] diff of stderr:
[00:53:20] 
[00:53:20] 10 LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
[00:53:20] 11    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:53:20] 12 note: ...which requires const-evaluating + checking `std::intrinsics::size_of`...
[00:53:20] -   --> $SRC_DIR/libcore/intrinsics.rs:LL:COL
[00:53:20] -    |
[00:53:20] - LL |     pub fn size_of<T>() -> usize;
[00:53:20] -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:53:20] 17 note: ...which requires const-evaluating + checking `std::intrinsics::size_of`...
[00:53:20] -   --> $SRC_DIR/libcore/intrinsics.rs:LL:COL
[00:53:20] -    |
[00:53:20] - LL |     pub fn size_of<T>() -> usize;
[00:53:20] -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:53:20] 22    = note: ...which requires computing layout of `Foo`...
[00:53:20] 23    = note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: [u8; _] }`...
[00:53:20] 24 note: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}#0`...
[00:53:20] 
[00:53:20] The actual stderr differed from the expected stderr.
[00:53:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44415/issue-44415.stderr
[00:53:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44415/issue-44415.stderr
[00:53:20] To update references, rerun the tests and pass the `--bless` flag
[00:53:20] To only update this specific test, also pass `--test-args issues/issue-44415.rs`
[00:53:20] error: 1 errors occurred comparing output.
[00:53:20] status: exit code: 1
[00:53:20] status: exit code: 1
[00:53:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-44415.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44415" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44415/auxiliary" "-A" "unused"
[00:53:20] ------------------------------------------
[00:53:20] 
[00:53:20] ------------------------------------------
[00:53:20] stderr:
[00:53:20] stderr:
[00:53:20] ------------------------------------------
[00:53:20] error[E0391]: cycle detected when const-evaluating + checking `Foo::bytes::{{constant}}#0`
[00:53:20]    |
[00:53:20]    |
[00:53:20] LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
[00:53:20]    |
[00:53:20]    |
[00:53:20] note: ...which requires const-evaluating `Foo::bytes::{{constant}}#0`...
[00:53:20]    |
[00:53:20]    |
[00:53:20] LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
[00:53:20]    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:53:20] note: ...which requires const-evaluating + checking `std::intrinsics::size_of`...
[00:53:20] note: ...which requires const-evaluating + checking `std::intrinsics::size_of`...
[00:53:20]    = note: ...which requires computing layout of `Foo`...
[00:53:20]    = note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: [u8; _] }`...
[00:53:20] note: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}#0`...
[00:53:20]    |
[00:53:20]    |
[00:53:20] LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
[00:53:20]    |                 ^^^^^^
[00:53:20]    = note: ...which again requires const-evaluating + checking `Foo::bytes::{{constant}}#0`, completing the cycle
[00:53:20] note: cycle used when processing `Foo`
[00:53:20]    |
[00:53:20] LL | struct Foo {
[00:53:20]    | ^^^^^^^^^^
[00:53:20] 
---
[00:53:20] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:53:20] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:53:20] 
[00:53:20] 
[00:53:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.37.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:53:20] 
[00:53:20] 
[00:53:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:53:20] Build completed unsuccessfully in 0:49:51
---
travis_time:end:03bb7ef0:start=1561112131572283512,finish=1561112131578122609,duration=5839097
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18e70f51
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:28e5457c
travis_time:start:28e5457c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0375be80
$ dmesg | grep -i kill
