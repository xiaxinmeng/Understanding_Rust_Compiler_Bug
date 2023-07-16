plain
travis_time:end:17178c00:start=1555666307213947243,finish=1555666307974453291,duration=760506048
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:08:18]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:13:31]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:13:31]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:31]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:13:38] error[E0609]: no field `sse4a_target_feature` on type `std::rc::Rc<syntax::feature_gate::Features>`
[00:13:38]     --> src/librustc_typeck/collect.rs:2429:63
[00:13:38]      |
[00:13:38] 2429 |                 Some("sse4a_target_feature") => rust_features.sse4a_target_feature,
[00:13:38] 
[00:13:38] 
[00:13:38] error[E0609]: no field `tbm_target_feature` on type `std::rc::Rc<syntax::feature_gate::Features>`
[00:13:38]     --> src/librustc_typeck/collect.rs:2430:61
[00:13:38]      |
[00:13:38] 2430 |                 Some("tbm_target_feature") => rust_features.tbm_target_feature,
[00:13:38] 
[00:13:38] 
[00:13:38] error[E0609]: no field `adx_target_feature` on type `std::rc::Rc<syntax::feature_gate::Features>`
[00:13:38]     --> src/librustc_typeck/collect.rs:2433:61
[00:13:38]      |
[00:13:38] 2433 |                 Some("adx_target_feature") => rust_features.adx_target_feature,
[00:13:38] 
[00:13:39] error: aborting due to 3 previous errors
[00:13:39] 
[00:13:39] For more information about this error, try `rustc --explain E0609`.
---
159652 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
157492 ./obj/build/bootstrap/debug/incremental
156496 ./src/llvm-project/clang
142508 ./obj/build/bootstrap/debug/incremental/bootstrap-hfsog967tquc
142504 ./obj/build/bootstrap/debug/incremental/bootstrap-hfsog967tquc/s-fbfrbdhss0-4y1omn-3k7gym5px36ja
108532 ./src/llvm-project/lldb
100608 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
100604 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
98060 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
