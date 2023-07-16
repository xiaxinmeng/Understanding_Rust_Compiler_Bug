plain
travis_time:end:01a7da7c:start=1553942471076994883,finish=1553942473306716244,duration=2229721361
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:01:52] 
######################################################################## 100.0%
[00:01:52] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:52]     Updating crates.io index
[00:02:06]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:02:07]   Downloaded petgraph v0.4.13
[00:02:07]   Downloaded serde_derive v1.0.81
[00:02:07]   Downloaded libc v0.2.51
[00:02:07]   Downloaded time v0.1.40
---
tidy check
[00:03:55] * 569 error codes
[00:03:55] * highest error code: E0725
[00:03:56] * 252 features
[00:03:57] invalid source: "git+https://github.com/gnzlbg/libtest?branch=clippy_ci#1e98ee4f3b0b6b3f099efd761e1a3d916418797d"

[00:03:57] travis_time:end:tidy:start=1553942718330847223,finish=1553942720100139220,duration=1769291997

[00:03:57] Build completed successfully in 0:00:44
---
[00:04:55]    Compiling libc v0.2.51
[00:04:55]    Compiling getopts v0.2.17
[00:04:55]    Compiling termcolor v1.0.4
[00:04:55]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:04:57]    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1e98ee4f)
[00:05:08]     Finished release [optimized] target(s) in 13.20s
[00:05:08] Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:05:08] travis_fold:end:stage0-test

---
[00:28:56]    Compiling libc v0.2.51
[00:28:56]    Compiling getopts v0.2.17
[00:28:56]    Compiling termcolor v1.0.4
[00:28:56]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:28:59]    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1e98ee4f)
[00:28:59] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[00:28:59]     --> /cargo/git/checkouts/libtest-ed9c19a14a678411/1e98ee4/src/lib.rs:1689:9
[00:28:59]      |
[00:28:59] 1689 |         std::hint::black_box(x)
[00:28:59]      |
[00:28:59]      |
[00:28:59]      = help: add #![feature(test)] to the crate attributes to enable
[00:29:00] error: aborting due to previous error
[00:29:00] 
[00:29:00] For more information about this error, try `rustc --explain E0658`.
[00:29:00] error: Could not compile `libtest`.
---
travis_time:end:007884ac:start=1553944236351236762,finish=1553944236358200420,duration=6963658
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05cc194c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00f1ccdd
travis_time:start:00f1ccdd
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:23396794
$ dmesg | grep -i kill
