plain
travis_time:end:1f330945:start=1556607076633602957,finish=1556607162008355166,duration=85374752209
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:12]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:42] error[E0308]: mismatched types
[00:07:42]     --> src/librustc/ty/layout.rs:1634:21
[00:07:42]      |
[00:07:42] 1634 | /                     cx.layout_of(this.ty).to_result().map(|layout| {
[00:07:42] 1635 | |                         assert_eq!(layout.variants, Variants::Single { index });layout
[00:07:42]      | |______________________^ expected enum `std::result::Result`, found struct `rustc_target::abi::TyLayout`
[00:07:42]      |
[00:07:42]      |
[00:07:42]      = note: expected type `std::result::Result<std::result::Result<rustc_target::abi::TyLayout<'tcx, &'tcx ty::TyS<'tcx>>, !>, !>`
[00:07:42]                 found type `std::result::Result<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, <<C as rustc_target::abi::LayoutOf>::TyLayout as ty::layout::MaybeResult<rustc_target::abi::TyLayout<'tcx, &'tcx ty::TyS<'tcx>>>>::Item>`
[00:07:46] error: aborting due to previous error
[00:07:46] 
[00:07:46] For more information about this error, try `rustc --explain E0308`.
Tue, 30 Apr 2019 07:00:37 GMT
---
travis_time:end:068ed6c0:start=1556607638200813248,finish=1556607638205254819,duration=4441571
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08f903f7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:28dc8c50
travis_time:start:28dc8c50
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i3
