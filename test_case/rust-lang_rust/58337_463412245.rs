plain
travis_time:end:1ae144b0:start=1550092937833812092,finish=1550092938743301613,duration=909489521
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-tools
---
[00:05:37]   Downloaded rustfmt-nightly v1.0.2
[00:05:37]   Downloaded new_debug_unreachable v1.0.1
[00:05:37]   Downloaded macro-utils v0.1.2
[00:05:37]   Downloaded block-buffer v0.3.3
[00:05:37]   Downloaded argon2rs v0.2.5
[00:05:37]   Downloaded unicode-normalization v0.1.7
[00:05:37]   Downloaded bit-set v0.5.0
[00:05:37]   Downloaded unicode-bidi v0.3.4
[00:05:37]   Downloaded atty v0.2.11
---
[00:05:40]   Downloaded wincolor v1.0.1
[00:05:40]   Downloaded open v1.2.1
[00:05:40]   Downloaded rls-data v0.18.2
[00:05:40]   Downloaded backtrace-sys v0.1.27
[00:05:40]   Downloaded constant_time_eq v0.1.3
[00:05:40]   Downloaded ena v0.11.0
[00:05:40]   Downloaded pest_generator v2.1.0
[00:05:41]   Downloaded crypto-hash v0.3.1
[00:05:41]   Downloaded string_cache v0.7.3
---
[00:05:44]   Downloaded rustc_version v0.2.3
[00:05:44]   Downloaded rls-vfs v0.7.0
[00:05:44]   Downloaded bufstream v0.1.4
[00:05:44]   Downloaded slab v0.4.2
[00:05:44]   Downloaded blake2-rfc v0.2.18
[00:05:44]   Downloaded rls-rustc v0.5.0
[00:05:44]   Downloaded parking_lot v0.6.4
[00:05:44]   Downloaded utf8-ranges v1.0.2
[00:05:44]   Downloaded cloudabi v0.0.3
---
[01:27:17] +For more information about this error, try `rustc --explain E0080`.
[01:27:17] +
[01:27:17] 
[01:27:17] The actual stderr differed from the expected stderr.
[01:27:17] Actual stderr saved to /tmp/compiletestrEB8cT/function_pointers.stderr
[01:27:17] To update references, run this command from build directory:
[01:27:17] tests/run-pass/update-references.sh '/tmp/compiletestrEB8cT' 'function_pointers.rs'
[01:27:17] error: 1 errors occurred comparing output.
[01:27:17] status: exit code: 1
[01:27:17] status: exit code: 1
[01:27:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/function_pointers.rs" "-L" "/tmp/compiletestrEB8cT" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestrEB8cT/function_pointers.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestrEB8cT/function_pointers.stage-id.aux" "-A" "unused"
[01:27:17] ------------------------------------------
[01:27:17] 
[01:27:17] ------------------------------------------
[01:27:17] stderr:
---
[01:27:17] Verifying status of rust-by-example...
[01:27:17] Verifying status of rls...
[01:27:17] This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
[01:27:17] 
[01:27:17] ⚠️ We detected that this PR updated 'rls', but its tests failed.
[01:27:17] 
[01:27:17] If you do intend to update 'rls', please check the error messages above and
[01:27:17] commit another update.
[01:27:17] 
[01:27:17] If you do NOT intend to update 'rls', please ensure you did not accidentally
[01:27:17] change the submodule at 'src/tools/rls'. You may ask your reviewer for the
[01:27:17] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:124987f4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 13 22:49:50 UTC 2019
---
travis_time:end:352049e0:start=1550098192220351068,finish=1550098192232528567,duration=12177499
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2d8dc610
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:028ca7c8
travis_time:start:028ca7c8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:005df471
$ dmesg | grep -i kill
