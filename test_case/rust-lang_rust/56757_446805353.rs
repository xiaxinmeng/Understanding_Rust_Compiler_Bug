plain
[01:13:02] +For more information about this error, try `rustc --explain E0658`.
[01:13:02] +
[01:13:02] 
[01:13:02] The actual stderr differed from the expected stderr.
[01:13:02] Actual stderr saved to /tmp/compiletestxCqJFh/regions-mock-trans.stderr
[01:13:02] To update references, run this command from build directory:
[01:13:02] tests/run-pass/update-references.sh '/tmp/compiletestxCqJFh' 'regions-mock-trans.rs'
[01:13:02] error: 1 errors occurred comparing output.
[01:13:02] status: exit code: 1
[01:13:02] status: exit code: 1
[01:13:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-mock-trans.rs" "-L" "/tmp/compiletestxCqJFh" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestxCqJFh/regions-mock-trans.stage-id" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestxCqJFh/regions-mock-trans.stage-id.aux" "-A" "unused"
[01:13:02] ------------------------------------------
[01:13:02] 
[01:13:02] ------------------------------------------
[01:13:02] stderr:
---
[01:13:02] +For more information about this error, try `rustc --explain E0658`.
[01:13:02] +
[01:13:02] 
[01:13:02] The actual stderr differed from the expected stderr.
[01:13:02] Actual stderr saved to /tmp/compiletestxCqJFh/thread-local.stderr
[01:13:02] To update references, run this command from build directory:
[01:13:02] tests/run-pass/update-references.sh '/tmp/compiletestxCqJFh' 'thread-local.rs'
[01:13:02] error: 1 errors occurred comparing output.
[01:13:02] status: exit code: 1
[01:13:02] status: exit code: 1
[01:13:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/thread-local.rs" "-L" "/tmp/compiletestxCqJFh" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestxCqJFh/thread-local.stage-id" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestxCqJFh/thread-local.stage-id.aux" "-A" "unused"
[01:13:02] ------------------------------------------
[01:13:02] 
[01:13:02] ------------------------------------------
[01:13:02] stderr:
---
[01:13:02] Verifying status of rustfmt...
[01:13:02] Verifying status of clippy-driver...
[01:13:02] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:13:02] 
[01:13:02] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:13:02] 
[01:13:02] If you do intend to update 'clippy-driver', please check the error messages above and
[01:13:02] commit another update.
[01:13:02] 
[01:13:02] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:13:02] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:13:02] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:140fb5d1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Dec 13 01:11:25 UTC 2018
---
travis_time:end:05de8f8d:start=1544663486857619001,finish=1544663486864984707,duration=7365706
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2b5d4fbe
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01249ab0
travis_time:start:01249ab0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06c0aedf
$ dmesg | grep -i kill
