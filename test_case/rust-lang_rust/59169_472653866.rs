plain
travis_time:end:1af189e8:start=1552520069455033198,finish=1552520144729783184,duration=75274749986
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:30:48]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:30:49] error: incorrect close delimiter: `)`
[00:30:49]     --> src/librustc/session/config.rs:3280:82
[00:30:49]      |
[00:30:49] 3280 |         opts.debugging_opts.allow_features = Some(vec![String::from("never_type"));
[00:30:49]      |                                                  |    |
[00:30:49]      |                                                  |    un-closed delimiter
[00:30:49]      |                                                  close delimiter possibly meant for this
[00:30:49] 
