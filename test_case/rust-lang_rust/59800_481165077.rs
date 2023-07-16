plain
travis_time:end:0ea187aa:start=1554797068584566238,finish=1554797071063024404,duration=2478458166
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:50:51]    Compiling tempfile v3.0.5
[00:50:53]    Compiling parking_lot v0.7.1
[00:50:54]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:53:05]    Compiling rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
[00:53:06] error: cannot satisfy dependencies so `getopts` only shows up once
[00:53:06]   |
[00:53:06]   = help: having upstream crates all available in one format will likely make this go away
[00:53:06] error: aborting due to previous error
[00:53:06] 
[00:53:06] error: Could not compile `rustdoc-tool`.
[00:53:06] 
