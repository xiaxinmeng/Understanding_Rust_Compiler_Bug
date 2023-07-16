plain
travis_time:end:129ddc38:start=1553620829448729043,finish=1553620830475099708,duration=1026370665
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:01:30] 
######################################################################## 100.0%
[00:01:30] extracting /checkout/obj/build/cache/2019-02-27/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:30]     Updating crates.io index
[00:01:44]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:01:46]   Downloaded libc v0.2.50
[00:01:46]   Downloaded serde_json v1.0.33
[00:01:46]   Downloaded num_cpus v1.8.0
[00:01:46]   Downloaded cmake v0.1.33
---
tidy check
[00:03:35] * 569 error codes
[00:03:35] * highest error code: E0725
[00:03:35] * 252 features
[00:03:36] invalid source: "git+https://github.com/gnzlbg/libtest?branch=clippy_ci#719256356b565a138e0005cabbc56153e5902dca"

[00:03:36] travis_time:end:tidy:start=1553621056560170676,finish=1553621058543745738,duration=1983575062

[00:03:36] Build completed successfully in 0:00:44
---
[00:04:34]    Compiling libc v0.2.50
[00:04:34]    Compiling termcolor v1.0.4
[00:04:34]    Compiling getopts v0.2.17
[00:04:34]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:04:36]    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#71925635)
[00:04:46]     Finished release [optimized] target(s) in 13.05s
[00:04:46] Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:04:46] travis_fold:end:stage0-test

---
[00:27:24]    Compiling libc v0.2.50
[00:27:24]    Compiling getopts v0.2.17
[00:27:24]    Compiling termcolor v1.0.4
[00:27:24]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:27:27]    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#71925635)
[00:27:27] error[E0463]: can't find crate for `test`
[00:27:27]   --> /cargo/git/checkouts/libtest-ed9c19a14a678411/7192563/src/lib.rs:15:1
[00:27:27] 15 | extern crate test;
[00:27:27]    | ^^^^^^^^^^^^^^^^^^ can't find crate
[00:27:27] 
[00:27:27] error: aborting due to previous error
---
travis_time:end:084960da:start=1553622502787770652,finish=1553622502793260870,duration=5490218
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06d44d6c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07b1a8c8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/buil
