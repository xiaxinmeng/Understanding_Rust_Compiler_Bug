plain
travis_time:end:01b349a1:start=1559431636120147225,finish=1559431727150626560,duration=91030479335
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:14:57]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:15:03] error[E0308]: mismatched types
[00:15:03]    --> src/librustc_typeck/check/compare_method.rs:617:44
[00:15:03]     |
[00:15:03] 617 |                                 synthetic: hir::SyntheticTyParamKind::ImplTrait, ..
[00:15:03]     |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found enum `rustc::hir::SyntheticTyParamKind`
[00:15:03]     |
[00:15:03]     = note: expected type `std::option::Option<rustc::hir::SyntheticTyParamKind>`
[00:15:03]                found type `rustc::hir::SyntheticTyParamKind`
[00:15:03] error[E0308]: mismatched types
[00:15:03]    --> src/librustc_typeck/check/compare_method.rs:632:36
[00:15:03]     |
[00:15:03]     |
[00:15:03] 632 |                         synthetic: hir::SyntheticTyParamKind::ImplTrait, ..
[00:15:03]     |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found enum `rustc::hir::SyntheticTyParamKind`
[00:15:03]     |
[00:15:03]     = note: expected type `std::option::Option<rustc::hir::SyntheticTyParamKind>`
[00:15:03]                found type `rustc::hir::SyntheticTyParamKind`
[00:15:04] error: aborting due to 2 previous errors
[00:15:04] 
[00:15:04] For more information about this error, try `rustc --explain E0308`.
[00:15:04] error: Could not compile `rustc_typeck`.
