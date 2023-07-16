plain
travis_time:end:02bffe27:start=1560711591078106586,finish=1560711677855319335,duration=86777212749
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:25:48] 511 |     type Output = Self;
[00:25:48]     |     ^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
[00:25:48]     |
[00:25:48]     = help: the trait `marker::Sized` is not implemented for `Self`
[00:25:48]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:25:48]     = help: consider adding a `where Self: marker::Sized` bound
[00:25:48] note: required by `ops::arith::Rem`
[00:25:48]    --> src/libcore/ops/arith.rs:508:1
[00:25:48]     |
[00:25:48] 508 | pub trait Rem<Rhs=Self> {
[00:25:48] 
[00:25:49]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:25:49]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:25:49]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
---
travis_time:end:0242e968:start=1560713240632915291,finish=1560713240637436918,duration=4521627
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:108439b9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:109b6ae9
travis_time:start:109b6ae9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:039710a0
$ dmesg | grep -i kill
