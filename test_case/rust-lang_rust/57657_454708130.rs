plain
travis_time:end:0119a708:start=1547626650510324207,finish=1547626719641221140,duration=69130896933
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:58:31] .................................................................................................... 1500/5306
[00:58:33] ...........................................................................................i........ 1600/5306
[00:58:36] .................................................................i.................................. 1700/5306
[00:58:39] .................................................................................................... 1800/5306
[00:58:43] ......................................................................F............................. 1900/5306
[00:58:49] ..................i................................................................................. 2100/5306
[00:58:52] .................................................................................................... 2200/5306
[00:58:56] .................................................................................................... 2300/5306
[00:59:00] .................................................................................................... 2400/5306
---
[01:00:39] 1 error[E0669]: invalid value for constraint in inline assembly
[01:00:39] -   --> $DIR/issue-53787-inline-assembler-macro.rs:22:16
[01:00:39] +   --> $DIR/issue-53787-inline-assembler-macro.rs:21:16
[01:00:39] 3    |
[01:00:39] 4 LL |     fake_jump!("FirstFunc"); //~ ERROR invalid value for constraint in inline assembly
[01:00:39] 
[01:00:39] 
[01:00:39] The actual stderr differed from the expected stderr.
[01:00:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-53787-inline-assembler-macro/issue-53787-inline-assembler-macro.stderr
[01:00:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-53787-inline-assembler-macro/issue-53787-inline-assembler-macro.stderr
[01:00:39] To update references, rerun the tests and pass the `--bless` flag
[01:00:39] To only update this specific test, also pass `--test-args issue-53787-inline-assembler-macro.rs`
[01:00:39] error: 1 errors occurred comparing output.
[01:00:39] status: exit code: 1
[01:00:39] status: exit code: 1
[01:00:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-53787-inline-assembler-macro.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-53787-inline-assembler-macro/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-53787-inline-assembler-macro/auxiliary" "-A" "unused"
[01:00:39] ------------------------------------------
[01:00:39] 
[01:00:39] ------------------------------------------
[01:00:39] stderr:
[01:00:39] stderr:
[01:00:39] ------------------------------------------
[01:00:39] {"message":"invalid value for constraint in inline assembly","code":{"code":"E0669","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-53787-inline-assembler-macro.rs","byte_start":418,"byte_end":429,"line_start":21,"line_end":21,"column_start":16,"column_end":27,"is_primary":true,"text":[{"text":"    fake_jump!(\"FirstFunc\"); //~ ERROR invalid value for constraint in inline assembly","highlight_start":16,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0669]: invalid value for constraint in inline assembly\n  --> /checkout/src/test/ui/issue-53787-inline-assembler-macro.rs:21:16\n   |\nLL |     fake_jump!(\"FirstFunc\"); //~ ERROR invalid value for constraint in inline assembly\n   |                ^^^^^^^^^^^\n\n"}
[01:00:39] {"message":"For more information about this error, try `rustc --explain E0669`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0669`.\n"}
[01:00:39] 
[01:00:39] ------------------------------------------
[01:00:39] 
---
[01:00:39] 
[01:00:39] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:00:39] 
[01:00:39] 
[01:00:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:39] 
[01:00:39] 
[01:00:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:39] Build completed unsuccessfully in 0:03:52
[01:00:39] Build completed unsuccessfully in 0:03:52
[01:00:39] Makefile:48: recipe for target 'check' failed
[01:00:39] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:053a8d4a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jan 16 09:19:28 UTC 2019
---
travis_time:end:1d925a80:start=1547630369107076014,finish=1547630369111429458,duration=4353444
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00075a82
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "
