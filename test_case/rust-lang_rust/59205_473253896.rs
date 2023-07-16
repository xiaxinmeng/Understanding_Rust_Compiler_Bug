plain
travis_time:end:15e351a0:start=1552646564546581546,finish=1552646565500286631,duration=953705085
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:52:48]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:52:48] error: incorrect close delimiter: `}`
[00:52:48]    --> src/librustc_interface/queries.rs:227:13
[00:52:48]     |
[00:52:48] 213 |             self.global_ctxt()?.peek_mut().enter(|tcx| {
[00:52:48] ...
[00:52:48] 221 |                 Ok((passes::start_codegen(
[00:52:48]     |                   - un-closed delimiter
[00:52:48] ...
