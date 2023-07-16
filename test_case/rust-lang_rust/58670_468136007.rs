plain
travis_time:end:02863380:start=1551327919421466430,finish=1551328006850479333,duration=87429012903
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:18:31]    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
[00:18:31]    Compiling rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
[00:19:32]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:19:32]    Compiling rustc_borrowck v0.0.0 (/checkout/src/librustc_borrowck)
[00:19:33] error[E0599]: no method named `cast_kinds` found for type `&'a rustc::ty::TypeckTables<'tcx>` in the current scope
[00:19:33]    --> src/librustc_passes/rvalue_promotion.rs:322:28
[00:19:33]     |
[00:19:33] 322 |             match v.tables.cast_kinds().get(from.hir_id) {
[00:19:33] 
[00:19:34] error: aborting due to previous error
[00:19:34] 
[00:19:34] For more information about this error, try `rustc --explain E0599`.
---
travis_time:end:2c1f1ec0:start=1551329207577928757,finish=1551329207583123096,duration=5194339
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0bf32635
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0026290b
travis_time:start:0026290b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: N
