plain
travis_time:end:1dab1a26:start=1548836873920701014,finish=1548836875521684483,duration=1600983469
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-tools
---
[01:34:26] +For more information about this error, try `rustc --explain E0599`.
[01:34:26] +
[01:34:26] 
[01:34:26] The actual stderr differed from the expected stderr.
[01:34:26] Actual stderr saved to /tmp/compiletestD8PbKF/generator.stderr
[01:34:26] To update references, run this command from build directory:
[01:34:26] tests/run-pass/update-references.sh '/tmp/compiletestD8PbKF' 'generator.rs'
[01:34:26] error: 1 errors occurred comparing output.
[01:34:26] status: exit code: 1
[01:34:26] status: exit code: 1
[01:34:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/generator.rs" "-L" "/tmp/compiletestD8PbKF" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestD8PbKF/generator.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestD8PbKF/generator.stage-id.aux" "-A" "unused"
[01:34:26] ------------------------------------------
[01:34:26] 
[01:34:26] ------------------------------------------
[01:34:26] stderr:
---
[01:34:26] Verifying status of clippy-driver...
[01:34:26] Verifying status of miri...
[01:34:26] This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
[01:34:26] 
[01:34:26] ⚠️ We detected that this PR updated 'miri', but its tests failed.
[01:34:26] 
[01:34:26] If you do intend to update 'miri', please check the error messages above and
[01:34:26] commit another update.
[01:34:26] 
[01:34:26] If you do NOT intend to update 'miri', please ensure you did not accidentally
[01:34:26] change the submodule at 'src/tools/miri'. You may ask your reviewer for the
[01:34:26] proper steps.
travis_time:end:14f2f388:start=1548836886433793174,finish=1548842553032863319,duration=5666599070145
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:08a49c20
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1644dc6c:start=1548842554365103496,finish=1548842554369881338,duration=4777842
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01c4a838
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:007ab950
travis_time:start:007ab950
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:33bd639e
$ dmesg | grep -i kill
