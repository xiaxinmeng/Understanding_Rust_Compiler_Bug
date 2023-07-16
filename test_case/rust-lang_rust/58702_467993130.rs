plain
travis_time:end:1803c309:start=1551294690079037847,finish=1551294692334123364,duration=2255085517
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:57]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:03:57]    Compiling libc v0.2.46
[00:03:57]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:58]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:04:03] error[E0433]: failed to resolve: could not find `ops` in `{{root}}`
[00:04:03] 
[00:04:03] error[E0433]: failed to resolve: could not find `option` in `{{root}}`
[00:04:03] 
[00:04:03] error[E0433]: failed to resolve: could not find `iter` in `{{root}}`
[00:04:03] 
[00:04:03] error[E0433]: failed to resolve: could not find `result` in `{{root}}`
[00:04:03] 
[00:04:03] error[E0433]: failed to resolve: could not find `convert` in `{{root}}`
[00:04:03]    Compiling compiler_builtins v0.1.5
[00:04:03]    Compiling cmake v0.1.33
[00:04:03]    Compiling backtrace-sys v0.1.27
[00:04:06]    Compiling std v0.0.0 (/checkout/src/libstd)
---
[00:04:12]     |
[00:04:12] 492 |         for entry in entries {
[00:04:12]     |             ^^^^^ doesn't have a size known at compile-time
[00:04:12]     |
[00:04:12]     = help: the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:04:12]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:12]     = note: the left-hand-side of an assignment must have a statically known size
[00:04:12] error[E0277]: the size for values of type `dyn fmt::Debug` cannot be known at compilation time
[00:04:12]    --> src/libcore/fmt/builders.rs:630:13
[00:04:12]     |
[00:04:12] 630 |         for entry in entries {
[00:04:12] 630 |         for entry in entries {
[00:04:12]     |             ^^^^^ doesn't have a size known at compile-time
[00:04:12]     |
[00:04:12]     = help: the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:04:12]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:12]     = note: the left-hand-side of an assignment must have a statically known size
[00:04:12] error[E0277]: the size for values of type `dyn fmt::Debug` cannot be known at compilation time
[00:04:12]    --> src/libcore/fmt/builders.rs:791:13
[00:04:12]     |
[00:04:12]     |
[00:04:12] 791 |         for (k, v) in entries {
[00:04:12]     |
[00:04:12]     |
[00:04:12]     = help: within `(dyn fmt::Debug, dyn fmt::Debug)`, the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:04:12]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:12]     = note: required because it appears within the type `(dyn fmt::Debug, dyn fmt::Debug)`
[00:04:12]     = note: the left-hand-side of an assignment must have a statically known size
[00:04:14] error: aborting due to 8 previous errors
[00:04:14] 
[00:04:14] Some errors occurred: E0277, E0433.
[00:04:14] For more information about an error, try `rustc --explain E0277`.
