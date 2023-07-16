plain
travis_time:end:00ce1cda:start=1552108483731564947,finish=1552108484615711045,duration=884146098
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:56:56]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:57:01] error[E0615]: attempted to take value of method `chars` on type `std::string::String`
[00:57:01]    --> src/librustdoc/passes/collect_intra_doc_links.rs:302:25
[00:57:01]     |
[00:57:01] 302 |             if ori_link.chars.first().unwrap().is_numeric() {
[00:57:01]     |                         ^^^^^ help: use parentheses to call the method: `chars()`
[00:57:01] error: aborting due to previous error
[00:57:01] 
[00:57:01] For more information about this error, try `rustc --explain E0615`.
[00:57:01] error: Could not compile `rustdoc`.
