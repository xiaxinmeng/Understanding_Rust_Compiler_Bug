plain
travis_time:end:145d4767:start=1556725084284532097,finish=1556725171389207083,duration=87104674986
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:06:05]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:06:18]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:06:29]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:33]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:42] error[E0425]: cannot find value `d_lo` in this scope
[00:06:42]    --> src/libsyntax/parse/mod.rs:315:38
[00:06:42]     |
[00:06:42] 315 |     span.with_lo(span.lo() + BytePos(d_lo)).with_hi(span.hi() - BytePos(d_hi))
[00:06:42] 
[00:06:42] 
[00:06:42] error[E0425]: cannot find value `d_hi` in this scope
[00:06:42]    --> src/libsyntax/parse/mod.rs:315:73
[00:06:42]     |
[00:06:42] 315 |     span.with_lo(span.lo() + BytePos(d_lo)).with_hi(span.hi() - BytePos(d_hi))
[00:06:42] 
[00:06:47] error: aborting due to 2 previous errors
[00:06:47] 
[00:06:47] For more information about this error, try `rustc --explain E0425`.
---
travis_time:end:094c3193:start=1556725588992030206,finish=1556725588996531201,duration=4500995
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05416d2a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:17314ecf
travis_time:start:17314ecf
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f1f2fe6
$ dmesg | grep -i kill
