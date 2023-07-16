plain
travis_time:end:2b88a504:start=1560870057883435257,finish=1560871204604813905,duration=1146721378648
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:13:49]     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:13:50]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:13:50]     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
[00:14:42]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:14:51] error[E0277]: the trait bound `std::sync::Arc<std::vec::Vec<&ty::sty::RegionKind>>: ty::fold::TypeFoldable<'_>` is not satisfied
[00:14:51]    --> src/librustc/macros.rs:340:32
[00:14:51]     |
[00:14:51] 328 | / macro_rules! BraceStructTypeFoldableImpl {
[00:14:51] 329 | |     (impl<$($p:tt),*> TypeFoldable<$tcx:tt> for $s:path {
[00:14:51] 330 | |         $($field:ident),* $(,)?
[00:14:51] 331 | |     } $(where $($wc:tt)*)*) => {
[00:14:51] ...   |
[00:14:51] 340 | |                 $s { $($field: $crate::ty::fold::TypeFoldable::fold_with($field, folder),)* }
[00:14:51]     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ty::fold::TypeFoldable<'_>` is not implemented for `std::sync::Arc<std::vec::Vec<&ty::sty::RegionKind>>`
[00:14:51] 351 | |     };
[00:14:51] 352 | | }
[00:14:51] 352 | | }
[00:14:51]     | |_- in this expansion of `BraceStructTypeFoldableImpl!`
[00:14:51]    ::: src/librustc/infer/region_constraints/mod.rs:172:1
[00:14:51]     |
[00:14:51]     |
[00:14:51] 172 | / BraceStructTypeFoldableImpl! {
[00:14:51] 173 | |     impl<'tcx> TypeFoldable<'tcx> for MemberConstraint<'tcx> {
[00:14:51] 174 | |         opaque_type_def_id, definition_span, hidden_ty, member_region, choice_regions
[00:14:51] 176 | | }
[00:14:51]     | |_- in this macro invocation
[00:14:51]     |
[00:14:51] note: required by `ty::fold::TypeFoldable::fold_with`
[00:14:51] note: required by `ty::fold::TypeFoldable::fold_with`
[00:14:51]    --> src/librustc/ty/fold.rs:49:5
[00:14:51]     |
[00:14:51] 49  |     fn fold_with<F: TypeFolder<'tcx>>(&self, folder: &mut F) -> Self {
[00:14:51] 
[00:14:51] 
[00:14:51] error[E0277]: the trait bound `std::sync::Arc<std::vec::Vec<&ty::sty::RegionKind>>: ty::fold::TypeFoldable<'_>` is not satisfied
[00:14:51]    --> src/librustc/macros.rs:348:28
[00:14:51]     |
[00:14:51] 328 | / macro_rules! BraceStructTypeFoldableImpl {
[00:14:51] 329 | |     (impl<$($p:tt),*> TypeFoldable<$tcx:tt> for $s:path {
[00:14:51] 330 | |         $($field:ident),* $(,)?
[00:14:51] 331 | |     } $(where $($wc:tt)*)*) => {
[00:14:51] ...   |
[00:14:51] 348 | |                 false $(|| $crate::ty::fold::TypeFoldable::visit_with($field, visitor))*
[00:14:51]     | |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ty::fold::TypeFoldable<'_>` is not implemented for `std::sync::Arc<std::vec::Vec<&ty::sty::RegionKind>>`
[00:14:51] 351 | |     };
[00:14:51] 352 | | }
[00:14:51] 352 | | }
[00:14:51]     | |_- in this expansion of `BraceStructTypeFoldableImpl!`
[00:14:51]    ::: src/librustc/infer/region_constraints/mod.rs:172:1
[00:14:51]     |
[00:14:51]     |
[00:14:51] 172 | / BraceStructTypeFoldableImpl! {
[00:14:51] 173 | |     impl<'tcx> TypeFoldable<'tcx> for MemberConstraint<'tcx> {
[00:14:51] 174 | |         opaque_type_def_id, definition_span, hidden_ty, member_region, choice_regions
[00:14:51] 176 | | }
[00:14:51]     | |_- in this macro invocation
[00:14:51]     |
[00:14:51] note: required by `ty::fold::TypeFoldable::visit_with`
[00:14:51] note: required by `ty::fold::TypeFoldable::visit_with`
[00:14:51]    --> src/librustc/ty/fold.rs:54:5
[00:14:51]     |
[00:14:51] 54  |     fn visit_with<V: TypeVisitor<'tcx>>(&self, visitor: &mut V) -> bool {
[00:14:51] 
[00:15:10] error: aborting due to 2 previous errors
[00:15:10] 
[00:15:10] For more information about this error, try `rustc --explain E0277`.
---
travis_time:end:331b7f38:start=1560872127783469791,finish=1560872127788407801,duration=4938010
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:281a7a06
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ee76e7c
travis_time:start:0ee76e7c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01f55d44
$ dmesg | grep -i kill
