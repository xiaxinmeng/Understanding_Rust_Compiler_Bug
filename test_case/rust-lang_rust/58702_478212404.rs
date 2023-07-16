plain
travis_time:end:04c02c86:start=1553926524343021816,finish=1553926526701901827,duration=2358880011
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:03:56]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:03:56]    Compiling libc v0.2.51
[00:03:56]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:57]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:04:01] error[E0433]: failed to resolve: could not find `ops` in `{{root}}`
[00:04:01] 
[00:04:01] error[E0433]: failed to resolve: could not find `result` in `{{root}}`
[00:04:01] 
[00:04:01] error[E0433]: failed to resolve: could not find `convert` in `{{root}}`
[00:04:01] 
[00:04:01] error[E0433]: failed to resolve: could not find `option` in `{{root}}`
[00:04:01] 
[00:04:01] error[E0433]: failed to resolve: could not find `iter` in `{{root}}`
[00:04:02]    Compiling compiler_builtins v0.1.8
[00:04:02]    Compiling cmake v0.1.33
[00:04:02]    Compiling backtrace-sys v0.1.27
[00:04:05]    Compiling std v0.0.0 (/checkout/src/libstd)
---
[00:04:11]     |
[00:04:11] 492 |         for entry in entries {
[00:04:11]     |             ^^^^^ doesn't have a size known at compile-time
[00:04:11]     |
[00:04:11]     = help: the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:04:11]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:11]     = note: the left-hand-side of an assignment must have a statically known size
[00:04:11] error[E0277]: the size for values of type `dyn fmt::Debug` cannot be known at compilation time
[00:04:11]    --> src/libcore/fmt/builders.rs:630:13
[00:04:11]     |
[00:04:11] 630 |         for entry in entries {
[00:04:11] 630 |         for entry in entries {
[00:04:11]     |             ^^^^^ doesn't have a size known at compile-time
[00:04:11]     |
[00:04:11]     = help: the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:04:11]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:11]     = note: the left-hand-side of an assignment must have a statically known size
[00:04:11] error[E0277]: the size for values of type `dyn fmt::Debug` cannot be known at compilation time
[00:04:11]    --> src/libcore/fmt/builders.rs:791:13
[00:04:11]     |
[00:04:11]     |
[00:04:11] 791 |         for (k, v) in entries {
[00:04:11]     |
[00:04:11]     |
[00:04:11]     = help: within `(dyn fmt::Debug, dyn fmt::Debug)`, the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:04:11]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:11]     = note: required because it appears within the type `(dyn fmt::Debug, dyn fmt::Debug)`
[00:04:11]     = note: the left-hand-side of an assignment must have a statically known size
[00:04:13] error: aborting due to 8 previous errors
[00:04:13] 
[00:04:13] Some errors occurred: E0277, E0433.
[00:04:13] For more information about an error, try `rustc --explain E0277`.
