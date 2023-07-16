plain
travis_time:end:00e9044c:start=1554647088455066760,finish=1554647091032972537,duration=2577905777
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:03:55]    Compiling libc v0.2.51
[00:03:55]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:03:55]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:56]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:04:00] error[E0433]: failed to resolve: could not find `ops` in `{{root}}`
[00:04:00] 
[00:04:00] error[E0433]: failed to resolve: could not find `result` in `{{root}}`
[00:04:00] 
[00:04:00] error[E0433]: failed to resolve: could not find `convert` in `{{root}}`
[00:04:00] 
[00:04:00] error[E0433]: failed to resolve: could not find `option` in `{{root}}`
[00:04:00] 
[00:04:00] error[E0433]: failed to resolve: could not find `iter` in `{{root}}`
[00:04:01]    Compiling compiler_builtins v0.1.9
[00:04:01]    Compiling cmake v0.1.33
[00:04:01]    Compiling backtrace-sys v0.1.27
[00:04:04]    Compiling std v0.0.0 (/checkout/src/libstd)
---
[00:04:10] 
[00:04:10] error[E0277]: the size for values of type `dyn fmt::Debug` cannot be known at compilation time
[00:04:10]    --> src/libcore/fmt/builders.rs:770:13
[00:04:10]     |
[00:04:10] 770 |         for (k, v) in entries {
[00:04:10]     |
[00:04:10]     |
[00:04:10]     = help: within `(dyn fmt::Debug, dyn fmt::Debug)`, the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:04:10]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:10]     = note: required because it appears within the type `(dyn fmt::Debug, dyn fmt::Debug)`
[00:04:10]     = note: the left-hand-side of an assignment must have a statically known size
[00:04:12] error: aborting due to 8 previous errors
[00:04:12] 
[00:04:12] Some errors occurred: E0277, E0433.
[00:04:12] For more information about an error, try `rustc --explain E0277`.
