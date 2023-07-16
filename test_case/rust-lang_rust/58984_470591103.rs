plain
travis_time:end:17679444:start=1551975114878313651,finish=1551975115900423181,duration=1022109530
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:06:21]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:06:24]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:06:25]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:29]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:30] error: unused variable: `err_as_bug`
[00:06:30]     |
[00:06:30]     |
[00:06:30] 660 |         let err_as_bug = self.flags.treat_err_as_bug.unwrap_or(0);
[00:06:30]     |             ^^^^^^^^^^ help: consider prefixing with an underscore: `_err_as_bug`
[00:06:30]     = note: `-D unused-variables` implied by `-D warnings`
[00:06:30] 
[00:06:30] error: aborting due to previous error
[00:06:30] 
