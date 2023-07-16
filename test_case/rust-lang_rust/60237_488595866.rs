plain
travis_time:end:0b19f870:start=1556786471469241895,finish=1556786562520184786,duration=91050942891
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:48]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:19] error[E0308]: mismatched types
[00:08:19]     --> src/librustc/ty/layout.rs:1693:32
[00:08:19]      |
[00:08:19] 1657 |     fn field(this: TyLayout<'tcx>, cx: &C, i: usize) -> C::TyLayout {
[00:08:19]      |                                                         ----------- expected `<C as rustc_target::abi::LayoutOf>::TyLayout` because of return type
[00:08:19] ...
[00:08:19] 1693 |                         return ptr_layout;
[00:08:19]      |                                ^^^^^^^^^^ expected associated type, found struct `rustc_target::abi::TyLayout`
[00:08:19]      |
[00:08:19]      = note: expected type `<C as rustc_target::abi::LayoutOf>::TyLayout`
[00:08:19]                 found type `rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>`
[00:08:23] error: aborting due to previous error
[00:08:23] 
[00:08:23] For more information about this error, try `rustc --explain E0308`.
[00:08:23] error: Could not compile `rustc`.
---
travis_time:end:204b235a:start=1556787076116641990,finish=1556787076121473966,duration=4831976
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04ec3c8a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1577ead9
travis_time:start:1577ead9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:082fdfca
$ dmesg | grep -i kill
