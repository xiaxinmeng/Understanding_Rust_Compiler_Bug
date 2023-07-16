plain
travis_time:end:00052f10:start=1554151212444418755,finish=1554151213351767797,duration=907349042
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:34:10]    Compiling rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
[01:34:28]    Compiling clippy v0.0.212 (/checkout/src/tools/clippy)
[01:34:35]     Finished release [optimized] target(s) in 3m 00s
[01:34:35]      Running build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/clippy_driver-bcc57f90f2beedb3
[01:34:35] /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/clippy_driver-bcc57f90f2beedb3: symbol lookup error: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblibtest-ec6093eac7e5c75c.so: undefined symbol: _ZN10rustc_term6stdout17h7477aa7f2f47c698E
[01:34:35] error: test failed, to rerun pass '--bin clippy-driver'
[01:34:35] 
[01:34:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/clippy/Cargo.toml" "--features" "rustc-workspace-hack/all-static"
[01:34:35] expected success, got: exit code: 127
[01:34:35] 
---
[01:47:36] Verifying status of rustfmt...
[01:47:36] Verifying status of clippy-driver...
[01:47:36] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:47:36] 
[01:47:36] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:47:36] 
[01:47:36] If you do intend to update 'clippy-driver', please check the error messages above and
[01:47:36] commit another update.
[01:47:36] 
[01:47:36] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:47:36] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:47:36] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:0843edf0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr  1 22:28:00 UTC 2019
---
travis_time:end:12140b50:start=1554157682026058171,finish=1554157682030618691,duration=4560520
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ba36170
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f77cdb4
travis_time:start:0f77cdb4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1f35f2c0
$ dmesg | grep -i kill
