plain
travis_time:end:18f807f1:start=1544087150515670062,finish=1544087151505841241,duration=990171179
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:05:15]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:38]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:12:23]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:12:23]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:12:26] error[E0425]: cannot find value `unleash` in this scope
[00:12:26]    --> src/librustc_mir/transform/qualify_consts.rs:991:68
[00:12:26]     |
[00:12:26] 991 |                                 if self.tcx.is_const_fn(def_id) || unleash {
[00:12:26] 
[00:12:40] error: aborting due to previous error
[00:12:40] 
[00:12:40] For more information about this error, try `rustc --explain E0425`.
