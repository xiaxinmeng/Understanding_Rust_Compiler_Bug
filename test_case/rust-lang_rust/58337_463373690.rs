plain
travis_time:end:1baeb5fa:start=1550085801047903504,finish=1550085892411712199,duration=91363808695
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-tools
---
[00:02:08]   Downloaded log_settings v0.1.2
[00:02:08]   Downloaded chalk-engine v0.9.0
[00:02:08]   Downloaded phf v0.7.22
[00:02:08]   Downloaded rand_isaac v0.1.1
[00:02:08]   Downloaded argon2rs v0.2.5
[00:02:08]   Downloaded strum_macros v0.11.0
[00:02:08]   Downloaded rustc-rayon-core v0.1.1
[00:02:08]   Downloaded cargo_metadata v0.6.2
[00:02:08]   Downloaded log v0.4.6
---
[00:02:10]   Downloaded curl v0.4.19
[00:02:10]   Downloaded miniz_oxide v0.2.0
[00:02:10]   Downloaded phf_generator v0.7.22
[00:02:10]   Downloaded rand_core v0.2.2
[00:02:10]   Downloaded blake2-rfc v0.2.18
[00:02:10]   Downloaded itertools v0.8.0
[00:02:10]   Downloaded mdbook v0.2.3
[00:02:10]   Downloaded mio-named-pipes v0.1.6
[00:02:10]   Downloaded semver-parser v0.7.0
---
[00:02:11]   Downloaded tokio-current-thread v0.1.4
[00:02:11]   Downloaded rusty-fork v0.2.1
[00:02:11]   Downloaded pkg-config v0.3.14
[00:02:11]   Downloaded yaml-rust v0.3.5
[00:02:11]   Downloaded constant_time_eq v0.1.3
[00:02:11]   Downloaded futf v0.1.4
[00:02:11]   Downloaded toml-query v0.7.0
[00:02:11]   Downloaded futures v0.1.21
[00:02:11]   Downloaded pretty_assertions v0.5.1
---
[01:20:24] +For more information about this error, try `rustc --explain E0080`.
[01:20:24] +
[01:20:24] 
[01:20:24] The actual stderr differed from the expected stderr.
[01:20:24] Actual stderr saved to /tmp/compiletestyV5hCm/function_pointers.stderr
[01:20:24] To update references, run this command from build directory:
[01:20:24] tests/run-pass/update-references.sh '/tmp/compiletestyV5hCm' 'function_pointers.rs'
[01:20:24] error: 1 errors occurred comparing output.
[01:20:24] status: exit code: 1
[01:20:24] status: exit code: 1
[01:20:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/function_pointers.rs" "-L" "/tmp/compiletestyV5hCm" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestyV5hCm/function_pointers.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestyV5hCm/function_pointers.stage-id.aux" "-A" "unused"
[01:20:24] ------------------------------------------
[01:20:24] 
[01:20:24] ------------------------------------------
[01:20:24] stderr:
---
[01:20:24] Verifying status of rust-by-example...
[01:20:24] Verifying status of rls...
[01:20:24] This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
[01:20:24] 
[01:20:24] ⚠️ We detected that this PR updated 'rls', but its tests failed.
[01:20:24] 
[01:20:24] If you do intend to update 'rls', please check the error messages above and
[01:20:24] commit another update.
[01:20:24] 
[01:20:24] If you do NOT intend to update 'rls', please ensure you did not accidentally
[01:20:24] change the submodule at 'src/tools/rls'. You may ask your reviewer for the
[01:20:24] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:00cf9952
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 13 20:45:25 UTC 2019
---
travis_time:end:0a7f31ba:start=1550090726265737334,finish=1550090726270452418,duration=4715084
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13856273
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b6f4342
travis_time:start:0b6f4342
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03df3329
$ dmesg | grep -i kill
