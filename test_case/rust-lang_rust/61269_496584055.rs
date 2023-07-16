plain
travis_time:end:1db34cb6:start=1559058880614299859,finish=1559058967598286603,duration=86983986744
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:44]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:13:32]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:13:32]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:13:40] error[E0599]: no method named `local` found for type `&rustc::mir::Place<'_>` in the current scope
[00:13:40]    --> src/librustc_mir/build/expr/as_rvalue.rs:531:32
[00:13:40]     |
[00:13:40] 531 |                     match base.local() {
[00:13:40] 
[00:13:46] error: aborting due to previous error
[00:13:46] 
[00:13:46] For more information about this error, try `rustc --explain E0599`.
---
travis_time:end:00fe2298:start=1559059881362464039,finish=1559059881366727073,duration=4263034
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0012146c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c2b5287
travis_time:start:0c2b5287
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13a6abcc
$ dmesg | grep -i kill
