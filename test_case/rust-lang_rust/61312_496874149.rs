plain
travis_time:end:2462d1d7:start=1559124288932906714,finish=1559124383704774932,duration=94771868218
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:02:20]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:46] error: RUSTC_STAGE was not set
[00:02:46]   --> src/bootstrap/bin/rustdoc.rs:15:17
[00:02:46]    |
[00:02:46] 15 |     let stage = env!("RUSTC_STAGE","RUSTC_STAGE was not set").to_String();
[00:02:46] 
[00:02:46] error: aborting due to previous error
[00:02:46] 
[00:02:46] error: Could not compile `bootstrap`.
---
[00:02:50]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:50] error: RUSTC_STAGE was not set
[00:02:50]   --> src/bootstrap/bin/rustdoc.rs:15:17
[00:02:50]    |
[00:02:50] 15 |     let stage = env!("RUSTC_STAGE","RUSTC_STAGE was not set").to_String();
[00:02:50] 
[00:02:50] error: aborting due to previous error
[00:02:50] 
[00:02:50] error: Could not compile `bootstrap`.
---
[00:02:55]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:55] error: RUSTC_STAGE was not set
[00:02:55]   --> src/bootstrap/bin/rustdoc.rs:15:17
[00:02:55]    |
[00:02:55] 15 |     let stage = env!("RUSTC_STAGE","RUSTC_STAGE was not set").to_String();
[00:02:55] 
[00:02:55] error: aborting due to previous error
[00:02:55] 
[00:02:55] error: Could not compile `bootstrap`.
---
[00:02:58]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:58] error: RUSTC_STAGE was not set
[00:02:58]   --> src/bootstrap/bin/rustdoc.rs:15:17
[00:02:58]    |
[00:02:58] 15 |     let stage = env!("RUSTC_STAGE","RUSTC_STAGE was not set").to_String();
[00:02:58] 
[00:02:58] error: aborting due to previous error
[00:02:58] 
[00:02:58] error: Could not compile `bootstrap`.
---
[00:03:03]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:03:03] error: RUSTC_STAGE was not set
[00:03:03]   --> src/bootstrap/bin/rustdoc.rs:15:17
[00:03:03]    |
[00:03:03] 15 |     let stage = env!("RUSTC_STAGE","RUSTC_STAGE was not set").to_String();
[00:03:03] 
[00:03:03] error: aborting due to previous error
[00:03:03] 
[00:03:03] error: Could not compile `bootstrap`.
---
travis_time:end:0bd8cbd1:start=1559124579624136503,finish=1559124579630306727,duration=6170224
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:113c233e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0bceb4e0
travis_time:start:0bceb4e0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13d3daa4
$ dmesg | grep -i kill
