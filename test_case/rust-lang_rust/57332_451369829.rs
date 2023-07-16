plain
travis_time:end:1b56faa0:start=1546586293616002126,finish=1546586401542122060,duration=107926119934
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:30]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:05:33]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:33]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:38]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:05:51] error: unused import: `emit_feature_err`
[00:05:51]  --> src/libsyntax/config.rs:8:5
[00:05:51] 8 |     emit_feature_err,
[00:05:51]   |     ^^^^^^^^^^^^^^^^
[00:05:51]   |
[00:05:51]   = note: `-D unused-imports` implied by `-D warnings`
