plain
travis_time:end:0742d578:start=1560625015477235089,finish=1560625016353973599,duration=876738510
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:26:39] 511 |     type Output = Self;
[00:26:39]     |     ^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
[00:26:39]     |
[00:26:39]     = help: the trait `marker::Sized` is not implemented for `Self`
[00:26:39]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:26:39]     = help: consider adding a `where Self: marker::Sized` bound
[00:26:39] note: required by `ops::arith::Rem`
[00:26:39]    --> src/libcore/ops/arith.rs:508:1
[00:26:39]     |
[00:26:39] 508 | pub trait Rem<Rhs=Self> {
[00:26:39] 
[00:26:40]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:26:40]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:26:40]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
