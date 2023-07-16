plain
travis_time:end:106e5dd0:start=1548713318013727339,finish=1548713321167929911,duration=3154202572
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:12]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:06:12]    Compiling rustc_apfloat v0.0.0 (/checkout/src/librustc_apfloat)
[00:06:15]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:06:15]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:15] error: internal compiler error: src/librustc/middle/stability.rs:751: encountered unmarked API: DefId(49/1:9 ~ serde_derive[774e]::Serialize[0])
[00:06:15]    |
[00:06:15]    |
[00:06:15] 40 | use serde::{Serialize, Deserialize};
[00:06:15] 
[00:06:15] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:526:9
[00:06:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:06:15] error: aborting due to previous error
---
[00:06:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:06:15] 
[00:06:15] note: rustc 1.33.0-beta.1 (d1add9723 2019-01-17) running on x86_64-unknown-linux-gnu
[00:06:15] 
[00:06:15] note: compiler flags: -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib
[00:06:15] note: some of the compiler flags provided by cargo are hidden
[00:06:15] 
[00:06:15] error: Could not compile `syntax_pos`.
[00:06:15] warning: build failed, waiting for other jobs to finish...
