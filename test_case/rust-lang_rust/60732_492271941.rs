plain
travis_time:end:15deab00:start=1557845082264275994,finish=1557845084734265316,duration=2469989322
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:06:41]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:06:54]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:05]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:07:09]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:19] error[E0425]: cannot find value `arbitrary_enum_discriminant` in module `sym`
[00:07:19]    --> src/libsyntax/feature_gate.rs:558:14
[00:07:19]     |
[00:07:19] 558 |     (active, arbitrary_enum_discriminant, "1.36.0", Some(60553), None),
[00:07:19]     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `sym`
[00:07:19] 
[00:07:19] error[E0425]: cannot find value `arbitrary_enum_discriminant` in module `sym`
[00:07:19]     --> src/libsyntax/feature_gate.rs:2016:25
[00:07:19] 2016 |                         arbitrary_enum_discriminant,
[00:07:19]      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `sym`
[00:07:19] 
[00:07:19] 
[00:07:19] error[E0425]: cannot find value `arbitrary_enum_discriminant` in module `sym`
[00:07:19]     --> src/libsyntax/feature_gate.rs:2028:29
[00:07:19] 2028 |                             arbitrary_enum_discriminant,
[00:07:19]      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `sym`
[00:07:19] 
