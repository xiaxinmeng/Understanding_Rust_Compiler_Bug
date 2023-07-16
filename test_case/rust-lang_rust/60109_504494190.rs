plain
travis_time:end:014cd93c:start=1561134690888808494,finish=1561134691639879711,duration=751071217
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:13]    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
[00:08:20]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:13:54]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:54]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:14:00] error[E0609]: no field `sse4a_target_feature` on type `&syntax::feature_gate::Features`
[00:14:00]     --> src/librustc_typeck/collect.rs:2365:66
[00:14:00]      |
[00:14:00] 2365 |                 Some(sym::sse4a_target_feature) => rust_features.sse4a_target_feature,
[00:14:00] 
[00:14:00] 
[00:14:00] error[E0609]: no field `tbm_target_feature` on type `&syntax::feature_gate::Features`
[00:14:00]     --> src/librustc_typeck/collect.rs:2366:64
[00:14:00]      |
[00:14:00] 2366 |                 Some(sym::tbm_target_feature) => rust_features.tbm_target_feature,
[00:14:00] 
[00:14:00] 
[00:14:00] error[E0609]: no field `adx_target_feature` on type `&syntax::feature_gate::Features`
[00:14:00]     --> src/librustc_typeck/collect.rs:2369:64
[00:14:00]      |
[00:14:00] 2369 |                 Some(sym::adx_target_feature) => rust_features.adx_target_feature,
[00:14:00] 
[00:14:01] error: aborting due to 3 previous errors
[00:14:01] 
[00:14:01] For more information about this error, try `rustc --explain E0609`.
---
travis_time:end:225e24b0:start=1561135768973833522,finish=1561135768979922448,duration=6088926
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2ce0c726
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:072ee998
travis_time:start:072ee998
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:034ba7d6
$ dmesg | grep -i kill
