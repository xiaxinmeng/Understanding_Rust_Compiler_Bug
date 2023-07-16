plain
travis_time:end:12c993a2:start=1547750714968761818,finish=1547750716010170075,duration=1041408257
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:59]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:59] error[E0583]: file not found for module `powerpc64_unknown_freebsd`
[00:05:59]    --> src/librustc_target/spec/mod.rs:330:35
[00:05:59]     |
[00:05:59] 330 |     ("powerpc64-unknown-freebsd", powerpc64_unknown_freebsd),
[00:05:59]     |
[00:05:59]     |
[00:05:59]     = help: name the file either powerpc64_unknown_freebsd.rs or powerpc64_unknown_freebsd/mod.rs inside the directory "src/librustc_target/spec"
[00:05:59] error: aborting due to previous error
[00:05:59] 
[00:05:59] For more information about this error, try `rustc --explain E0583`.
[00:05:59] error: Could not compile `rustc_target`.
