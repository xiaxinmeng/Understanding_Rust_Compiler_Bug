plain
travis_time:end:0dd86c36:start=1558502345786188854,finish=1558502434527415879,duration=88741227025
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:35]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:48] error[E0061]: this function takes 6 parameters but 5 parameters were supplied
[00:07:48]    --> src/libsyntax/parse/mod.rs:353:5
[00:07:48]     |
[00:07:48] 353 |       Parser::new(sess, stream, Some(base_dir), true, false)
[00:07:48]     | 
[00:07:48]    ::: src/libsyntax/parse/parser.rs:536:5
[00:07:48]     |
[00:07:48] 536 | /     pub fn new(
[00:07:48] 536 | /     pub fn new(
[00:07:48] 537 | |         sess: &'a ParseSess,
[00:07:48] 538 | |         tokens: TokenStream,
[00:07:48] 539 | |         directory: Option<Directory<'a>>,
[00:07:48] 590 | |         parser
[00:07:48] 591 | |     }
[00:07:48]     | |_____- defined here
[00:07:48] 
