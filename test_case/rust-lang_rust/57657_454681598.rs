plain
travis_time:end:05afcdf4:start=1547620315679502085,finish=1547620418282802265,duration=102603300180
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:03:19] .................................................................................................... 1500/5306
[01:03:22] ...........................................................................................i........ 1600/5306
[01:03:25] .................................................................i.................................. 1700/5306
[01:03:28] .................................................................................................... 1800/5306
[01:03:32] ......................................................................F............................. 1900/5306
[01:03:39] ..................i................................................................................. 2100/5306
[01:03:43] .................................................................................................... 2200/5306
[01:03:47] .................................................................................................... 2300/5306
[01:03:51] .................................................................................................... 2400/5306
---
[01:05:37] 1 error[E0669]: invalid value for constraint in inline assembly
[01:05:37] -   --> $DIR/issue-53787-inline-assembler-macro.rs:22:16
[01:05:37] +   --> $DIR/issue-53787-inline-assembler-macro.rs:21:16
[01:05:37] 3    |
[01:05:37] 4 LL |     fake_jump!("FirstFunc"); //~ ERROR invalid value for constraint in inline assembly
[01:05:37] 
[01:05:37] 
[01:05:37] The actual stderr differed from the expected stderr.
[01:05:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-53787-inline-assembler-macro/issue-53787-inline-assembler-macro.stderr
[01:05:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-53787-inline-assembler-macro/issue-53787-inline-assembler-macro.stderr
[01:05:37] To update references, rerun the tests and pass the `--bless` flag
[01:05:37] To only update this specific test, also pass `--test-args issue-53787-inline-assembler-macro.rs`
[01:05:37] error: 1 errors occurred comparing output.
[01:05:37] status: exit code: 1
[01:05:37] status: exit code: 1
[01:05:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-53787-inline-assembler-macro.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-53787-inline-assembler-macro/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-53787-inline-assembler-macro/auxiliary" "-A" "unused"
[01:05:37] ------------------------------------------
[01:05:37] 
[01:05:37] ------------------------------------------
[01:05:37] stderr:
[01:05:37] stderr:
[01:05:37] ------------------------------------------
[01:05:37] {"message":"invalid value for constraint in inline assembly","code":{"code":"E0669","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-53787-inline-assembler-macro.rs","byte_start":418,"byte_end":429,"line_start":21,"line_end":21,"column_start":16,"column_end":27,"is_primary":true,"text":[{"text":"    fake_jump!(\"FirstFunc\"); //~ ERROR invalid value for constraint in inline assembly","highlight_start":16,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0669]: invalid value for constraint in inline assembly\n  --> /checkout/src/test/ui/issue-53787-inline-assembler-macro.rs:21:16\n   |\nLL |     fake_jump!(\"FirstFunc\"); //~ ERROR invalid value for constraint in inline assembly\n   |                ^^^^^^^^^^^\n\n"}
[01:05:37] {"message":"For more information about this error, try `rustc --explain E0669`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0669`.\n"}
[01:05:37] 
[01:05:37] ------------------------------------------
[01:05:37] 
---
[01:05:37] 
[01:05:37] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:05:37] 
[01:05:37] 
[01:05:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:37] 
[01:05:37] 
[01:05:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:37] Build completed unsuccessfully in 0:04:08
[01:05:37] Build completed unsuccessfully in 0:04:08
[01:05:37] make: *** [check] Error 1
[01:05:37] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:202e9a50
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jan 16 07:39:25 UTC 2019
---
travis_time:end:1eea5dda:start=1547624366248880189,finish=1547624366253612271,duration=4732082
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b6d9cb4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:069f4b5f
travis_time:start:069f4b5f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08ae41dd
$ dmesg | grep -i kill
