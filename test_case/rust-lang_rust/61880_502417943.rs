plain
[01:48:12] error: tests/compile-fail/null_pointer_deref.rs:2: expected error not found: invalid use of NULL pointer
[01:48:12] 
[01:48:12] error: 1 unexpected errors found, 1 expected errors not found
[01:48:12] status: exit code: 1
[01:48:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/null_pointer_deref.rs" "-L" "/tmp/compiletestSyF7iA" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestSyF7iA/null_pointer_deref.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestSyF7iA/null_pointer_deref.stage-id.aux" "-A" "unused"
[01:48:12]     Error {
[01:48:12]         line_num: 2,
[01:48:12]         kind: Some(
[01:48:12]             Error,
[01:48:12]             Error,
[01:48:12]         ),
[01:48:12]         msg: "2:27: 2:44: Miri evaluation error: a memory access tried to interpret some bytes as a pointer [E0080]",
[01:48:12] ]
[01:48:12] 
[01:48:12] not found errors (from test file): [
[01:48:12]     Error {
---
[01:48:13] error: tests/compile-fail/null_pointer_write.rs:2: expected error not found: invalid use of NULL pointer
[01:48:13] 
[01:48:13] error: 1 unexpected errors found, 1 expected errors not found
[01:48:13] status: exit code: 1
[01:48:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/null_pointer_write.rs" "-L" "/tmp/compiletestSyF7iA" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestSyF7iA/null_pointer_write.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestSyF7iA/null_pointer_write.stage-id.aux" "-A" "unused"
[01:48:13]     Error {
[01:48:13]         line_num: 2,
[01:48:13]         kind: Some(
[01:48:13]             Error,
[01:48:13]             Error,
[01:48:13]         ),
[01:48:13]         msg: "2:14: 2:42: Miri evaluation error: a memory access tried to interpret some bytes as a pointer [E0080]",
[01:48:13] ]
[01:48:13] 
[01:48:13] not found errors (from test file): [
[01:48:13]     Error {
---
[01:48:19] This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
[01:48:19] 
[01:48:19] ⚠️ We detected that this PR updated 'miri', but its tests failed.
[01:48:19] 
[01:48:19] If you do intend to update 'miri', please check the error messages above and
[01:48:19] commit another update.
[01:48:19] 
[01:48:19] If you do NOT intend to update 'miri', please ensure you did not accidentally
[01:48:19] change the submodule at 'src/tools/miri'. You may ask your reviewer for the
[01:48:19] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:11b829c6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jun 16 04:00:14 UTC 2019
---
travis_time:end:0569cf4f:start=1560657616534001509,finish=1560657616541343882,duration=7342373
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09528306
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:22530e10
travis_time:start:22530e10
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:21afa058
$ dmesg | grep -i kill
