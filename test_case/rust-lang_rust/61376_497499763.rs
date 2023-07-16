plain
travis_time:end:0e983cb8:start=1559252928788958470,finish=1559252929616764885,duration=827806415
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:05:04]    Compiling backtrace v0.3.25
[00:05:07] error[E0539]: incorrect meta item
[00:05:07]    --> src/libcore/ops/range.rs:711:42
[00:05:07]     |
[00:05:07] 711 |     #[unstable(feature = "bound_cloned", issue = 61356)]
[00:05:07] 
[00:05:08]    Compiling compiler_builtins v0.1.15
[00:05:08]    Compiling backtrace-sys v0.1.27
[00:05:08]    Compiling cmake v0.1.38
---
[00:05:13]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:05:14] error[E0308]: mismatched types
[00:05:14]    --> src/libcore/ops/range.rs:715:51
[00:05:14]     |
[00:05:14] 715 |             Bound::Included(x) => Bound::Included(x.clone()),
[00:05:14]     |                                                   ^^^^^^^^^ expected type parameter, found &T
[00:05:14]     = note: expected type `T`
[00:05:14]                found type `&T`
[00:05:14] 
[00:05:14] error[E0308]: mismatched types
[00:05:14] error[E0308]: mismatched types
[00:05:14]    --> src/libcore/ops/range.rs:716:51
[00:05:14]     |
[00:05:14] 716 |             Bound::Excluded(x) => Bound::Excluded(x.clone()),
[00:05:14]     |                                                   ^^^^^^^^^ expected type parameter, found &T
[00:05:14]     = note: expected type `T`
[00:05:14]                found type `&T`
[00:05:14] 
[00:05:19] error: aborting due to 3 previous errors
