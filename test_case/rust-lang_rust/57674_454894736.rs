plain
travis_time:end:05fca5af:start=1547664051058777848,finish=1547664052337147155,duration=1278369307
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:54]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:06:58] error[E0425]: cannot find value `values` in this scope
[00:06:58]   --> src/librustc/ty/erase_regions.rs:25:13
[00:06:58]    |
[00:06:58] 25 |         if !values.has_erasable_regions() {
[00:06:58] 
travis_time:end:100e1cb2:start=1547664062089716078,finish=1547664509266816551,duration=447177100473
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a1b302c
