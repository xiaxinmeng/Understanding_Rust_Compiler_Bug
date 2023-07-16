plain
travis_time:end:123d1f14:start=1551896681564999974,finish=1551896755399873792,duration=73834873818
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:03:55]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:03:55]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:03:55]    Compiling rustc-demangle v0.1.10
[00:04:00]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:06] error: type does not implement `fmt::Debug`; consider adding #[derive(Debug)] or a manual implementation
[00:04:06]     |
[00:04:06]     |
[00:04:06] 239 | pub struct IsBoxDynError<T>(T);
[00:04:06]     |
[00:04:06] note: lint level defined here
[00:04:06]    --> src/libstd/lib.rs:210:9
[00:04:06]     |
