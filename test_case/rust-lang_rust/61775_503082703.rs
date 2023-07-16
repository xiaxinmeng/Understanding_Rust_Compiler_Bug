plain
travis_time:end:0ea8be7e:start=1560860209111692248,finish=1560860296966330249,duration=87854638001
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:08:00]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:04] error[E0433]: failed to resolve: use of undeclared type or module `Arc`
[00:08:04]    --> src/librustc/ty/structural_impls.rs:391:31
[00:08:04]     |
[00:08:04] 391 |         tcx.lift(&**self).map(Arc::new)
[00:08:04]     |                               ^^^ use of undeclared type or module `Arc`
[00:08:05] error[E0412]: cannot find type `Arc` in this scope
[00:08:05]    --> src/librustc/ty/structural_impls.rs:388:42
[00:08:05]     |
[00:08:05]     |
[00:08:05] 388 | impl<'tcx, T: Lift<'tcx>> Lift<'tcx> for Arc<T> {
[00:08:05] help: possible candidate is found in another module, you can import it into scope
[00:08:05]     |
[00:08:05] 6   | use std::sync::Arc;
[00:08:05]     |
[00:08:05]     |
[00:08:05] 
[00:08:05] error[E0412]: cannot find type `Arc` in this scope
[00:08:05]    --> src/librustc/ty/structural_impls.rs:389:19
[00:08:05]     |
[00:08:05] 389 |     type Lifted = Arc<T::Lifted>;
[00:08:05] help: possible candidate is found in another module, you can import it into scope
[00:08:05]     |
[00:08:05] 6   | use std::sync::Arc;
[00:08:05]     |
[00:08:05]     |
[00:08:05] 
[00:08:09] error[E0207]: the type parameter `T` is not constrained by the impl trait, self type, or predicates
[00:08:09]    --> src/librustc/ty/structural_impls.rs:388:12
[00:08:09]     |
[00:08:09] 388 | impl<'tcx, T: Lift<'tcx>> Lift<'tcx> for Arc<T> {
[00:08:09]     |            ^ unconstrained type parameter
[00:08:09] error: aborting due to 4 previous errors
[00:08:09] 
[00:08:09] Some errors have detailed explanations: E0207, E0412, E0433.
[00:08:09] For more information about an error, try `rustc --explain E0207`.
---
travis_time:end:05432bc8:start=1560860809730996033,finish=1560860809736102999,duration=5106966
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c427929
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04916f7d
travis_time:start:04916f7d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:164bd8a7
$ dmesg | grep -i kill
