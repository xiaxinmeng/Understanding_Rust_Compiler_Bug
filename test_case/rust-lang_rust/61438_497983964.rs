plain
travis_time:end:3b1dbe19:start=1559428824735613408,finish=1559428914487778542,duration=89752165134
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:14:39]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:14:46] error[E0308]: mismatched types
[00:14:46]    --> src/librustc_typeck/check/compare_method.rs:617:44
[00:14:46]     |
[00:14:46] 617 |                                 synthetic: hir::SyntheticTyParamKind::ImplTrait,
[00:14:46]     |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found enum `rustc::hir::SyntheticTyParamKind`
[00:14:46]     |
[00:14:46]     = note: expected type `std::option::Option<rustc::hir::SyntheticTyParamKind>`
[00:14:46]                found type `rustc::hir::SyntheticTyParamKind`
[00:14:46] error[E0027]: pattern does not mention field `default`
[00:14:46]    --> src/librustc_typeck/check/compare_method.rs:616:29
[00:14:46]     |
[00:14:46] 616 | /                             GenericParamKind::Type {
[00:14:46] 616 | /                             GenericParamKind::Type {
[00:14:46] 617 | |                                 synthetic: hir::SyntheticTyParamKind::ImplTrait,
[00:14:46] 618 | |                             } => Some(p.span),
[00:14:46]     | |_____________________________^ missing field `default`
[00:14:46] error[E0308]: mismatched types
[00:14:46]    --> src/librustc_typeck/check/compare_method.rs:632:36
[00:14:46]     |
[00:14:46]     |
[00:14:46] 632 |                         synthetic: hir::SyntheticTyParamKind::ImplTrait,
[00:14:46]     |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found enum `rustc::hir::SyntheticTyParamKind`
[00:14:46]     |
[00:14:46]     = note: expected type `std::option::Option<rustc::hir::SyntheticTyParamKind>`
[00:14:46]                found type `rustc::hir::SyntheticTyParamKind`
[00:14:46] error[E0027]: pattern does not mention field `default`
[00:14:46]    --> src/librustc_typeck/check/compare_method.rs:631:21
[00:14:46]     |
[00:14:46] 631 | /                     GenericParamKind::Type {
[00:14:46] 631 | /                     GenericParamKind::Type {
[00:14:46] 632 | |                         synthetic: hir::SyntheticTyParamKind::ImplTrait,
[00:14:46] 633 | |                     } => Some(p.span),
[00:14:46]     | |_____________________^ missing field `default`
[00:14:47] error: aborting due to 4 previous errors
[00:14:47] 
[00:14:47] Some errors have detailed explanations: E0027, E0308.
[00:14:47] For more information about an error, try `rustc --explain E0027`.
---
19256 ./src/llvm-project/lldb/www/cpp_reference
19252 ./src/llvm-project/lldb/www/cpp_reference/html
travis_time:end:0c961dca:start=1559430063887415004,finish=1559430064486770474,duration=599355470
travis_fold:end:after_failure.1
travis_fold:/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:19b4579c
$ dmesg | grep -i kill
