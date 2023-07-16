plain
travis_time:end:0f5e86ba:start=1557280517335648013,finish=1557280603654443197,duration=86318795184
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:01:47]   Downloaded unicode-xid v0.1.0
[00:01:47]    Compiling proc-macro2 v0.4.24
[00:01:47]    Compiling unicode-xid v0.1.0
[00:01:47]    Compiling serde v1.0.82
[00:01:47]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[00:01:48]    Compiling unicode-width v0.1.5
[00:01:48]    Compiling ordermap v0.3.5
[00:01:48]    Compiling fixedbitset v0.1.9
[00:01:48]    Compiling itoa v0.4.3
---
[00:03:08]    Compiling syn v0.15.22
[00:03:14]    Compiling serde_json v1.0.33
[00:03:27]    Compiling serde_derive v1.0.81
[00:03:47]    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
[00:03:48] error: constant item is never used: `WHITELISTED_SOURCES`
[00:03:48]  --> src/tools/tidy/src/extdeps.rs:7:1
[00:03:48] 7 | / const WHITELISTED_SOURCES: &[&str] = &[
[00:03:48] 7 | / const WHITELISTED_SOURCES: &[&str] = &[
[00:03:48] 8 | |     "\"registry+https://github.com/rust-lang/crates.io-index\"",
[00:03:48] 9 | | ];
[00:03:48]   |
[00:03:48]   = note: `-D dead-code` implied by `-D warnings`
[00:03:48] 
[00:03:48] error: aborting due to previous error
[00:03:48] error: aborting due to previous error
[00:03:48] 
[00:03:48] error: Could not compile `tidy`.
[00:03:48] To learn more, run the command again with --verbose.
[00:03:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--message-format" "json"
[00:03:48] expected success, got: exit code: 101
[00:03:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:48] Build completed unsuccessfully in 0:00:54
[00:03:48] make: *** [tidy] Error 1
[00:03:48] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11b1a006
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May  8 02:00:41 UTC 2019
---
travis_time:end:0154e54c:start=1557280842055524805,finish=1557280842060728761,duration=5203956
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0254ced5
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1c54cdc0
travis_time:start:1c54cdc0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0137055c
$ dmesg | grep -i kill
