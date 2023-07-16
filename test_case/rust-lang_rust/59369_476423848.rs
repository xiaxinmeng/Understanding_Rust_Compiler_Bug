plain
travis_time:end:0265c1f0:start=1553557105428732598,finish=1553557187673816883,duration=82245084285
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:27:53]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:27:54]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:27:54]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:27:55]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:27:59] error: internal compiler error: src/librustc/ty/sty.rs:2163: cannot lift ParamEnvAnd {
[00:27:59]     param_env: ParamEnv {
[00:27:59]         caller_bounds: [
[00:27:59]                 OutlivesPredicate(
[00:27:59]                     '_#0r,
[00:27:59]                     '_#1r
[00:27:59]                 )
[00:27:59]                 )
[00:27:59]             )
[00:27:59]         ],
[00:27:59]         reveal: UserFacing,
[00:27:59]         def_id: None
[00:27:59]     },
[00:27:59]     value: usize
[00:27:59] } for Const {
[00:27:59]     ty: usize,
[00:27:59]     val: Scalar(
[00:27:59]             size: 8,
[00:27:59]             bits: 2
[00:27:59]         }
[00:27:59]     )
---
[00:28:00] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:28:00] 
[00:28:00] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[00:28:00] 
[00:28:00] note: compiler flags: -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:28:00] note: some of the compiler flags provided by cargo are hidden
[00:28:00] 
[00:28:00] error: Could not compile `core`.
[00:28:00] 
