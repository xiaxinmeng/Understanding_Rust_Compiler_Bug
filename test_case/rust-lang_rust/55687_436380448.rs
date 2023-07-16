plain
travis_time:end:30fa75b4:start=1541531533811833891,finish=1541531609009158985,duration=75197325094
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:20:50]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:20:50]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:20:51]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:20:51]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:21:26] error[E0719]: the value of the associated type `Output` (from the trait `core::ops::Add`) is already specified
[00:21:26] 
[00:21:26] error[E0719]: the value of the associated type `Output` (from the trait `core::ops::Sub`) is already specified
[00:21:26] 
[00:21:26] error[E0719]: the value of the associated type `Output` (from the trait `core::ops::Div`) is already specified
[00:21:26] 
[00:21:26] error[E0719]: the value of the associated type `Output` (from the trait `core::ops::Shl`) is already specified
[00:21:26] 
[00:21:26] error[E0719]: the value of the associated type `Output` (from the trait `core::ops::Shr`) is already specified
[00:21:26] 
[00:21:26] error[E0719]: the value of the associated type `Output` (from the trait `core::ops::BitOr`) is already specified
[00:21:26] 
[00:21:26] error[E0719]: the value of the associated type `Output` (from the trait `core::ops::BitXor`) is already specified
[00:21:26] 
[00:21:26] error[E0719]: the value of the associated type `Output` (from the trait `core::ops::BitAnd`) is already specified
[00:21:26] 
[00:21:26] error[E0719]: the value of the associated type `Output` (from the trait `core::ops::Not`) is already specified
[00:21:26] error: aborting due to 9 previous errors
[00:21:26] 
[00:21:26] For mo
travis_time:end:0d8c9a7a:start=1541531619233137714,finish=1541532906345179950,duration=1287112042236
