plain
travis_time:end:0a8d1876:start=1551426417322061806,finish=1551426536556350387,duration=119234288581
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:51]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:57] error: unused `core::result::Result` that must be used
[00:04:57]    --> src/libstd/sys/unix/fs.rs:903:9
[00:04:57]     |
[00:04:57] 903 |         writer.set_permissions(perm);
[00:04:57]     |
[00:04:57]     = note: `-D unused-must-use` implied by `-D warnings`
[00:04:57]     = note: `-D unused-must-use` implied by `-D warnings`
[00:04:57]     = note: this `Result` may be an `Err` variant, which should be handled
[00:04:57] error: aborting due to previous error
[00:04:57] 
[00:04:57] error: Could not compile `std`.
[00:04:57] 
