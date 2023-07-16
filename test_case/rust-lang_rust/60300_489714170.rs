plain
travis_time:end:0b529774:start=1557164028516611654,finish=1557164116066617906,duration=87550006252
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:14:49]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:14:49]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:14:49]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:20:20]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:20:20] error: expected one of `::`, `=`, or `|`, found `!=`
[00:20:20]     |
[00:20:20]     |
[00:20:20] 535 |             if let AdtKind::Struct != field_def.adt_kind() {
[00:20:20]     |                                    ^^ expected one of `::`, `=`, or `|` here
[00:20:21]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:20:22] error: aborting due to previous error
[00:20:22] 
[00:20:22] error: Could not compile `rustc_lint`.
