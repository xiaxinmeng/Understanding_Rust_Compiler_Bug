plain
travis_time:end:23800e72:start=1556384972365871248,finish=1556384974736935568,duration=2371064320
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:58]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:36] error: lifetime may not live long enough
[00:08:36]    --> src/librustc/macros.rs:268:32
[00:08:36]     |
[00:08:36] 257 | / macro_rules! BraceStructLiftImpl {
[00:08:36] 258 | |     (impl<$($p:tt),*> Lift<$tcx:tt> for $s:path {
[00:08:36] 259 | |         type Lifted = $lifted:ty;
[00:08:36] 260 | |         $($field:ident),* $(,)?
[00:08:36] ...   |
[00:08:36] 268 | |                 $(let $field = tcx.lift(&self.$field)?;)*
[00:08:36]     | |                                ^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'tcx`
[00:08:36] 272 | |     };
[00:08:36] 273 | | }
[00:08:36] 273 | | }
[00:08:36]     | |_- in this expansion of `BraceStructLiftImpl!`
[00:08:36]    ::: src/librustc/ty/subst.rs:737:1
[00:08:36]     |
[00:08:36]     |
[00:08:36] 737 | / BraceStructLiftImpl! {
[00:08:36] 738 | |     impl<'a, 'tcx> Lift<'tcx> for UserSubsts<'a> {
[00:08:36]     | |          --  ---- lifetime `'tcx` defined here
[00:08:36]     | |          lifetime `'a` defined here
[00:08:36]     | |          lifetime `'a` defined here
[00:08:36] 739 | |         type Lifted = UserSubsts<'tcx>;
[00:08:36] 741 | |         user_self_ty,
[00:08:36] 742 | |     }
[00:08:36] 743 | | }
[00:08:36]     | |_- in this macro invocation
[00:08:36]     | |_- in this macro invocation
[00:08:36] 
[00:08:36] error: lifetime may not live long enough
[00:08:36]    --> src/librustc/macros.rs:268:32
[00:08:36]     |
[00:08:36] 257 | / macro_rules! BraceStructLiftImpl {
[00:08:36] 258 | |     (impl<$($p:tt),*> Lift<$tcx:tt> for $s:path {
[00:08:36] 259 | |         type Lifted = $lifted:ty;
[00:08:36] 260 | |         $($field:ident),* $(,)?
[00:08:36] ...   |
[00:08:36] 268 | |                 $(let $field = tcx.lift(&self.$field)?;)*
[00:08:36]     | |                                ^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'tcx`
[00:08:36] 272 | |     };
[00:08:36] 273 | | }
[00:08:36] 273 | | }
[00:08:36]     | |_- in this expansion of `BraceStructLiftImpl!`
[00:08:36]    ::: src/librustc/ty/structural_impls.rs:777:1
[00:08:36]     |
[00:08:36]     |
[00:08:36] 777 | / BraceStructLiftImpl! {
[00:08:36] 778 | |     impl<'a, 'tcx> Lift<'tcx> for ty::Instance<'a> {
[00:08:36]     | |          --  ---- lifetime `'tcx` defined here
[00:08:36]     | |          lifetime `'a` defined here
[00:08:36] 779 | |         type Lifted = ty::Instance<'tcx>;
[00:08:36] 780 | |         def, substs
[00:08:36] 781 | |     }
[00:08:36] 781 | |     }
[00:08:36] 782 | | }
[00:08:36]     | |_- in this macro invocation
[00:08:36] 
[00:08:44] error: lifetime may not live long enough
[00:08:44]     --> src/librustc/mir/mod.rs:2540:42
[00:08:44]      |
[00:08:44] 2475 | impl<'tcx> Debug for Rvalue<'tcx> {
[00:08:44]      |      ---- lifetime `'tcx` defined here
[00:08:44] ...
[00:08:44] 2540 |                             let substs = tcx.lift(&substs).expect("could not lift for printing");
[00:08:44]      |                                          ^^^^^^^^^^^^^^^^^ argument requires that `'tcx` must outlive `'static`
[00:08:46] error: lifetime may not live long enough
[00:08:46] error: lifetime may not live long enough
[00:08:46]    --> src/librustc/traits/fulfill.rs:467:31
[00:08:46]     |
[00:08:46] 237 | impl<'a, 'b, 'gcx, 'tcx> ObligationProcessor for FulfillProcessor<'a, 'b, 'gcx, 'tcx> {
[00:08:46]     |              ----  ---- lifetime `'tcx` defined here
[00:08:46]     |              |
[00:08:46]     |              lifetime `'gcx` defined here
[00:08:46] ...
[00:08:46] 467 |                         match self.selcx.tcx().lift_to_global(&substs) {
[00:08:46]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'tcx` must outlive `'gcx`
[00:08:46] error: lifetime may not live long enough
[00:08:46]    --> src/librustc/traits/project.rs:420:43
[00:08:46]     |
[00:08:46]     |
[00:08:46] 324 | impl<'a, 'b, 'gcx, 'tcx> TypeFolder<'gcx, 'tcx> for AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> {
[00:08:46]     |              ----  ---- lifetime `'tcx` defined here
[00:08:46]     |              |
[00:08:46]     |              lifetime `'gcx` defined here
[00:08:46] ...
[00:08:46] 420 |                     if let Some(substs) = self.tcx().lift_to_global(&substs) {
[00:08:46]     |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'tcx` must outlive `'gcx`
[00:08:46] error: lifetime may not live long enough
[00:08:46]    --> src/librustc/traits/select.rs:783:23
[00:08:46]     |
[00:08:46]     |
[00:08:46] 462 | impl<'cx, 'gcx, 'tcx> SelectionContext<'cx, 'gcx, 'tcx> {
[00:08:46]     |           ----  ---- lifetime `'tcx` defined here
[00:08:46]     |           |
[00:08:46]     |           lifetime `'gcx` defined here
[00:08:46] ...
[00:08:46] 783 |                 match tcx.lift_to_global(&(obligation.param_env, substs)) {
[00:08:46]     |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'tcx` must outlive `'gcx`
[00:08:47] error: lifetime may not live long enough
[00:08:47]    --> src/librustc/traits/specialize/mod.rs:129:26
[00:08:47]     |
[00:08:47] 112 | pub fn find_associated_item<'a, 'tcx>(
[00:08:47] 112 | pub fn find_associated_item<'a, 'tcx>(
[00:08:47]     |                             --  ---- lifetime `'tcx` defined here
[00:08:47]     |                             |
[00:08:47]     |                             lifetime `'a` defined here
[00:08:47] ...
[00:08:47] 129 |             let substs = tcx.infer_ctxt().enter(|infcx| {
[00:08:47]     |                          ^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'tcx`
[00:08:47] error[E0716]: temporary value dropped while borrowed
[00:08:47]    --> src/librustc/traits/specialize/mod.rs:129:26
[00:08:47]     |
[00:08:47]     |
[00:08:47] 112 |   pub fn find_associated_item<'a, 'tcx>(
[00:08:47]     |                                   ---- lifetime `'tcx` defined here
[00:08:47] ...
[00:08:47] 129 |               let substs = tcx.infer_ctxt().enter(|infcx| {
[00:08:47]     |                            -^^^^^^^^^^^^^^^
[00:08:47]     |  __________________________creates a temporary which is freed while still in use
[00:08:47]     | |
[00:08:47]     | |
[00:08:47] 130 | |                 let param_env = param_env.with_reveal_all();
[00:08:47] 131 | |                 let substs = substs.rebase_onto(tcx, trait_def_id, impl_data.substs);
[00:08:47] 132 | |                 let substs = translate_substs(&infcx, param_env, impl_data.impl_def_id,
[00:08:47] 139 | |                 )
[00:08:47] 140 | |             });
[00:08:47]     | |              -- temporary value is freed at the end of this statement
[00:08:47]     | |______________|
[00:08:47]     | |______________|
[00:08:47]     |                argument requires that borrow lasts for `'tcx`
[00:08:47] 
[00:08:47] error: lifetime may not live long enough
[00:08:47]    --> src/librustc/traits/structural_impls.rs:586:19
[00:08:47]     |
[00:08:47] 578 | impl<'a, 'tcx> Lift<'tcx> for traits::Vtable<'a, ()> {
[00:08:47]     |      --  ---- lifetime `'tcx` defined here
[00:08:47]     |      lifetime `'a` defined here
[00:08:47] ...
[00:08:47] ...
[00:08:47] 586 |             }) => tcx.lift(&substs).map(|substs|
[00:08:47]     |                   ^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'tcx`
[00:08:47] error: lifetime may not live long enough
[00:08:47]    --> src/librustc/traits/query/normalize.rs:204:42
[00:08:47]     |
[00:08:47]     |
[00:08:47] 85  | impl<'cx, 'gcx, 'tcx> TypeFolder<'gcx, 'tcx> for QueryNormalizer<'cx, 'gcx, 'tcx> {
[00:08:47]     |           ----  ---- lifetime `'tcx` defined here
[00:08:47]     |           |
[00:08:47]     |           lifetime `'gcx` defined here
[00:08:47] ...
[00:08:47] 204 |                             let substs = tcx.lift_to_global(&substs).unwrap();
[00:08:47]     |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'tcx` must outlive `'gcx`
[00:08:47] 
[00:08:47] error[E0515]: cannot return value referencing local data `closure_substs.substs`
[00:08:47]      |
[00:08:47]      |
[00:08:47] 1451 |                 WalkTysIter::Types(closure_substs.substs.types())
[00:08:47]      |                 |                  |
[00:08:47]      |                 |                  |
[00:08:47]      |                 |                  `closure_substs.substs` is borrowed here
[00:08:47] 
[00:08:47] error[E0515]: cannot return value referencing local variable `substs`
[00:08:47]     --> src/librustc/ty/mod.rs:1454:17
[00:08:47]      |
[00:08:47]      |
[00:08:47] 1454 |                 WalkTysIter::Types(substs.types())
[00:08:47]      |                 |                  |
[00:08:47]      |                 |                  `substs` is borrowed here
[00:08:47]      |                 returns a value referencing data owned by the current function
[00:08:47] 
[00:08:47] 
[00:08:48] error[E0515]: cannot return value referencing function parameter `substs`
[00:08:48]    --> src/librustc/ty/print/mod.rs:202:9
[00:08:48]     |
[00:08:48] 202 |         &substs[own_params]
[00:08:48]     |         ^------^^^^^^^^^^^^
[00:08:48]     |         ||
[00:08:48]     |         |`substs` is borrowed here
[00:08:48] 
[00:08:50] error: lifetime may not live long enough
[00:08:50]    --> src/librustc/ty/relate.rs:474:47
[00:08:50]     |
[00:08:50]     |
[00:08:50] 347 | pub fn super_relate_tys<'a, 'gcx, 'tcx, R>(relation: &mut R,
[00:08:50]     |                             ----  ---- lifetime `'tcx` defined here
[00:08:50]     |                             |
[00:08:50]     |                             lifetime `'gcx` defined here
[00:08:50] ...
[00:08:50] 474 |                         if let Some(substs) = tcx.lift_to_global(&substs) {
[00:08:50]     |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'tcx` must outlive `'gcx`
[00:08:51] error[E0515]: cannot return value referencing local variable `substs`
[00:08:51]    --> src/librustc/ty/walk.rs:107:17
[00:08:51]     |
[00:08:51]     |
[00:08:51] 107 |                 substs.types().rev().chain(opt_ty)
[00:08:51]     |                 |
[00:08:51]     |                 returns a value referencing data owned by the current function
[00:08:51]     |                 `substs` is borrowed here
[00:08:51] 
[00:08:51] 
[00:08:51] error: lifetime may not live long enough
[00:08:51]    --> src/librustc/ty/instance.rs:178:26
[00:08:51]     |
[00:08:51] 175 | impl<'tcx> fmt::Display for Instance<'tcx> {
[00:08:51]     |      ---- lifetime `'tcx` defined here
[00:08:51] ...
[00:08:51] 178 |             let substs = tcx.lift(&self.substs).expect("could not lift for printing");
[00:08:51]     |                          ^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'tcx` must outlive `'static`
[00:08:52] error: lifetime may not live long enough
[00:08:52]    --> src/librustc/ty/structural_impls.rs:420:9
[00:08:52]     |
[00:08:52]     |
[00:08:52] 417 | impl<'a, 'tcx> Lift<'tcx> for ty::TraitRef<'a> {
[00:08:52]     |      --  ---- lifetime `'tcx` defined here
[00:08:52]     |      lifetime `'a` defined here
[00:08:52] ...
[00:08:52] ...
[00:08:52] 420 |         tcx.lift(&self.substs).map(|substs| ty::TraitRef {
[00:08:52]     |         ^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'tcx`
[00:08:52] error: lifetime may not live long enough
[00:08:52]    --> src/librustc/ty/structural_impls.rs:430:9
[00:08:52]     |
[00:08:52]     |
[00:08:52] 427 | impl<'a, 'tcx> Lift<'tcx> for ty::ExistentialTraitRef<'a> {
[00:08:52]     |      --  ---- lifetime `'tcx` defined here
[00:08:52]     |      lifetime `'a` defined here
[00:08:52] ...
[00:08:52] ...
[00:08:52] 430 |         tcx.lift(&self.substs).map(|substs| ty::ExistentialTraitRef {
[00:08:52]     |         ^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'tcx`
[00:08:52] error: lifetime may not live long enough
[00:08:52]    --> src/librustc/ty/structural_impls.rs:487:9
[00:08:52]     |
[00:08:52]     |
[00:08:52] 483 | impl<'a, 'tcx> Lift<'tcx> for ty::ProjectionTy<'a> {
[00:08:52]     |      --  ---- lifetime `'tcx` defined here
[00:08:52]     |      lifetime `'a` defined here
[00:08:52] ...
[00:08:52] ...
[00:08:52] 487 |         tcx.lift(&self.substs).map(|substs| {
[00:08:52]     |         ^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'tcx`
[00:08:52] error: lifetime may not live long enough
[00:08:52]    --> src/librustc/ty/structural_impls.rs:512:9
[00:08:52]     |
[00:08:52]     |
[00:08:52] 509 | impl<'a, 'tcx> Lift<'tcx> for ty::ExistentialProjection<'a> {
[00:08:52]     |      --  ---- lifetime `'tcx` defined here
[00:08:52]     |      lifetime `'a` defined here
[00:08:52] ...
[00:08:52] ...
[00:08:52] 512 |         tcx.lift(&self.substs).map(|substs| {
[00:08:52]     |         ^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'tcx`
[00:08:52] error: lifetime may not live long enough
[00:08:52]    --> src/librustc/ty/structural_impls.rs:554:17
[00:08:52]     |
[00:08:52]     |
[00:08:52] 522 | impl<'a, 'tcx> Lift<'tcx> for ty::Predicate<'a> {
[00:08:52]     |      --  ---- lifetime `'tcx` defined here
[00:08:52]     |      lifetime `'a` defined here
[00:08:52] ...
[00:08:52] ...
[00:08:52] 554 |                 tcx.lift(&substs).map(|substs| {
[00:08:52]     |                 ^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'tcx`
[00:08:52] error: lifetime may not live long enough
[00:08:52]    --> src/librustc/ty/structural_impls.rs:599:9
[00:08:52]     |
[00:08:52]     |
[00:08:52] 596 | impl<'a, 'tcx> Lift<'tcx> for ty::ClosureSubsts<'a> {
[00:08:52]     |      --  ---- lifetime `'tcx` defined here
[00:08:52]     |      lifetime `'a` defined here
[00:08:52] ...
[00:08:52] ...
[00:08:52] 599 |         tcx.lift(&self.substs).map(|substs| {
[00:08:52]     |         ^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'tcx`
[00:08:52] error: lifetime may not live long enough
[00:08:52]    --> src/librustc/ty/structural_impls.rs:608:9
[00:08:52]     |
[00:08:52]     |
[00:08:52] 605 | impl<'a, 'tcx> Lift<'tcx> for ty::GeneratorSubsts<'a> {
[00:08:52]     |      --  ---- lifetime `'tcx` defined here
[00:08:52]     |      lifetime `'a` defined here
[00:08:52] ...
[00:08:52] ...
[00:08:52] 608 |         tcx.lift(&self.substs).map(|substs| {
[00:08:52]     |         ^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'tcx`
[00:08:52] 
[00:08:52] error[E0515]: cannot return value referencing local data `self.substs`
[00:08:52]    --> src/librustc/ty/sty.rs:335:9
[00:08:52]     |
[00:08:52] 335 | /         SplitClosureSubsts {
[00:08:52] 336 | |             closure_kind_ty: self.substs.type_at(parent_len),
[00:08:52] 337 | |             closure_sig_ty: self.substs.type_at(parent_len + 1),
[00:08:52] 338 | |             upvar_kinds: &self.substs[parent_len + 2..],
[00:08:52]     | |                           ----------- `self.substs` is borrowed here
[00:08:52]     | |_________^ returns a value referencing data owned by the current function
[00:08:52] 
[00:08:52] 
[00:08:52] error[E0515]: cannot return value referencing local data `self.substs`
[00:08:52]    --> src/librustc/ty/sty.rs:411:9
[00:08:52]     |
[00:08:52] 411 | /         SplitGeneratorSubsts {
[00:08:52] 412 | |             yield_ty: self.substs.type_at(parent_len),
[00:08:52] 413 | |             return_ty: self.substs.type_at(parent_len + 1),
[00:08:52] 414 | |             witness: self.substs.type_at(parent_len + 2),
[00:08:52] 415 | |             upvar_kinds: &self.substs[parent_len + 3..],
[00:08:52]     | |                           ----------- `self.substs` is borrowed here
[00:08:52]     | |_________^ returns a value referencing data owned by the current function
[00:08:52] 
[00:08:53] error: aborting due to 25 previous errors
[00:08:53] 
---
travis_time:end:19902e7e:start=1556385519928735097,finish=1556385519935715687,duration=6980590
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0510ea40
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3402cbd5
travis_time:start:3402cbd5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:052fb705
$ dmesg | grep -i kill
