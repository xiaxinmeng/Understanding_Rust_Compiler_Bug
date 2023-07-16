plain
travis_time:end:0a522208:start=1555017948065683441,finish=1555018064662934039,duration=116597250598
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:06:14]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:06:15] error: unused variable: `expects_semi`
[00:06:15]    --> src/libsyntax/parse/parser.rs:799:17
[00:06:15]     |
[00:06:15] 799 |             let expects_semi = expected.iter().any(|t| match t {
[00:06:15]     |                 ^^^^^^^^^^^^ help: consider prefixing with an underscore: `_expects_semi`
[00:06:15]     = note: `-D unused-variables` implied by `-D warnings`
[00:06:15] 
[00:06:15] error: aborting due to previous error
[00:06:15] 
