plain
travis_time:end:06c43a9a:start=1561212341109231978,finish=1561212343291519974,duration=2182287996
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:34] 
[01:07:34] running 9 tests
[01:07:34] iiiiiiiii
[01:07:34] 
[01:07:34]  finished in 0.156
[01:07:34] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:51] 
[01:07:51] running 122 tests
[01:08:17] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:08:22] .i.i......iii.i.....ii
[01:08:22] 
[01:08:22]  finished in 31.387
[01:08:22] travis_fold:end:test_debuginfo

---
[01:08:23] 
[01:08:23] running 24 tests
[01:08:30] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:08:30] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:08:30] ..F.....................
[01:08:30] 
[01:08:30] ---- [ui] ui-fulldeps/deprecated-derive.rs stdout ----
[01:08:30] diff of stderr:
[01:08:30] 
[01:08:30] 
[01:08:30] - warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[01:08:30] + warning: use of deprecated item 'Encodable': derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[01:08:30] 3    |
[01:08:30] 3    |
[01:08:30] 4 LL | #[derive(Encodable)]
[01:08:30] 5    |          ^^^^^^^^^
[01:08:30] +    |
[01:08:30] +    = note: #[warn(deprecated)] on by default
[01:08:30] 6 
[01:08:30] 6 
[01:08:30] 7 
[01:08:30] 
[01:08:30] 
[01:08:30] The actual stderr differed from the expected stderr.
[01:08:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/deprecated-derive/deprecated-derive.stderr
[01:08:30] To update references, rerun the tests and pass the `--bless` flag
[01:08:30] To only update this specific test, also pass `--test-args deprecated-derive.rs`
[01:08:30] error: 1 errors occurred comparing output.
[01:08:30] status: exit code: 0
[01:08:30] status: exit code: 0
[01:08:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/deprecated-derive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/deprecated-derive/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/deprecated-derive/auxiliary" "-A" "unused"
[01:08:30] ------------------------------------------
[01:08:30] 
[01:08:30] ------------------------------------------
[01:08:30] stderr:
[01:08:30] stderr:
[01:08:30] ------------------------------------------
[01:08:30] warning: use of deprecated item 'Encodable': derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[01:08:30]    |
[01:08:30]    |
[01:08:30] LL | #[derive(Encodable)]
[01:08:30]    |
[01:08:30]    = note: #[warn(deprecated)] on by default
[01:08:30] 
[01:08:30] 
---
[01:08:30] test result: FAILED. 23 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:08:30] 
[01:08:30] 
[01:08:30] 
[01:08:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:30] 
[01:08:30] 
[01:08:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:30] Build completed unsuccessfully in 1:03:34
---
travis_time:end:0378c49d:start=1561216465837204983,finish=1561216465842431365,duration=5226382
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0cab7180
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:126e984c
travis_time:start:126e984c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:097e989e
$ dmesg | grep -i kill
