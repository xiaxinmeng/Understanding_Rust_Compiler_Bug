plain
travis_time:end:22773400:start=1551622162967745851,finish=1551622260666175817,duration=97698429966
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:52]    Compiling test v0.0.0 (/checkout/src/libtest)
[00:04:52] error: stability attributes may not be used outside of the standard library
[00:04:52]   --> src/libtest/lib.rs:12:1
[00:04:52]    |
[00:04:52] 12 | #![unstable(feature = "test", issue = "27812")]
[00:04:52] 
[00:04:52] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[00:04:52]   --> src/libtest/lib.rs:16:1
[00:04:52]    |
[00:04:52]    |
[00:04:52] 16 | extern crate libtest;
[00:04:52]    | ^^^^^^^^^^^^^^^^^^^^^
[00:04:52]    |
[00:04:52]    = help: add #![feature(test)] to the crate attributes to enable
[00:04:52] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[00:04:52]   --> src/libtest/lib.rs:17:9
[00:04:52]    |
[00:04:52] 17 | pub use libtest::*;
[00:04:52] 17 | pub use libtest::*;
[00:04:52]    |         ^^^^^^^
[00:04:52]    |
[00:04:52]    = help: add #![feature(test)] to the crate attributes to enable
[00:04:52] error: aborting due to 3 previous errors
[00:04:52] 
[00:04:52] For more information about this error, try `rustc --explain E0658`.
[00:04:52] error: Could not compile `test`.
---
travis_time:end:0a30f297:start=1551622564246109805,finish=1551622564252614456,duration=6504651
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0164faf9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03da33cb
travis_time:start:03da33cb
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11a8a35c
$ dmesg | grep -i kill
