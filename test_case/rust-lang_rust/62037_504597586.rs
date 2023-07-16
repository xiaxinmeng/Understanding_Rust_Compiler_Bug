plain
travis_time:end:00978e6b:start=1561152293273614327,finish=1561152294076285515,duration=802671188
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:56] 
[01:03:56] running 9 tests
[01:03:56] iiiiiiiii
[01:03:56] 
[01:03:56]  finished in 0.150
[01:03:56] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:12] 
[01:04:12] running 122 tests
[01:04:36] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:04:41] .i.i......iii.i.....ii
[01:04:41] 
[01:04:41]  finished in 28.924
[01:04:41] travis_fold:end:test_debuginfo

---
travis_time:start:stage0-unstable-book-gen
Building stage0 tool unstable-book-gen (x86_64-unknown-linux-gnu)
[01:29:22]    Compiling num-traits v0.2.6
[01:29:24]    Compiling unstable-book-gen v0.1.0 (/checkout/src/tools/unstable-book-gen)
[01:29:24] error[E0432]: unresolved import `tidy::features::collect_lib_features`
[01:29:24]  --> src/tools/unstable-book-gen/src/main.rs:8:41
[01:29:24]   |
[01:29:24] 8 | use tidy::features::{Feature, Features, collect_lib_features, collect_lang_features};
[01:29:24]   |                                         |
[01:29:24]   |                                         no `collect_lib_features` in `features`
[01:29:24]   |                                         help: a similar name exists in the module: `collect_lang_features`
[01:29:24] 
---
travis_time:end:042fd443:start=1561157671392699414,finish=1561157671397850919,duration=5151505
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03ccfe64
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0784c58c
travis_time:start:0784c58c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02da1b9e
$ dmesg | grep -i kill
