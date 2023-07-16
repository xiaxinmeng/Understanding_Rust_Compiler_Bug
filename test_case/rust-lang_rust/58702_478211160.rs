plain
travis_time:end:030d76c4:start=1553925301944894751,finish=1553925304964537970,duration=3019643219
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:04:17]    Compiling libc v0.2.50
[00:04:17]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:04:17]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:04:17]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:04:22] error[E0433]: failed to resolve: could not find `ops` in `{{root}}`
[00:04:22] 
[00:04:22] error[E0433]: failed to resolve: could not find `result` in `{{root}}`
[00:04:22] 
[00:04:22] error[E0433]: failed to resolve: could not find `convert` in `{{root}}`
[00:04:22] 
[00:04:22] error[E0433]: failed to resolve: could not find `option` in `{{root}}`
[00:04:22] 
[00:04:22] error[E0433]: failed to resolve: could not find `iter` in `{{root}}`
[00:04:22] 
[00:04:23] error: `...` range patterns are deprecated
[00:04:23]      |
[00:04:23]      |
[00:04:23] 3942 |             b'A'...b'Z' | b'a'...b'z' => true,
[00:04:23]      |                 ^^^ help: use `..=` for an inclusive range
[00:04:23] note: lint level defined here
[00:04:23]     --> src/libcore/lib.rs:65:9
[00:04:23]      |
[00:04:23]      |
[00:04:23] 65   | #![deny(rust_2018_idioms)]
[00:04:23]      |         ^^^^^^^^^^^^^^^^
[00:04:23]      = note: #[deny(ellipsis_inclusive_range_patterns)] implied by #[deny(rust_2018_idioms)]
[00:04:23] 
[00:04:23] error: `...` range patterns are deprecated
[00:04:23]      |
[00:04:23]      |
[00:04:23] 3942 |             b'A'...b'Z' | b'a'...b'z' => true,
[00:04:23]      |                               ^^^ help: use `..=` for an inclusive range
[00:04:23] 
[00:04:23] error: `...` range patterns are deprecated
[00:04:23]      |
[00:04:23]      |
[00:04:23] 3977 |             b'A'...b'Z' => true,
[00:04:23]      |                 ^^^ help: use `..=` for an inclusive range
[00:04:23] 
[00:04:23] error: `...` range patterns are deprecated
[00:04:23]      |
[00:04:23]      |
[00:04:23] 4012 |             b'a'...b'z' => true,
[00:04:23]      |                 ^^^ help: use `..=` for an inclusive range
[00:04:23] 
[00:04:23] error: `...` range patterns are deprecated
[00:04:23]      |
[00:04:23]      |
[00:04:23] 4050 |             b'0'...b'9' | b'A'...b'Z' | b'a'...b'z' => true,
[00:04:23]      |                 ^^^ help: use `..=` for an inclusive range
[00:04:23] 
[00:04:23] error: `...` range patterns are deprecated
[00:04:23]      |
[00:04:23]      |
[00:04:23] 4050 |             b'0'...b'9' | b'A'...b'Z' | b'a'...b'z' => true,
[00:04:23]      |                               ^^^ help: use `..=` for an inclusive range
[00:04:23] 
[00:04:23] error: `...` range patterns are deprecated
[00:04:23]      |
[00:04:23]      |
[00:04:23] 4050 |             b'0'...b'9' | b'A'...b'Z' | b'a'...b'z' => true,
[00:04:23]      |                                             ^^^ help: use `..=` for an inclusive range
[00:04:23] 
[00:04:23] error: `...` range patterns are deprecated
[00:04:23]      |
[00:04:23]      |
[00:04:23] 4085 |             b'0'...b'9' => true,
[00:04:23]      |                 ^^^ help: use `..=` for an inclusive range
[00:04:23] 
[00:04:23] error: `...` range patterns are deprecated
[00:04:23]      |
[00:04:23]      |
[00:04:23] 4123 |             b'0'...b'9' | b'A'...b'F' | b'a'...b'f' => true,
[00:04:23]      |                 ^^^ help: use `..=` for an inclusive range
[00:04:23] 
[00:04:23] error: `...` range patterns are deprecated
[00:04:23]      |
[00:04:23]      |
[00:04:23] 4123 |             b'0'...b'9' | b'A'...b'F' | b'a'...b'f' => true,
[00:04:23]      |                               ^^^ help: use `..=` for an inclusive range
[00:04:23] 
[00:04:23] error: `...` range patterns are deprecated
[00:04:23]      |
[00:04:23]      |
[00:04:23] 4123 |             b'0'...b'9' | b'A'...b'F' | b'a'...b'f' => true,
[00:04:23]      |                                             ^^^ help: use `..=` for an inclusive range
[00:04:23] 
[00:04:23] error: `...` range patterns are deprecated
[00:04:23]      |
[00:04:23]      |
[00:04:23] 4162 |             b'!'...b'/' | b':'...b'@' | b'['...b'`' | b'{'...b'~' => true,
[00:04:23]      |                 ^^^ help: use `..=` for an inclusive range
[00:04:23] 
[00:04:23] error: `...` range patterns are deprecated
[00:04:23]      |
[00:04:23]      |
[00:04:23] 4162 |             b'!'...b'/' | b':'...b'@' | b'['...b'`' | b'{'...b'~' => true,
[00:04:23]      |                               ^^^ help: use `..=` for an inclusive range
[00:04:23] 
[00:04:23] error: `...` range patterns are deprecated
[00:04:23]      |
[00:04:23]      |
[00:04:23] 4162 |             b'!'...b'/' | b':'...b'@' | b'['...b'`' | b'{'...b'~' => true,
[00:04:23]      |                                             ^^^ help: use `..=` for an inclusive range
[00:04:23] 
[00:04:23] error: `...` range patterns are deprecated
[00:04:23]      |
[00:04:23]      |
[00:04:23] 4162 |             b'!'...b'/' | b':'...b'@' | b'['...b'`' | b'{'...b'~' => true,
[00:04:23]      |                                                           ^^^ help: use `..=` for an inclusive range
[00:04:23] 
[00:04:23] error: `...` range patterns are deprecated
[00:04:23]      |
[00:04:23]      |
[00:04:23] 4197 |             b'!'...b'~' => true,
[00:04:23]      |                 ^^^ help: use `..=` for an inclusive range
[00:04:23] 
[00:04:23] error: `...` range patterns are deprecated
[00:04:23]      |
[00:04:23]      |
[00:04:23] 4286 |             b'\0'...b'\x1F' | b'\x7F' => true,
[00:04:23]      |                  ^^^ help: use `..=` for an inclusive range
[00:04:23]    Compiling compiler_builtins v0.1.5
[00:04:23]    Compiling cmake v0.1.33
[00:04:23]    Compiling backtrace-sys v0.1.27
[00:04:26]    Compiling std v0.0.0 (/checkout/src/libstd)
---
[00:04:32]     |
[00:04:32] 492 |         for entry in entries {
[00:04:32]     |             ^^^^^ doesn't have a size known at compile-time
[00:04:32]     |
[00:04:32]     = help: the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:04:32]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:32]     = note: the left-hand-side of an assignment must have a statically known size
[00:04:32] error[E0277]: the size for values of type `dyn fmt::Debug` cannot be known at compilation time
[00:04:32]    --> src/libcore/fmt/builders.rs:630:13
[00:04:32]     |
[00:04:32] 630 |         for entry in entries {
[00:04:32] 630 |         for entry in entries {
[00:04:32]     |             ^^^^^ doesn't have a size known at compile-time
[00:04:32]     |
[00:04:32]     = help: the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:04:32]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:32]     = note: the left-hand-side of an assignment must have a statically known size
[00:04:32] error[E0277]: the size for values of type `dyn fmt::Debug` cannot be known at compilation time
[00:04:32]    --> src/libcore/fmt/builders.rs:791:13
[00:04:32]     |
[00:04:32]     |
[00:04:32] 791 |         for (k, v) in entries {
[00:04:32]     |
[00:04:32]     |
[00:04:32]     = help: within `(dyn fmt::Debug, dyn fmt::Debug)`, the trait `marker::Sized` is not implemented for `dyn fmt::Debug`
[00:04:32]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:32]     = note: required because it appears within the type `(dyn fmt::Debug, dyn fmt::Debug)`
[00:04:32]     = note: the left-hand-side of an assignment must have a statically known size
[00:04:35] error: aborting due to 25 previous errors
[00:04:35] 
[00:04:35] Some errors occurred: E0277, E0433.
[00:04:35] For more information about an error, try `rustc --explain E0277`.
