plain
travis_time:end:012bed40:start=1558294773169039585,finish=1558294773964235677,duration=795196092
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:05:23]    Compiling synstructure v0.10.1
[00:05:37]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:05:44]     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:45]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:05:59] error: function is never used: `dummy`
[00:05:59]   --> src/libsyntax/tokenstream.rs:55:1
[00:05:59] 55 | / fn dummy()
[00:05:59] 56 | | where
[00:05:59] 57 | |     Span: Send + Sync,
[00:05:59] 57 | |     Span: Send + Sync,
[00:05:59] 58 | |     token::Token: Send + Sync,
[00:05:59] ...  |
[00:05:59] 61 | |     TokenStream: Send + Sync,
[00:05:59] 62 | | {}
[00:05:59]    |
[00:05:59]    = note: `-D dead-code` implied by `-D warnings`
[00:05:59] 
[00:06:00] error: aborting due to previous error
---
travis_time:end:0366d711:start=1558295146392795974,finish=1558295146398991039,duration=6195065
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:065171c8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1829528a
travis_time:start:1829528a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2dc5c9e0
$ dmesg | grep -i kill
