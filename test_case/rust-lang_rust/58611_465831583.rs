plain
travis_time:end:1103f9c1:start=1550710977374801267,finish=1550711050515666217,duration=73140864950
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:58:22]  Documenting core v0.0.0 (/checkout/src/libcore)
[00:58:38] warning: doc comment contains an invalid Rust code block
[00:58:38]     --> src/libcore/num/mod.rs:28:9
[00:58:38]      |
[00:58:38] 28   |           #[doc = $x]
[00:58:38]      | 
[00:58:38]     ::: src/libcore/sync/atomic.rs:1922:1
[00:58:38]      |
[00:58:38]      |
[00:58:38] 1922 | / atomic_int! {
[00:58:38] 1923 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:58:38] 1924 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:58:38] 1925 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:58:38] ...    |
[00:58:38] 1934 | |     i8 AtomicI8 ATOMIC_I8_INIT
[00:58:38]      | |_- in this macro invocation
[00:58:38]      |
[00:58:38]      = help: mark blocks that do not contain Rust code as text: 