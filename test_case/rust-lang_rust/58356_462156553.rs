plain
travis_time:end:0770cd5c:start=1549819690825389191,finish=1549819691749055397,duration=923666206
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:25]   Downloaded new_debug_unreachable v1.0.1
[00:03:25]   Downloaded env_logger v0.5.13
[00:03:25]   Downloaded utf-8 v0.7.2
[00:03:25]   Downloaded rustc-ap-rustc_cratesio_shim v306.0.0
[00:03:25]   Downloaded constant_time_eq v0.1.3
[00:03:25]   Downloaded rand_xorshift v0.1.0
[00:03:25]   Downloaded backtrace v0.3.11
[00:03:25]   Downloaded mdbook v0.1.7
[00:03:25]   Downloaded bytesize v1.0.0
---
[00:03:29]   Downloaded libssh2-sys v0.2.11
[00:03:29]   Downloaded rustc-ap-graphviz v366.0.0
[00:03:29]   Downloaded regex v0.2.11
[00:03:29]   Downloaded crossbeam-utils v0.5.0
[00:03:29]   Downloaded blake2-rfc v0.2.18
[00:03:29]   Downloaded rustc-rayon-core v0.1.1
[00:03:29]   Downloaded rustc-ap-syntax v366.0.0
[00:03:29]   Downloaded flate2 v1.0.6
[00:03:29]   Downloaded precomputed-hash v0.1.1
[00:03:29]   Downloaded precomputed-hash v0.1.1
[00:03:29]   Downloaded mio-uds v0.6.7
[00:03:29]   Downloaded bytecount v0.5.1
[00:03:29]   Downloaded argon2rs v0.2.5
[00:03:31] Build completed successfully in 0:01:30
[00:03:31] travis_fold:end:make-prepare

[00:03:31] travis_time:end:0fe8c065:start=1549819823079196866,finish=1549819913856721674,duration=90777524808
---
tidy check
[00:04:16] * 565 error codes
[00:04:16] * highest error code: E0722
[00:04:16] * 250 features
[00:04:17] crate `rustc-ap-syntax` is duplicated in `Cargo.lock`
[00:04:17]   * rustc-ap-syntax 306.0.0 (registry+https://github.com/rust-lang/crates.io-index)
[00:04:17]   * rustc-ap-syntax 366.0.0 (registry+https://github.com/rust-lang/crates.io-index)
[00:04:17] some tidy checks failed
[00:04:17] 
[00:04:17] 
[00:04:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:17] 
[00:04:17] 
[00:04:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:17] Build completed unsuccessfully in 0:00:46
[00:04:17] Build completed unsuccessfully in 0:00:46
[00:04:17] make: *** [tidy] Error 1
[00:04:17] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:019fb5e0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 10 17:32:40 UTC 2019
---
travis_time:end:131a582f:start=1549819961271267730,finish=1549819961275982844,duration=4715114
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06ed96e7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1333ca28
travis_time:start:1333ca28
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0173bb0c
$ dmesg | grep -i kill
