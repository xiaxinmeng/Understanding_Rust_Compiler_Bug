plain
travis_time:end:15fb308e:start=1561204356208300142,finish=1561204445234571845,duration=89026271703
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:26:41]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:26:41]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:26:41]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:26:42]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:27:00] error: the feature `tbm_target_feature` has been stable since 1.37.0 and no longer requires an attribute to enable
[00:27:00]     |
[00:27:00] 110 | #![feature(tbm_target_feature)]
[00:27:00]     |            ^^^^^^^^^^^^^^^^^^
[00:27:00]     |
[00:27:00]     |
[00:27:00]     = note: `-D stable-features` implied by `-D warnings`
[00:27:00] 
[00:27:00] error: the feature `sse4a_target_feature` has been stable since 1.37.0 and no longer requires an attribute to enable
[00:27:00]     |
[00:27:00] 111 | #![feature(sse4a_target_feature)]
[00:27:00]     |            ^^^^^^^^^^^^^^^^^^^^
[00:27:00] 
[00:27:00] 
[00:27:00] error: the feature `adx_target_feature` has been stable since 1.37.0 and no longer requires an attribute to enable
[00:27:00]     |
[00:27:00] 127 | #![feature(adx_target_feature)]
[00:27:00]     |            ^^^^^^^^^^^^^^^^^^
[00:27:00] 
