plain
travis_time:end:198348e8:start=1542333890240540368,finish=1542333891307180192,duration=1066639824
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:06:38]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:06:48] error[E0614]: type `ty::Predicate<'_>` cannot be dereferenced
[00:06:48]    --> librustc/infer/outlives/verify.rs:303:27
[00:06:48]     |
[00:06:48] 303 |             .map(|(p, _)| *p);
[00:06:48] 
[00:06:49] error: aborting due to previous error
[00:06:49] 
[00:06:49] For more information about this error, try `rustc --explain E0614`.
