plain
travis_time:end:0479f026:start=1557948186399977215,finish=1557948277285952012,duration=90885974797
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:06:43]    Compiling synstructure v0.10.1
[00:07:06]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:13]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:07:17]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:25] error[E0425]: cannot find value `weak_into_raw` in module `sym`
[00:07:25]    --> src/libsyntax/feature_gate.rs:558:14
[00:07:25]     |
[00:07:25] 558 |     (active, weak_into_raw, "1.36.0", Some(60728), None),
[00:07:25]     |              ^^^^^^^^^^^^^ not found in `sym`
[00:07:31] error: aborting due to previous error
[00:07:31] 
[00:07:31] For more information about this error, try `rustc --explain E0425`.
[00:07:31] error: Could not compile `syntax`.
---
travis_time:end:02cc3fe6:start=1557948739273421192,finish=1557948739278061071,duration=4639879
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0969b9af
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:179a3bb5
travis_time:start:179a3bb5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2f0baca0
$ dmesg | grep -i kill
