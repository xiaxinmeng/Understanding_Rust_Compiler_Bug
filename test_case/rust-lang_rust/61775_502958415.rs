plain
travis_time:end:01baf860:start=1560831084735326890,finish=1560831087091499623,duration=2356172733
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:05:45]     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:47]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:05:47]     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
[00:06:39]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:06:48] error[E0277]: the trait bound `std::sync::Arc<std::vec::Vec<&ty::sty::RegionKind>>: ty::context::Lift<'_>` is not satisfied
[00:06:48]    --> src/librustc/macros.rs:268:36
[00:06:48]     |
[00:06:48] 257 | / macro_rules! BraceStructLiftImpl {
[00:06:48] 258 | |     (impl<$($p:tt),*> Lift<$tcx:tt> for $s:path {
[00:06:48] 259 | |         type Lifted = $lifted:ty;
[00:06:48] 260 | |         $($field:ident),* $(,)?
[00:06:48] ...   |
[00:06:48] 268 | |                 $(let $field = tcx.lift(&self.$field)?;)*
[00:06:48]     | |                                    ^^^^ the trait `ty::context::Lift<'_>` is not implemented for `std::sync::Arc<std::vec::Vec<&ty::sty::RegionKind>>`
[00:06:48] 272 | |     };
[00:06:48] 273 | | }
[00:06:48] 273 | | }
[00:06:48]     | |_- in this expansion of `BraceStructLiftImpl!`
[00:06:48]    ::: src/librustc/infer/region_constraints/mod.rs:178:1
[00:06:48]     |
[00:06:48]     |
[00:06:48] 178 | / BraceStructLiftImpl! {
[00:06:48] 179 | |     impl<'a, 'tcx> Lift<'tcx> for PickConstraint<'a> {
[00:06:48] 180 | |         type Lifted = PickConstraint<'tcx>;
[00:06:48] 181 | |         opaque_type_def_id, definition_span, hidden_ty, pick_region, option_regions
[00:06:48] 183 | | }
[00:06:48]     | |_- in this macro invocation
[00:06:48] 
[00:06:48] 
[00:06:48] error[E0277]: the trait bound `std::sync::Arc<std::vec::Vec<&ty::sty::RegionKind>>: ty::fold::TypeFoldable<'_>` is not satisfied
[00:06:48]    --> src/librustc/macros.rs:340:32
[00:06:48]     |
[00:06:48] 328 | / macro_rules! BraceStructTypeFoldableImpl {
[00:06:48] 329 | |     (impl<$($p:tt),*> TypeFoldable<$tcx:tt> for $s:path {
[00:06:48] 330 | |         $($field:ident),* $(,)?
[00:06:48] 331 | |     } $(where $($wc:tt)*)*) => {
[00:06:48] ...   |
[00:06:48] 340 | |                 $s { $($field: $crate::ty::fold::TypeFoldable::fold_with($field, folder),)* }
[00:06:48]     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ty::fold::TypeFoldable<'_>` is not implemented for `std::sync::Arc<std::vec::Vec<&ty::sty::RegionKind>>`
[00:06:48] 351 | |     };
[00:06:48] 352 | | }
[00:06:48] 352 | | }
[00:06:48]     | |_- in this expansion of `BraceStructTypeFoldableImpl!`
[00:06:48]    ::: src/librustc/infer/region_constraints/mod.rs:172:1
[00:06:48]     |
[00:06:48]     |
[00:06:48] 172 | / BraceStructTypeFoldableImpl! {
[00:06:48] 173 | |     impl<'tcx> TypeFoldable<'tcx> for PickConstraint<'tcx> {
[00:06:48] 174 | |         opaque_type_def_id, definition_span, hidden_ty, pick_region, option_regions
[00:06:48] 176 | | }
[00:06:48]     | |_- in this macro invocation
[00:06:48]     |
[00:06:48] note: required by `ty::fold::TypeFoldable::fold_with`
[00:06:48] note: required by `ty::fold::TypeFoldable::fold_with`
[00:06:48]    --> src/librustc/ty/fold.rs:49:5
[00:06:48]     |
[00:06:48] 49  |     fn fold_with<F: TypeFolder<'tcx>>(&self, folder: &mut F) -> Self {
[00:06:48] 
[00:06:48] 
[00:06:48] error[E0277]: the trait bound `std::sync::Arc<std::vec::Vec<&ty::sty::RegionKind>>: ty::fold::TypeFoldable<'_>` is not satisfied
[00:06:48]    --> src/librustc/macros.rs:348:28
[00:06:48]     |
[00:06:48] 328 | / macro_rules! BraceStructTypeFoldableImpl {
[00:06:48] 329 | |     (impl<$($p:tt),*> TypeFoldable<$tcx:tt> for $s:path {
[00:06:48] 330 | |         $($field:ident),* $(,)?
[00:06:48] 331 | |     } $(where $($wc:tt)*)*) => {
[00:06:48] ...   |
[00:06:48] 348 | |                 false $(|| $crate::ty::fold::TypeFoldable::visit_with($field, visitor))*
[00:06:48]     | |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ty::fold::TypeFoldable<'_>` is not implemented for `std::sync::Arc<std::vec::Vec<&ty::sty::RegionKind>>`
[00:06:48] 351 | |     };
[00:06:48] 352 | | }
[00:06:48] 352 | | }
[00:06:48]     | |_- in this expansion of `BraceStructTypeFoldableImpl!`
[00:06:48]    ::: src/librustc/infer/region_constraints/mod.rs:172:1
[00:06:48]     |
[00:06:48]     |
[00:06:48] 172 | / BraceStructTypeFoldableImpl! {
[00:06:48] 173 | |     impl<'tcx> TypeFoldable<'tcx> for PickConstraint<'tcx> {
[00:06:48] 174 | |         opaque_type_def_id, definition_span, hidden_ty, pick_region, option_regions
[00:06:48] 176 | | }
[00:06:48]     | |_- in this macro invocation
[00:06:48]     |
[00:06:48] note: required by `ty::fold::TypeFoldable::visit_with`
[00:06:48] note: required by `ty::fold::TypeFoldable::visit_with`
[00:06:48]    --> src/librustc/ty/fold.rs:54:5
[00:06:48]     |
[00:06:48] 54  |     fn visit_with<V: TypeVisitor<'tcx>>(&self, visitor: &mut V) -> bool {
[00:06:48] 
[00:07:07] error: aborting due to 3 previous errors
[00:07:07] 
[00:07:07] For more information about this error, try `rustc --explain E0277`.
---
travis_time:end:1a47bdfc:start=1560831528084356615,finish=1560831528090572817,duration=6216202
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00ea10bd
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0410c3a8
travis_time:start:0410c3a8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02ab2236
$ dmesg | grep -i kill
