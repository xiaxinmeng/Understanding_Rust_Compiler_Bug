plain
travis_time:end:051c3f48:start=1552088332595544177,finish=1552088417022026211,duration=84426482034
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:26:13]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:26:13] error: unexpected token: `,`
[00:26:13]     --> src/libcore/../stdsimd/crates/core_arch/src/x86/sse.rs:4015:31
[00:26:13]      |
[00:26:13] 4015 |         let a = _mm_setr_ps(0., 0., 3., 4.);
[00:26:13] 
[00:26:13] error: aborting due to previous error
[00:26:13] 
[00:26:13] error: Could not compile `core`.
---
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:0b4edcce
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access '/home/travis/Li86.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:124da0ff
$ dmesg | grep -i kill
