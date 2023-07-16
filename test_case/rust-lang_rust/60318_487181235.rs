plain
travis_time:end:26d0edb4:start=1556308197659287217,finish=1556308198435658759,duration=776371542
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:03:54]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:58] error[E0412]: cannot find type `Infallible` in this scope
[00:03:58]   --> src/libcore/array.rs:76:11
[00:03:58]    |
[00:03:58] 76 | impl From<Infallible> for TryFromSliceError {
[00:03:58] help: possible candidate is found in another module, you can import it into scope
[00:03:58]    |
[00:03:58] 12 | use crate::convert::Infallible;
[00:03:58]    |
[00:03:58]    |
[00:03:58] 
[00:03:58] error[E0412]: cannot find type `Infallible` in this scope
[00:03:58]   --> src/libcore/array.rs:77:16
[00:03:58]    |
[00:03:58] 77 |     fn from(x: Infallible) -> TryFromSliceError {
[00:03:58] help: possible candidate is found in another module, you can import it into scope
[00:03:58]    |
[00:03:58] 12 | use crate::convert::Infallible;
[00:03:58]    |
---
travis_time:end:02a98280:start=1556308463490887061,finish=1556308463495250691,duration=4363630
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16cfd913
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05b91784
travis_time:start:05b91784
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d6f5f9e
$ dmesg | grep -i kill
