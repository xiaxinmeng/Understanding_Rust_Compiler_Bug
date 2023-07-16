plain
travis_time:end:2674bf38:start=1552783685835745880,finish=1552783686918166667,duration=1082420787
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:13:39]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:42] error[E0615]: attempted to take value of method `span` on type `&syntax::ast::NestedMetaItem`
[00:13:42]   --> src/librustc_metadata/native_libs.rs:77:66
[00:13:42]    |
[00:13:42] 77 |                             struct_span_err!(self.tcx.sess, item.span, E0458,
[00:13:42]    |                                                                  ^^^^ help: use parentheses to call the method: `span()`
[00:13:42] error: aborting due to previous error
[00:13:42] 
[00:13:42] For more information about this error, try `rustc --explain E0615`.
[00:13:42] error: Could not compile `rustc_metadata`.
