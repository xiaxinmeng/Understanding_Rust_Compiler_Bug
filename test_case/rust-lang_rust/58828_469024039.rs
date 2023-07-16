plain
travis_time:end:002e59e1:start=1551620275882639991,finish=1551620278295484405,duration=2412844414
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:26]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:27] error[E0432]: unresolved import `crate::panicking::LOCAL_STDERR`
[00:04:27]    --> src/libstd/io/stdio.rs:762:9
[00:04:27]     |
[00:04:27] 762 |     use crate::panicking::LOCAL_STDERR;
[00:04:27]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `LOCAL_STDERR` in `panicking`
[00:04:30] error: aborting due to previous error
[00:04:30] 
[00:04:30] For more information about this error, try `rustc --explain E0432`.
[00:04:30] error: Could not compile `std`.
---
travis_time:end:28b49718:start=1551620560619339199,finish=1551620560624198164,duration=4858965
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a0dfbd8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1853d288
travis_time:start:1853d288
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_fai
