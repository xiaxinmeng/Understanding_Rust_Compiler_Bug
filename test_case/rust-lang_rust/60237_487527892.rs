plain
travis_time:end:132611ec:start=1556532561205224736,finish=1556532645711079738,duration=84505855002
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:06:32]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:06:44]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:48]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:55]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:28] error[E0277]: the trait bound `std::result::Result<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, ty::layout::LayoutError<'_>>: ty::layout::MaybeResult<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, !>` is not satisfied
[00:08:28]      |
[00:08:28]      |
[00:08:28] 1246 |                 match layout.field(self, i) {
[00:08:28]      |                              ^^^^^ the trait `ty::layout::MaybeResult<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, !>` is not implemented for `std::result::Result<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, ty::layout::LayoutError<'_>>`
[00:08:28]      = help: the following implementations were found:
[00:08:28]      = help: the following implementations were found:
[00:08:28]                <std::result::Result<T, E> as ty::layout::MaybeResult<T, E>>
[00:08:28]      = note: required because of the requirements on the impl of `rustc_target::abi::TyLayoutMethods<'_, ty::layout::LayoutCx<'_, ty::context::TyCtxt<'_, 'tcx, 'tcx>>>` for `&ty::TyS<'_>`
[00:08:28] 
[00:08:28] error[E0277]: the trait bound `std::result::Result<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, ty::layout::LayoutError<'_>>: ty::layout::MaybeResult<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, !>` is not satisfied
[00:08:28]      |
[00:08:28]      |
[00:08:28] 1313 |                                            layout.for_variant(self, i))
[00:08:28]      |                                                   ^^^^^^^^^^^ the trait `ty::layout::MaybeResult<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, !>` is not implemented for `std::result::Result<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, ty::layout::LayoutError<'_>>`
[00:08:28]      = help: the following implementations were found:
[00:08:28]      = help: the following implementations were found:
[00:08:28]                <std::result::Result<T, E> as ty::layout::MaybeResult<T, E>>
[00:08:28]      = note: required because of the requirements on the impl of `rustc_target::abi::TyLayoutMethods<'_, ty::layout::LayoutCx<'_, ty::context::TyCtxt<'_, 'tcx, 'tcx>>>` for `&ty::TyS<'_>`
[00:08:28] 
[00:08:28] error[E0277]: the trait bound `std::result::Result<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, ty::layout::LayoutError<'_>>: ty::layout::MaybeResult<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, !>` is not satisfied
[00:08:28]      |
[00:08:28]      |
[00:08:28] 1999 |                 return self.find_niche(layout.field(self, 0)?);
[00:08:28]      |                                               ^^^^^ the trait `ty::layout::MaybeResult<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, !>` is not implemented for `std::result::Result<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, ty::layout::LayoutError<'_>>`
[00:08:28]      = help: the following implementations were found:
[00:08:28]      = help: the following implementations were found:
[00:08:28]                <std::result::Result<T, E> as ty::layout::MaybeResult<T, E>>
[00:08:28]      = note: required because of the requirements on the impl of `rustc_target::abi::TyLayoutMethods<'_, ty::layout::LayoutCx<'_, ty::context::TyCtxt<'_, 'tcx, 'tcx>>>` for `&ty::TyS<'_>`
[00:08:28] 
[00:08:28] error[E0277]: the trait bound `std::result::Result<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, ty::layout::LayoutError<'_>>: ty::layout::MaybeResult<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, !>` is not satisfied
[00:08:28]      |
[00:08:28]      |
[00:08:28] 2007 |             if let Some(mut c) = self.find_niche(layout.field(self, i)?)? {
[00:08:28]      |                                                         ^^^^^ the trait `ty::layout::MaybeResult<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, !>` is not implemented for `std::result::Result<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>, ty::layout::LayoutError<'_>>`
[00:08:28]      = help: the following implementations were found:
[00:08:28]      = help: the following implementations were found:
[00:08:28]                <std::result::Result<T, E> as ty::layout::MaybeResult<T, E>>
[00:08:28]      = note: required because of the requirements on the impl of `rustc_target::abi::TyLayoutMethods<'_, ty::layout::LayoutCx<'_, ty::context::TyCtxt<'_, 'tcx, 'tcx>>>` for `&ty::TyS<'_>`
[00:08:32] error: aborting due to 4 previous errors
[00:08:32] 
[00:08:32] For more information about this error, try `rustc --explain E0277`.
[00:08:32] error: Could not compile `rustc`.
---
travis_time:end:0134fd90:start=1556533168379644068,finish=1556533168383790814,duration=4146746
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b8940e0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3cbd0073
travis_time:start:3cbd0073
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i
