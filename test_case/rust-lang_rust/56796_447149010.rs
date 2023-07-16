plain
travis_time:end:20e6936e:start=1544738342305069551,finish=1544738344148611549,duration=1843541998
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:45:08] .................................................................................................... 500/5170
[00:45:11] ..............................i..................................................................... 600/5170
[00:45:14] .................................................................................................... 700/5170
[00:45:20] .................................................................................................... 800/5170
[00:45:24] .i...............i.................................................................................. 900/5170
[00:45:27] ........................iiiii...........................................................F........... 1000/5170
[00:45:32] .................................................................................................... 1200/5170
[00:45:34] .................................................................................................... 1300/5170
[00:45:36] .................................................................................................... 1400/5170
[00:45:38] .................................................................................................... 1500/5170
---
[00:47:37] ---- [ui] ui/e0119/conflict-with-std.rs stdout ----
[00:47:37] diff of stderr:
[00:47:37] 
[00:47:37] 25    |
[00:47:37] 26    = note: conflicting implementation in crate `core`:
[00:47:37] 27            - impl<T, U> std::convert::TryFrom<U> for T
[00:47:37] -              where T: std::convert::From<U>;
[00:47:37] +              where U: std::convert::Into<T>;
[00:47:37] 30 error: aborting due to 3 previous errors
[00:47:37] 31 
[00:47:37] 
[00:47:37] 
[00:47:37] 
[00:47:37] The actual stderr differed from the expected stderr.
[00:47:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/e0119/conflict-with-std/conflict-with-std.stderr
[00:47:37] To update references, rerun the tests and pass the `--bboth the `impl<T> MyTrait for T` where T is all types and the `impl\nMyTrait for Foo`. Since a trait cannot be implemented multiple times,\nthis is an error. So, when you write:\n\n