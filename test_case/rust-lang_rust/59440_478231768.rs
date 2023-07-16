plain
travis_time:end:2d7d6da5:start=1553939410041327214,finish=1553939412305247373,duration=2263920159
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:01:43] 
######################################################################## 100.0%
[00:01:43] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:43]     Updating crates.io index
[00:01:58]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:02:00]   Downloaded serde_derive v1.0.81
[00:02:00]   Downloaded toml v0.4.10
[00:02:00]   Downloaded serde v1.0.82
[00:02:00]   Downloaded cmake v0.1.33
---
tidy check
[00:03:56] * 569 error codes
[00:03:56] * highest error code: E0725
[00:03:56] * 252 features
[00:03:57] invalid source: "git+https://github.com/gnzlbg/libtest?branch=clippy_ci#719256356b565a138e0005cabbc56153e5902dca"

[00:03:57] travis_time:end:tidy:start=1553939658795247574,finish=1553939660841750878,duration=2046503304

[00:03:57] Build completed successfully in 0:00:46
---
[00:05:00]    Compiling libc v0.2.51
[00:05:00]    Compiling termcolor v1.0.4
[00:05:00]    Compiling getopts v0.2.17
[00:05:00]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:05:02]    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#71925635)
[00:05:13]     Finished release [optimized] target(s) in 13.71s
[00:05:13] Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:05:13] travis_fold:end:stage0-test

---
[00:29:43]    Compiling libc v0.2.51
[00:29:43]    Compiling termcolor v1.0.4
[00:29:43]    Compiling getopts v0.2.17
[00:29:43]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:29:46]    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#71925635)
[00:29:46] error[E0463]: can't find crate for `test`
[00:29:46]   --> /cargo/git/checkouts/libtest-ed9c19a14a678411/7192563/src/lib.rs:15:1
[00:29:46] 15 | extern crate test;
[00:29:46]    | ^^^^^^^^^^^^^^^^^^ can't find crate
[00:29:46] 
[00:29:46] error: aborting due to previous error
---
travis_time:end:1277fce3:start=1553941224391106338,finish=1553941224398997267,duration=7890929
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:034f3392
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14df48f6
travis_time:start:14df48f6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0cf5f028
$ dmesg | grep -i kill
