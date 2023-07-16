plain
travis_time:end:036aa8a0:start=1561552896655659899,finish=1561552898843198151,duration=2187538252
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:57:31] 
[00:57:31] ---- [ui] ui/init-unsafe.rs stdout ----
[00:57:31] diff of stderr:
[00:57:31] 
[00:57:31] + warning: use of deprecated item 'std::intrinsics::init': no longer used by rustc, will be removed - use MaybeUnint instead
[00:57:31] +    |
[00:57:31] + LL | use std::intrinsics::{init};
[00:57:31] +    |                       ^^^^
[00:57:31] +    |
[00:57:31] +    |
[00:57:31] +    = note: #[warn(deprecated)] on by default
[00:57:31] + 
[00:57:31] + warning: use of deprecated item 'std::intrinsics::init': no longer used by rustc, will be removed - use MaybeUnint instead
[00:57:31] +    |
[00:57:31] +    |
[00:57:31] + LL |     let stuff = init::<isize>();
[00:57:31] + 
[00:57:31] 1 error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
[00:57:31] 2   --> $DIR/init-unsafe.rs:7:17
[00:57:31] 3    |
[00:57:31] 3    |
[00:57:31] 
[00:57:31] 
[00:57:31] The actual stderr differed from the expected stderr.
[00:57:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/init-unsafe/init-unsafe.stderr
[00:57:31] To update references, rerun the tests and pass the `--bless` flag
[00:57:31] To only update this specific test, also pass `--test-args init-unsafe.rs`
[00:57:31] error: 1 errors occurred comparing output.
[00:57:31] status: exit code: 1
[00:57:31] status: exit code: 1
[00:57:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/init-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/init-unsafe" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/init-unsafe/auxiliary" "-A" "unused"
[00:57:31] ------------------------------------------
[00:57:31] 
[00:57:31] ------------------------------------------
[00:57:31] stderr:
[00:57:31] stderr:
[00:57:31] ------------------------------------------
[00:57:31] warning: use of deprecated item 'std::intrinsics::init': no longer used by rustc, will be removed - use MaybeUnint instead
[00:57:31]    |
[00:57:31] LL | use std::intrinsics::{init};
[00:57:31]    |                       ^^^^
[00:57:31]    |
[00:57:31]    |
[00:57:31]    = note: #[warn(deprecated)] on by default
[00:57:31] 
[00:57:31] warning: use of deprecated item 'std::intrinsics::init': no longer used by rustc, will be removed - use MaybeUnint instead
[00:57:31]    |
[00:57:31]    |
[00:57:31] LL |     let stuff = init::<isize>(); //~ ERROR call to unsafe function is unsafe
[00:57:31] 
[00:57:31] error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
[00:57:31]   --> /checkout/src/test/ui/init-unsafe.rs:7:17
[00:57:31]    |
[00:57:31]    |
[00:57:31] LL |     let stuff = init::<isize>(); //~ ERROR call to unsafe function is unsafe
[00:57:31]    |                 ^^^^^^^^^^^^^^^ call to unsafe function
[00:57:31]    |
[00:57:31]    = note: consult the function's documentation for information on how to avoid undefined behavior
[00:57:31] error: aborting due to previous error
[00:57:31] 
[00:57:31] For more information about this error, try `rustc --explain E0133`.
[00:57:31] 
---
[00:57:31] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:57:31] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:57:31] 
[00:57:31] 
[00:57:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:31] 
[00:57:31] 
[00:57:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:31] Build completed unsuccessfully in 0:52:45
---
travis_time:end:099e4630:start=1561556362959203865,finish=1561556362963903619,duration=4699754
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fcb578c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:241f6aea
travis_time:start:241f6aea
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02c4dc68
$ dmesg | grep -i kill
