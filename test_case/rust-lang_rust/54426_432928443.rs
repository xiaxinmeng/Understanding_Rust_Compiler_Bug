plain
travis_time:end:00daeb1e:start=1540446961002816104,finish=1540447015822519859,duration=54819703755
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:20:28]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:20:30]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:20:30]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:20:30]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:21:00] error: internal compiler error: librustc_codegen_llvm/mir/place.rs:444: using operand local _0 as place
[00:21:00] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:600:9
[00:21:00] note: Run with `RUST_BACKTRACE=1` for a backtrace.
