plain
---- [ui] src/test/ui/impl-trait/in-trait/success.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/checkout/src/test/ui/impl-trait/in-trait/success.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/success" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/success/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', /cargo/registry/src/github.com-1ecc6299db9ec823/ena-0.14.0/src/snapshot_vec.rs:199:10
   0:     0x7f4038cd0acd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7aceaaec384abd8d
   1:     0x7f4038d36239 - core::fmt::write::h1935094ec1b611e9
   2:     0x7f4038cc1b11 - std::io::Write::write_fmt::h924a08855ab35277
   2:     0x7f4038cc1b11 - std::io::Write::write_fmt::h924a08855ab35277
   3:     0x7f4038cd3a7e - std::panicking::default_hook::{{closure}}::hdcba8bc567463877
   4:     0x7f4038cd3747 - std::panicking::default_hook::h5e023012d652ad1c
   5:     0x7f403968e2c4 - rustc_driver[2b12b44ab49049e7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f4038cd4221 - std::panicking::rust_panic_with_hook::hc50598e1e398727d
   7:     0x7f4038cd4047 - std::panicking::begin_panic_handler::{{closure}}::h2c1a8ae8110b48ef
   8:     0x7f4038cd1034 - std::sys_common::backtrace::__rust_end_short_backtrace::h4f0e14b26c3ea417
   9:     0x7f4038cd3d22 - rust_begin_unwind
  10:     0x7f4038c87cc3 - core::panicking::panic_fmt::h577a3b3df57afc41
  11:     0x7f4038c87c02 - core::panicking::panic_bounds_check::hea803f8b900e4221
  12:     0x7f403bfb80b0 - <ena[4ab445f0354216d2]::snapshot_vec::SnapshotVec<ena[4ab445f0354216d2]::unify::backing_vec::Delegate<rustc_middle[df447d51b984e2b2]::infer::unify_key::RegionVidKey>, &mut alloc[45623a189840f9f9]::vec::Vec<ena[4ab445f0354216d2]::unify::VarValue<rustc_middle[df447d51b984e2b2]::infer::unify_key::RegionVidKey>>, &mut rustc_infer[a1a39e651f51b0c1]::infer::undo_log::InferCtxtUndoLogs> as core[c1e30f1bd259d119]::ops::index::Index<usize>>::index
  13:     0x7f403bfd13d1 - <ena[4ab445f0354216d2]::unify::UnificationTable<ena[4ab445f0354216d2]::unify::backing_vec::InPlace<rustc_middle[df447d51b984e2b2]::infer::unify_key::RegionVidKey, &mut alloc[45623a189840f9f9]::vec::Vec<ena[4ab445f0354216d2]::unify::VarValue<rustc_middle[df447d51b984e2b2]::infer::unify_key::RegionVidKey>>, &mut rustc_infer[a1a39e651f51b0c1]::infer::undo_log::InferCtxtUndoLogs>>>::uninlined_get_root_key
  14:     0x7f403bf2a93e - <rustc_infer[a1a39e651f51b0c1]::infer::region_constraints::RegionConstraintCollector>::opportunistic_resolve_var
  15:     0x7f403bf07896 - <rustc_infer[a1a39e651f51b0c1]::infer::resolve::OpportunisticRegionResolver as rustc_middle[df447d51b984e2b2]::ty::fold::TypeFolder>::fold_region
  16:     0x7f403bf0fa1f - <rustc_middle[df447d51b984e2b2]::ty::Ty as rustc_middle[df447d51b984e2b2]::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_infer[a1a39e651f51b0c1]::infer::resolve::OpportunisticRegionResolver>
  17:     0x7f403bf07822 - <rustc_infer[a1a39e651f51b0c1]::infer::resolve::OpportunisticRegionResolver as rustc_middle[df447d51b984e2b2]::ty::fold::TypeFolder>::fold_ty
  18:     0x7f403bdcaba4 - <rustc_middle[df447d51b984e2b2]::ty::Term as rustc_middle[df447d51b984e2b2]::ty::fold::TypeFoldable>::fold_with::<rustc_infer[a1a39e651f51b0c1]::infer::resolve::OpportunisticRegionResolver>
  19:     0x7f403bcf7f0d - rustc_trait_selection[9ce78282165ecf4e]::traits::project::confirm_candidate
  20:     0x7f403bd048c3 - rustc_trait_selection[9ce78282165ecf4e]::traits::project::opt_normalize_projection_type
  21:     0x7f403bcf509a - rustc_trait_selection[9ce78282165ecf4e]::traits::project::normalize_projection_type
  22:     0x7f403ab01c7c - <rustc_infer[a1a39e651f51b0c1]::infer::InferCtxtBuilder as rustc_trait_selection[9ce78282165ecf4e]::infer::InferCtxtBuilderExt>::enter_canonical_trait_query::<rustc_middle[df447d51b984e2b2]::ty::ParamEnvAnd<rustc_middle[df447d51b984e2b2]::ty::sty::ProjectionTy>, rustc_middle[df447d51b984e2b2]::traits::query::NormalizationResult, rustc_traits[f93aaf4a965790c7]::normalize_projection_ty::normalize_projection_ty::{closure#0}>
  23:     0x7f403aae9b38 - rustc_traits[f93aaf4a965790c7]::normalize_projection_ty::normalize_projection_ty
  24:     0x7f403b3525a7 - rustc_query_system[75c107cad7cbc69e]::query::plumbing::get_query::<rustc_query_impl[42ef33d4955bd146]::queries::normalize_projection_ty, rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt>
  25:     0x7f403b1bbc0f - <rustc_query_impl[42ef33d4955bd146]::Queries as rustc_middle[df447d51b984e2b2]::ty::query::QueryEngine>::normalize_projection_ty
  26:     0x7f403bdf8a8d - <rustc_trait_selection[9ce78282165ecf4e]::traits::query::normalize::QueryNormalizer as rustc_middle[df447d51b984e2b2]::ty::fold::FallibleTypeFolder>::try_fold_ty
  27:     0x7f403aa01d68 - <&rustc_middle[df447d51b984e2b2]::ty::list::List<rustc_middle[df447d51b984e2b2]::ty::Ty> as rustc_middle[df447d51b984e2b2]::ty::fold::TypeFoldable>::try_fold_with::<rustc_trait_selection[9ce78282165ecf4e]::traits::query::normalize::QueryNormalizer>
  28:     0x7f403aa9cc46 - <rustc_middle[df447d51b984e2b2]::ty::sty::FnSig as rustc_middle[df447d51b984e2b2]::ty::fold::TypeFoldable>::try_fold_with::<rustc_trait_selection[9ce78282165ecf4e]::traits::query::normalize::QueryNormalizer>
  29:     0x7f403ab12645 - <rustc_infer[a1a39e651f51b0c1]::infer::at::At as rustc_trait_selection[9ce78282165ecf4e]::traits::query::normalize::AtExt>::normalize::<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>
  30:     0x7f403aa30327 - rustc_traits[f93aaf4a965790c7]::type_op::type_op_normalize::<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>
  31:     0x7f403aafcde3 - <rustc_infer[a1a39e651f51b0c1]::infer::InferCtxtBuilder as rustc_trait_selection[9ce78282165ecf4e]::infer::InferCtxtBuilderExt>::enter_canonical_trait_query::<rustc_middle[df447d51b984e2b2]::ty::ParamEnvAnd<rustc_middle[df447d51b984e2b2]::traits::query::type_op::Normalize<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>>, rustc_middle[df447d51b984e2b2]::ty::sty::FnSig, rustc_traits[f93aaf4a965790c7]::type_op::type_op_normalize<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>>
  32:     0x7f403aa30527 - rustc_traits[f93aaf4a965790c7]::type_op::type_op_normalize_fn_sig
  33:     0x7f403b3567f7 - rustc_query_system[75c107cad7cbc69e]::query::plumbing::get_query::<rustc_query_impl[42ef33d4955bd146]::queries::type_op_normalize_fn_sig, rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt>
  34:     0x7f403b1c263f - <rustc_query_impl[42ef33d4955bd146]::Queries as rustc_middle[df447d51b984e2b2]::ty::query::QueryEngine>::type_op_normalize_fn_sig
  35:     0x7f403be3ecd6 - <rustc_middle[df447d51b984e2b2]::ty::sty::FnSig as rustc_trait_selection[9ce78282165ecf4e]::traits::query::type_op::normalize::Normalizable>::type_op_method
  36:     0x7f403a731019 - <rustc_middle[df447d51b984e2b2]::traits::query::type_op::Normalize<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig> as rustc_trait_selection[9ce78282165ecf4e]::traits::query::type_op::QueryTypeOp>::fully_perform_into
  37:     0x7f403a873c5e - <rustc_middle[df447d51b984e2b2]::ty::ParamEnvAnd<rustc_middle[df447d51b984e2b2]::traits::query::type_op::Normalize<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>> as rustc_trait_selection[9ce78282165ecf4e]::traits::query::type_op::TypeOp>::fully_perform
  38:     0x7f403a8e0e4a - <rustc_borrowck[cd0647314e777f4b]::type_check::TypeChecker>::check_terminator
  39:     0x7f403a8ea768 - <rustc_borrowck[cd0647314e777f4b]::type_check::TypeChecker>::typeck_mir
  40:     0x7f403a8cda0f - rustc_borrowck[cd0647314e777f4b]::type_check::type_check
  41:     0x7f403a76f895 - rustc_borrowck[cd0647314e777f4b]::nll::compute_regions
  42:     0x7f403a9313af - rustc_borrowck[cd0647314e777f4b]::do_mir_borrowck
  43:     0x7f403a700b55 - <rustc_infer[a1a39e651f51b0c1]::infer::InferCtxtBuilder>::enter::<rustc_middle[df447d51b984e2b2]::mir::query::BorrowCheckResult, rustc_borrowck[cd0647314e777f4b]::mir_borrowck::{closure#0}>
  44:     0x7f403a9211d7 - rustc_borrowck[cd0647314e777f4b]::mir_borrowck
  45:     0x7f403a8f255e - <rustc_borrowck[cd0647314e777f4b]::provide::{closure#0} as core[c1e30f1bd259d119]::ops::function::FnOnce<(rustc_middle[df447d51b984e2b2]::ty::context::TyCtxt, rustc_span[8c7477ded0a91ee5]::def_id::LocalDefId)>>::call_once
  46:     0x7f403b24e8c5 - rustc_query_system[75c107cad7cbc69e]::query::plumbing::try_execute_query::<rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt, rustc_query_system[75c107cad7cbc69e]::query::caches::DefaultCache<rustc_span[8c7477ded0a91ee5]::def_id::LocalDefId, &rustc_middle[df447d51b984e2b2]::mir::query::BorrowCheckResult>>
  47:     0x7f403b31fcdc - rustc_query_system[75c107cad7cbc69e]::query::plumbing::get_query::<rustc_query_impl[42ef33d4955bd146]::queries::mir_borrowck, rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt>
  48:     0x7f403b1789d0 - <rustc_query_impl[42ef33d4955bd146]::Queries as rustc_middle[df447d51b984e2b2]::ty::query::QueryEngine>::mir_borrowck
  49:     0x7f4039832496 - <core[c1e30f1bd259d119]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b0c0ad648e9499f6]::sync::par_for_each_in<&[rustc_span[8c7477ded0a91ee5]::def_id::LocalDefId], <rustc_middle[df447d51b984e2b2]::hir::map::Map>::par_body_owners<rustc_interface[4989f6b1fe17a7df]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c1e30f1bd259d119]::ops::function::FnOnce<()>>::call_once
  50:     0x7f403979ae5b - rustc_data_structures[b0c0ad648e9499f6]::sync::par_for_each_in::<&[rustc_span[8c7477ded0a91ee5]::def_id::LocalDefId], <rustc_middle[df447d51b984e2b2]::hir::map::Map>::par_body_owners<rustc_interface[4989f6b1fe17a7df]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  51:     0x7f403981a625 - <rustc_session[5e51afea9c8b884e]::session::Session>::time::<(), rustc_interface[4989f6b1fe17a7df]::passes::analysis::{closure#2}>
  52:     0x7f40397d015b - rustc_interface[4989f6b1fe17a7df]::passes::analysis
  53:     0x7f403b290775 - rustc_query_system[75c107cad7cbc69e]::query::plumbing::try_execute_query::<rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt, rustc_query_system[75c107cad7cbc69e]::query::caches::DefaultCache<(), core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>>
  54:     0x7f403b371240 - rustc_query_system[75c107cad7cbc69e]::query::plumbing::get_query::<rustc_query_impl[42ef33d4955bd146]::queries::analysis, rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt>
  55:     0x7f403b14e3da - <rustc_query_impl[42ef33d4955bd146]::Queries as rustc_middle[df447d51b984e2b2]::ty::query::QueryEngine>::analysis
  56:     0x7f40396e3e8b - <rustc_interface[4989f6b1fe17a7df]::passes::QueryContext>::enter::<rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  57:     0x7f403968fcff - <rustc_interface[4989f6b1fe17a7df]::interface::Compiler>::enter::<rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}::{closure#2}, core[c1e30f1bd259d119]::result::Result<core[c1e30f1bd259d119]::option::Option<rustc_interface[4989f6b1fe17a7df]::queries::Linker>, rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  58:     0x7f403967cdc6 - rustc_span[8c7477ded0a91ee5]::with_source_map::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_interface[4989f6b1fe17a7df]::interface::create_compiler_and_run<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#1}>
  59:     0x7f403969112e - rustc_interface[4989f6b1fe17a7df]::interface::create_compiler_and_run::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>
  60:     0x7f4039673e92 - <scoped_tls[5efd78fa53ce51fd]::ScopedKey<rustc_span[8c7477ded0a91ee5]::SessionGlobals>>::set::<rustc_interface[4989f6b1fe17a7df]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  61:     0x7f40396ee46f - std[8a3c335779a4ef7b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4989f6b1fe17a7df]::util::run_in_thread_pool_with_globals<rustc_interface[4989f6b1fe17a7df]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  62:     0x7f403967e54e - std[8a3c335779a4ef7b]::panic::catch_unwind::<core[c1e30f1bd259d119]::panic::unwind_safe::AssertUnwindSafe<<std[8a3c335779a4ef7b]::thread::Builder>::spawn_unchecked_<rustc_interface[4989f6b1fe17a7df]::util::run_in_thread_pool_with_globals<rustc_interface[4989f6b1fe17a7df]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  63:     0x7f40396efc70 - <<std[8a3c335779a4ef7b]::thread::Builder>::spawn_unchecked_<rustc_interface[4989f6b1fe17a7df]::util::run_in_thread_pool_with_globals<rustc_interface[4989f6b1fe17a7df]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#1} as core[c1e30f1bd259d119]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  64:     0x7f4038ce0125 - std::sys::unix::thread::Thread::new::thread_start::h3a069647ae68a533
  65:     0x7f4038a81b43 - <unknown>
  66:     0x7f4038b13a00 - <unknown>
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  67:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (632a942a2 2022-09-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [normalize_projection_ty] normalizing `Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing, constness: NotConst }, value: ProjectionTy { substs: [&str], item_def_id: DefId(0:8 ~ success[5fe4]::Foo::bar::{opaque#0}) } } }`
#1 [type_op_normalize_fn_sig] normalizing `Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Region(U0) }], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing, constness: NotConst }, value: Normalize { value: ([&&str]; c_variadic: false)->impl for<'r> core::fmt::Display } } }`
#2 [mir_borrowck] borrow-checking `main`
#3 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/impl-trait/in-trait/nested-rpitit.rs stdout ----
---- [ui] src/test/ui/impl-trait/in-trait/nested-rpitit.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/checkout/src/test/ui/impl-trait/in-trait/nested-rpitit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/nested-rpitit" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/nested-rpitit/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', /cargo/registry/src/github.com-1ecc6299db9ec823/ena-0.14.0/src/snapshot_vec.rs:199:10
stack backtrace:
   0:     0x7f2c4e519acd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7aceaaec384abd8d
   1:     0x7f2c4e57f239 - core::fmt::write::h1935094ec1b611e9
   2:     0x7f2c4e50ab11 - std::io::Write::write_fmt::h924a08855ab35277
   3:     0x7f2c4e51ca7e - std::panicking::default_hook::{{closure}}::hdcba8bc567463877
   4:     0x7f2c4e51c747 - std::panicking::default_hook::h5e023012d652ad1c
   5:     0x7f2c4eed72c4 - rustc_driver[2b12b44ab49049e7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2c4e51d221 - std::panicking::rust_panic_with_hook::hc50598e1e398727d
   7:     0x7f2c4e51d047 - std::panicking::begin_panic_handler::{{closure}}::h2c1a8ae8110b48ef
   8:     0x7f2c4e51a034 - std::sys_common::backtrace::__rust_end_short_backtrace::h4f0e14b26c3ea417
  10:     0x7f2c4e4d0cc3 - core::panicking::panic_fmt::h577a3b3df57afc41
  10:     0x7f2c4e4d0cc3 - core::panicking::panic_fmt::h577a3b3df57afc41
  11:     0x7f2c4e4d0c02 - core::panicking::panic_bounds_check::hea803f8b900e4221
  12:     0x7f2c518010b0 - <ena[4ab445f0354216d2]::snapshot_vec::SnapshotVec<ena[4ab445f0354216d2]::unify::backing_vec::Delegate<rustc_middle[df447d51b984e2b2]::infer::unify_key::RegionVidKey>, &mut alloc[45623a189840f9f9]::vec::Vec<ena[4ab445f0354216d2]::unify::VarValue<rustc_middle[df447d51b984e2b2]::infer::unify_key::RegionVidKey>>, &mut rustc_infer[a1a39e651f51b0c1]::infer::undo_log::InferCtxtUndoLogs> as core[c1e30f1bd259d119]::ops::index::Index<usize>>::index
  13:     0x7f2c5181a3d1 - <ena[4ab445f0354216d2]::unify::UnificationTable<ena[4ab445f0354216d2]::unify::backing_vec::InPlace<rustc_middle[df447d51b984e2b2]::infer::unify_key::RegionVidKey, &mut alloc[45623a189840f9f9]::vec::Vec<ena[4ab445f0354216d2]::unify::VarValue<rustc_middle[df447d51b984e2b2]::infer::unify_key::RegionVidKey>>, &mut rustc_infer[a1a39e651f51b0c1]::infer::undo_log::InferCtxtUndoLogs>>>::uninlined_get_root_key
  14:     0x7f2c5177393e - <rustc_infer[a1a39e651f51b0c1]::infer::region_constraints::RegionConstraintCollector>::opportunistic_resolve_var
  15:     0x7f2c51750896 - <rustc_infer[a1a39e651f51b0c1]::infer::resolve::OpportunisticRegionResolver as rustc_middle[df447d51b984e2b2]::ty::fold::TypeFolder>::fold_region
  16:     0x7f2c51758a1f - <rustc_middle[df447d51b984e2b2]::ty::Ty as rustc_middle[df447d51b984e2b2]::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_infer[a1a39e651f51b0c1]::infer::resolve::OpportunisticRegionResolver>
  17:     0x7f2c51750822 - <rustc_infer[a1a39e651f51b0c1]::infer::resolve::OpportunisticRegionResolver as rustc_middle[df447d51b984e2b2]::ty::fold::TypeFolder>::fold_ty
  18:     0x7f2c51613ba4 - <rustc_middle[df447d51b984e2b2]::ty::Term as rustc_middle[df447d51b984e2b2]::ty::fold::TypeFoldable>::fold_with::<rustc_infer[a1a39e651f51b0c1]::infer::resolve::OpportunisticRegionResolver>
  19:     0x7f2c51540f0d - rustc_trait_selection[9ce78282165ecf4e]::traits::project::confirm_candidate
  20:     0x7f2c5154d8c3 - rustc_trait_selection[9ce78282165ecf4e]::traits::project::opt_normalize_projection_type
  21:     0x7f2c5153e09a - rustc_trait_selection[9ce78282165ecf4e]::traits::project::normalize_projection_type
  22:     0x7f2c5034ac7c - <rustc_infer[a1a39e651f51b0c1]::infer::InferCtxtBuilder as rustc_trait_selection[9ce78282165ecf4e]::infer::InferCtxtBuilderExt>::enter_canonical_trait_query::<rustc_middle[df447d51b984e2b2]::ty::ParamEnvAnd<rustc_middle[df447d51b984e2b2]::ty::sty::ProjectionTy>, rustc_middle[df447d51b984e2b2]::traits::query::NormalizationResult, rustc_traits[f93aaf4a965790c7]::normalize_projection_ty::normalize_projection_ty::{closure#0}>
  23:     0x7f2c50332b38 - rustc_traits[f93aaf4a965790c7]::normalize_projection_ty::normalize_projection_ty
  24:     0x7f2c50b9b5a7 - rustc_query_system[75c107cad7cbc69e]::query::plumbing::get_query::<rustc_query_impl[42ef33d4955bd146]::queries::normalize_projection_ty, rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt>
  25:     0x7f2c50a04c0f - <rustc_query_impl[42ef33d4955bd146]::Queries as rustc_middle[df447d51b984e2b2]::ty::query::QueryEngine>::normalize_projection_ty
  26:     0x7f2c51641a8d - <rustc_trait_selection[9ce78282165ecf4e]::traits::query::normalize::QueryNormalizer as rustc_middle[df447d51b984e2b2]::ty::fold::FallibleTypeFolder>::try_fold_ty
  27:     0x7f2c5024ad68 - <&rustc_middle[df447d51b984e2b2]::ty::list::List<rustc_middle[df447d51b984e2b2]::ty::Ty> as rustc_middle[df447d51b984e2b2]::ty::fold::TypeFoldable>::try_fold_with::<rustc_trait_selection[9ce78282165ecf4e]::traits::query::normalize::QueryNormalizer>
  28:     0x7f2c502e5c46 - <rustc_middle[df447d51b984e2b2]::ty::sty::FnSig as rustc_middle[df447d51b984e2b2]::ty::fold::TypeFoldable>::try_fold_with::<rustc_trait_selection[9ce78282165ecf4e]::traits::query::normalize::QueryNormalizer>
  29:     0x7f2c5035b645 - <rustc_infer[a1a39e651f51b0c1]::infer::at::At as rustc_trait_selection[9ce78282165ecf4e]::traits::query::normalize::AtExt>::normalize::<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>
  30:     0x7f2c50279327 - rustc_traits[f93aaf4a965790c7]::type_op::type_op_normalize::<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>
  31:     0x7f2c50345de3 - <rustc_infer[a1a39e651f51b0c1]::infer::InferCtxtBuilder as rustc_trait_selection[9ce78282165ecf4e]::infer::InferCtxtBuilderExt>::enter_canonical_trait_query::<rustc_middle[df447d51b984e2b2]::ty::ParamEnvAnd<rustc_middle[df447d51b984e2b2]::traits::query::type_op::Normalize<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>>, rustc_middle[df447d51b984e2b2]::ty::sty::FnSig, rustc_traits[f93aaf4a965790c7]::type_op::type_op_normalize<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>>
  32:     0x7f2c50279527 - rustc_traits[f93aaf4a965790c7]::type_op::type_op_normalize_fn_sig
  33:     0x7f2c50b9f7f7 - rustc_query_system[75c107cad7cbc69e]::query::plumbing::get_query::<rustc_query_impl[42ef33d4955bd146]::queries::type_op_normalize_fn_sig, rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt>
  34:     0x7f2c50a0b63f - <rustc_query_impl[42ef33d4955bd146]::Queries as rustc_middle[df447d51b984e2b2]::ty::query::QueryEngine>::type_op_normalize_fn_sig
  35:     0x7f2c51687cd6 - <rustc_middle[df447d51b984e2b2]::ty::sty::FnSig as rustc_trait_selection[9ce78282165ecf4e]::traits::query::type_op::normalize::Normalizable>::type_op_method
  36:     0x7f2c4ff7a019 - <rustc_middle[df447d51b984e2b2]::traits::query::type_op::Normalize<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig> as rustc_trait_selection[9ce78282165ecf4e]::traits::query::type_op::QueryTypeOp>::fully_perform_into
  37:     0x7f2c500bcc5e - <rustc_middle[df447d51b984e2b2]::ty::ParamEnvAnd<rustc_middle[df447d51b984e2b2]::traits::query::type_op::Normalize<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>> as rustc_trait_selection[9ce78282165ecf4e]::traits::query::type_op::TypeOp>::fully_perform
  38:     0x7f2c50129e4a - <rustc_borrowck[cd0647314e777f4b]::type_check::TypeChecker>::check_terminator
  39:     0x7f2c50133768 - <rustc_borrowck[cd0647314e777f4b]::type_check::TypeChecker>::typeck_mir
  40:     0x7f2c50116a0f - rustc_borrowck[cd0647314e777f4b]::type_check::type_check
  41:     0x7f2c4ffb8895 - rustc_borrowck[cd0647314e777f4b]::nll::compute_regions
  42:     0x7f2c5017a3af - rustc_borrowck[cd0647314e777f4b]::do_mir_borrowck
  43:     0x7f2c4ff49b55 - <rustc_infer[a1a39e651f51b0c1]::infer::InferCtxtBuilder>::enter::<rustc_middle[df447d51b984e2b2]::mir::query::BorrowCheckResult, rustc_borrowck[cd0647314e777f4b]::mir_borrowck::{closure#0}>
  44:     0x7f2c5016a1d7 - rustc_borrowck[cd0647314e777f4b]::mir_borrowck
  45:     0x7f2c5013b55e - <rustc_borrowck[cd0647314e777f4b]::provide::{closure#0} as core[c1e30f1bd259d119]::ops::function::FnOnce<(rustc_middle[df447d51b984e2b2]::ty::context::TyCtxt, rustc_span[8c7477ded0a91ee5]::def_id::LocalDefId)>>::call_once
  46:     0x7f2c50a978c5 - rustc_query_system[75c107cad7cbc69e]::query::plumbing::try_execute_query::<rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt, rustc_query_system[75c107cad7cbc69e]::query::caches::DefaultCache<rustc_span[8c7477ded0a91ee5]::def_id::LocalDefId, &rustc_middle[df447d51b984e2b2]::mir::query::BorrowCheckResult>>
  47:     0x7f2c50b68cdc - rustc_query_system[75c107cad7cbc69e]::query::plumbing::get_query::<rustc_query_impl[42ef33d4955bd146]::queries::mir_borrowck, rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt>
  48:     0x7f2c509c19d0 - <rustc_query_impl[42ef33d4955bd146]::Queries as rustc_middle[df447d51b984e2b2]::ty::query::QueryEngine>::mir_borrowck
  49:     0x7f2c4f07b496 - <core[c1e30f1bd259d119]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b0c0ad648e9499f6]::sync::par_for_each_in<&[rustc_span[8c7477ded0a91ee5]::def_id::LocalDefId], <rustc_middle[df447d51b984e2b2]::hir::map::Map>::par_body_owners<rustc_interface[4989f6b1fe17a7df]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c1e30f1bd259d119]::ops::function::FnOnce<()>>::call_once
  50:     0x7f2c4efe3e5b - rustc_data_structures[b0c0ad648e9499f6]::sync::par_for_each_in::<&[rustc_span[8c7477ded0a91ee5]::def_id::LocalDefId], <rustc_middle[df447d51b984e2b2]::hir::map::Map>::par_body_owners<rustc_interface[4989f6b1fe17a7df]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  51:     0x7f2c4f063625 - <rustc_session[5e51afea9c8b884e]::session::Session>::time::<(), rustc_interface[4989f6b1fe17a7df]::passes::analysis::{closure#2}>
  52:     0x7f2c4f01915b - rustc_interface[4989f6b1fe17a7df]::passes::analysis
  53:     0x7f2c50ad9775 - rustc_query_system[75c107cad7cbc69e]::query::plumbing::try_execute_query::<rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt, rustc_query_system[75c107cad7cbc69e]::query::caches::DefaultCache<(), core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>>
  54:     0x7f2c50bba240 - rustc_query_system[75c107cad7cbc69e]::query::plumbing::get_query::<rustc_query_impl[42ef33d4955bd146]::queries::analysis, rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt>
  55:     0x7f2c509973da - <rustc_query_impl[42ef33d4955bd146]::Queries as rustc_middle[df447d51b984e2b2]::ty::query::QueryEngine>::analysis
  56:     0x7f2c4ef2ce8b - <rustc_interface[4989f6b1fe17a7df]::passes::QueryContext>::enter::<rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  57:     0x7f2c4eed8cff - <rustc_interface[4989f6b1fe17a7df]::interface::Compiler>::enter::<rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}::{closure#2}, core[c1e30f1bd259d119]::result::Result<core[c1e30f1bd259d119]::option::Option<rustc_interface[4989f6b1fe17a7df]::queries::Linker>, rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  58:     0x7f2c4eec5dc6 - rustc_span[8c7477ded0a91ee5]::with_source_map::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_interface[4989f6b1fe17a7df]::interface::create_compiler_and_run<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#1}>
  59:     0x7f2c4eeda12e - rustc_interface[4989f6b1fe17a7df]::interface::create_compiler_and_run::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>
  60:     0x7f2c4eebce92 - <scoped_tls[5efd78fa53ce51fd]::ScopedKey<rustc_span[8c7477ded0a91ee5]::SessionGlobals>>::set::<rustc_interface[4989f6b1fe17a7df]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  61:     0x7f2c4ef3746f - std[8a3c335779a4ef7b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4989f6b1fe17a7df]::util::run_in_thread_pool_with_globals<rustc_interface[4989f6b1fe17a7df]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  62:     0x7f2c4eec754e - std[8a3c335779a4ef7b]::panic::catch_unwind::<core[c1e30f1bd259d119]::panic::unwind_safe::AssertUnwindSafe<<std[8a3c335779a4ef7b]::thread::Builder>::spawn_unchecked_<rustc_interface[4989f6b1fe17a7df]::util::run_in_thread_pool_with_globals<rustc_interface[4989f6b1fe17a7df]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  63:     0x7f2c4ef38c70 - <<std[8a3c335779a4ef7b]::thread::Builder>::spawn_unchecked_<rustc_interface[4989f6b1fe17a7df]::util::run_in_thread_pool_with_globals<rustc_interface[4989f6b1fe17a7df]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#1} as core[c1e30f1bd259d119]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  64:     0x7f2c4e529125 - std::sys::unix::thread::Thread::new::thread_start::h3a069647ae68a533
  65:     0x7f2c4e2cab43 - <unknown>
  66:     0x7f2c4e35ca00 - <unknown>
  67:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (632a942a2 2022-09-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
error: internal compiler error: compiler/rustc_ty_utils/src/assoc.rs:70:5: unexpected parent of trait or impl item or item not found: OpaqueTy(OpaqueTy { generics: Generics { params: [], predicates: [], has_where_clause_predicates: false, where_clause_span: /checkout/src/test/ui/impl-trait/in-trait/nested-rpitit.rs:10:21: 10:63 (#0), span: /checkout/src/test/ui/impl-trait/in-trait/nested-rpitit.rs:10:21: 10:63 (#0) }, bounds: [Trait(PolyTraitRef { bound_generic_params: [], trait_ref: TraitRef { path: Path { span: /checkout/src/test/ui/impl-trait/in-trait/nested-rpitit.rs:10:26: 10:63 (#0), res: Def(Trait, DefId(2:3589 ~ core[e3de]::ops::deref::Deref)), segments: [PathSegment { ident: Deref#0, hir_id: HirId { owner: DefId(0:11 ~ nested_rpitit[5876]::Foo::bar::{opaque#0}), local_id: 3 }, res: Def(Trait, DefId(2:3589 ~ core[e3de]::ops::deref::Deref)), args: Some(GenericArgs { args: [], bindings: [TypeBinding { hir_id: HirId { owner: DefId(0:11 ~ nested_rpitit[5876]::Foo::bar::{opaque#0}), local_id: 2 }, ident: Target#0, gen_args: GenericArgs { args: [], bindings: [], parenthesized: false, span_ext: no-location (#0) }, kind: Equality { term: Ty(Ty { hir_id: HirId { owner: DefId(0:11 ~ nested_rpitit[5876]::Foo::bar::{opaque#0}), local_id: 1 }, kind: OpaqueDef(ItemId { def_id: DefId(0:12 ~ nested_rpitit[5876]::Foo::bar::{opaque#0}::{opaque#0}) }, [], true), span: /checkout/src/test/ui/impl-trait/in-trait/nested-rpitit.rs:10:41: 10:62 (#0) }) }, span: /checkout/src/test/ui/impl-trait/in-trait/nested-rpitit.rs:10:32: 10:62 (#0) }], parenthesized: false, span_ext: /checkout/src/test/ui/impl-trait/in-trait/nested-rpitit.rs:10:31: 10:63 (#0) }), infer_args: false }] }, hir_ref_id: HirId { owner: DefId(0:11 ~ nested_rpitit[5876]::Foo::bar::{opaque#0}), local_id: 4 } }, span: /checkout/src/test/ui/impl-trait/in-trait/nested-rpitit.rs:10:26: 10:63 (#0) }, None)], origin: FnReturn(DefId(0:10 ~ nested_rpitit[5876]::Foo::bar)), in_trait: true })
   |
   |
LL |     fn bar(self) -> impl Deref<Target = impl Display + ?Sized>;

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1398:9
stack backtrace:
stack backtrace:
   0:     0x7f2c4e519acd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7aceaaec384abd8d
   1:     0x7f2c4e57f239 - core::fmt::write::h1935094ec1b611e9
   2:     0x7f2c4e50ab11 - std::io::Write::write_fmt::h924a08855ab35277
   3:     0x7f2c4e51ca7e - std::panicking::default_hook::{{closure}}::hdcba8bc567463877
   4:     0x7f2c4e51c747 - std::panicking::default_hook::h5e023012d652ad1c
   5:     0x7f2c4eed72c4 - rustc_driver[2b12b44ab49049e7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2c4e51d221 - std::panicking::rust_panic_with_hook::hc50598e1e398727d
   7:     0x7f2c4fb9f093 - std[8a3c335779a4ef7b]::panicking::begin_panic::<rustc_errors[a9e596aa33e6456]::ExplicitBug>::{closure#0}
   8:     0x7f2c4fb9d3a6 - std[8a3c335779a4ef7b]::sys_common::backtrace::__rust_end_short_backtrace::<std[8a3c335779a4ef7b]::panicking::begin_panic<rustc_errors[a9e596aa33e6456]::ExplicitBug>::{closure#0}, !>
   9:     0x7f2c4ed49d26 - std[8a3c335779a4ef7b]::panicking::begin_panic::<rustc_errors[a9e596aa33e6456]::ExplicitBug>
  10:     0x7f2c4fb9ca56 - std[8a3c335779a4ef7b]::panic::panic_any::<rustc_errors[a9e596aa33e6456]::ExplicitBug>
  11:     0x7f2c4fb9bb33 - <rustc_errors[a9e596aa33e6456]::HandlerInner>::span_bug::<rustc_span[8c7477ded0a91ee5]::span_encoding::Span, &alloc[45623a189840f9f9]::string::String>
  12:     0x7f2c4fb9b9e0 - <rustc_errors[a9e596aa33e6456]::Handler>::span_bug::<rustc_span[8c7477ded0a91ee5]::span_encoding::Span, &alloc[45623a189840f9f9]::string::String>
  13:     0x7f2c4fb8b52e - rustc_middle[df447d51b984e2b2]::ty::context::tls::with_context_opt::<rustc_middle[df447d51b984e2b2]::ty::context::tls::with_opt<rustc_middle[df447d51b984e2b2]::util::bug::opt_span_bug_fmt<rustc_span[8c7477ded0a91ee5]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f2c4fb8b359 - rustc_middle[df447d51b984e2b2]::util::bug::opt_span_bug_fmt::<rustc_span[8c7477ded0a91ee5]::span_encoding::Span>
  15:     0x7f2c4ed484f7 - rustc_middle[df447d51b984e2b2]::util::bug::span_bug_fmt::<rustc_span[8c7477ded0a91ee5]::span_encoding::Span>
  16:     0x7f2c4fbbf878 - rustc_ty_utils[5cc21446ab6f4c4d]::assoc::associated_item
  17:     0x7f2c50a61daf - rustc_query_system[75c107cad7cbc69e]::query::plumbing::try_execute_query::<rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt, rustc_query_system[75c107cad7cbc69e]::query::caches::ArenaCache<rustc_span[8c7477ded0a91ee5]::def_id::DefId, rustc_middle[df447d51b984e2b2]::ty::assoc::AssocItem>>
  18:     0x7f2c50b6e481 - rustc_query_system[75c107cad7cbc69e]::query::plumbing::get_query::<rustc_query_impl[42ef33d4955bd146]::queries::associated_item, rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt>
  19:     0x7f2c509b0795 - <rustc_query_impl[42ef33d4955bd146]::Queries as rustc_middle[df447d51b984e2b2]::ty::query::QueryEngine>::associated_item
  20:     0x7f2c519bfab8 - <rustc_middle[df447d51b984e2b2]::ty::print::pretty::FmtPrinter as rustc_middle[df447d51b984e2b2]::ty::print::pretty::PrettyPrinter>::pretty_print_opaque_impl_type
  21:     0x7f2c519c5f6d - <rustc_middle[df447d51b984e2b2]::ty::print::pretty::FmtPrinter as rustc_middle[df447d51b984e2b2]::ty::print::Printer>::print_type
  22:     0x7f2c51a752a8 - <rustc_middle[df447d51b984e2b2]::ty::Ty as core[c1e30f1bd259d119]::fmt::Display>::fmt
  23:     0x7f2c51a6df96 - <rustc_middle[df447d51b984e2b2]::ty::Ty as core[c1e30f1bd259d119]::fmt::Debug>::fmt
  24:     0x7f2c4e57f239 - core::fmt::write::h1935094ec1b611e9
  25:     0x7f2c4e5801c4 - <core::fmt::Formatter as core::fmt::Write>::write_fmt::h7e4ed53312471b7a
  26:     0x7f2c519398f5 - <rustc_middle[df447d51b984e2b2]::ty::sty::FnSig as core[c1e30f1bd259d119]::fmt::Debug>::fmt
  27:     0x7f2c4e57bf3d - core::fmt::builders::DebugStruct::field::h7642369eef3d4334
  28:     0x7f2c4e5802fb - core::fmt::Formatter::debug_struct_field1_finish::h5b94b5c02d851b22
  29:     0x7f2c5073bbc1 - <&rustc_middle[df447d51b984e2b2]::traits::query::type_op::Normalize<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig> as core[c1e30f1bd259d119]::fmt::Debug>::fmt
  30:     0x7f2c4e57bf3d - core::fmt::builders::DebugStruct::field::h7642369eef3d4334
  31:     0x7f2c4e5803ce - core::fmt::Formatter::debug_struct_field2_finish::he40bc74f473c9d3a
  32:     0x7f2c5095b009 - <&rustc_middle[df447d51b984e2b2]::ty::ParamEnvAnd<rustc_middle[df447d51b984e2b2]::traits::query::type_op::Normalize<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>> as core[c1e30f1bd259d119]::fmt::Debug>::fmt
  33:     0x7f2c4e57bf3d - core::fmt::builders::DebugStruct::field::h7642369eef3d4334
  34:     0x7f2c4e5804bd - core::fmt::Formatter::debug_struct_field3_finish::hc14dfdfdc85a6c54
  35:     0x7f2c50960950 - <rustc_middle[df447d51b984e2b2]::infer::canonical::Canonical<rustc_middle[df447d51b984e2b2]::ty::ParamEnvAnd<rustc_middle[df447d51b984e2b2]::traits::query::type_op::Normalize<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>>> as core[c1e30f1bd259d119]::fmt::Debug>::fmt
  36:     0x7f2c4e57f239 - core::fmt::write::h1935094ec1b611e9
  37:     0x7f2c4e56e7ff - alloc::fmt::format::format_inner::h8d3a65b1e7db262b
  38:     0x7f2c508e53c4 - <rustc_query_impl[42ef33d4955bd146]::queries::type_op_normalize_fn_sig as rustc_query_system[75c107cad7cbc69e]::query::config::QueryDescription<rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt>>::describe
  39:     0x7f2c5087fdc7 - rustc_query_impl[42ef33d4955bd146]::plumbing::create_query_frame::<rustc_middle[df447d51b984e2b2]::infer::canonical::Canonical<rustc_middle[df447d51b984e2b2]::ty::ParamEnvAnd<rustc_middle[df447d51b984e2b2]::traits::query::type_op::Normalize<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>>>>
  40:     0x7f2c50982b70 - <<rustc_query_impl[42ef33d4955bd146]::Queries>::try_collect_active_jobs::{closure#265} as core[c1e30f1bd259d119]::ops::function::FnOnce<(rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt, rustc_middle[df447d51b984e2b2]::infer::canonical::Canonical<rustc_middle[df447d51b984e2b2]::ty::ParamEnvAnd<rustc_middle[df447d51b984e2b2]::traits::query::type_op::Normalize<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>>>)>>::call_once
  41:     0x7f2c50a351fb - <rustc_query_system[75c107cad7cbc69e]::query::plumbing::QueryState<rustc_middle[df447d51b984e2b2]::infer::canonical::Canonical<rustc_middle[df447d51b984e2b2]::ty::ParamEnvAnd<rustc_middle[df447d51b984e2b2]::traits::query::type_op::Normalize<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>>>>>::try_collect_active_jobs::<rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt>
  42:     0x7f2c5098fa0e - <rustc_query_impl[42ef33d4955bd146]::Queries>::try_collect_active_jobs
  43:     0x7f2c509719c9 - rustc_query_system[75c107cad7cbc69e]::query::job::print_query_stack::<rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt>
  44:     0x7f2c4efe1484 - rustc_interface[4989f6b1fe17a7df]::interface::try_print_query_stack
  45:     0x7f2c4eed7e85 - rustc_driver[2b12b44ab49049e7]::report_ice
  46:     0x7f2c4e51d221 - std::panicking::rust_panic_with_hook::hc50598e1e398727d
  47:     0x7f2c4e51d047 - std::panicking::begin_panic_handler::{{closure}}::h2c1a8ae8110b48ef
  48:     0x7f2c4e51a034 - std::sys_common::backtrace::__rust_end_short_backtrace::h4f0e14b26c3ea417
  50:     0x7f2c4e4d0cc3 - core::panicking::panic_fmt::h577a3b3df57afc41
  50:     0x7f2c4e4d0cc3 - core::panicking::panic_fmt::h577a3b3df57afc41
  51:     0x7f2c4e4d0c02 - core::panicking::panic_bounds_check::hea803f8b900e4221
  52:     0x7f2c518010b0 - <ena[4ab445f0354216d2]::snapshot_vec::SnapshotVec<ena[4ab445f0354216d2]::unify::backing_vec::Delegate<rustc_middle[df447d51b984e2b2]::infer::unify_key::RegionVidKey>, &mut alloc[45623a189840f9f9]::vec::Vec<ena[4ab445f0354216d2]::unify::VarValue<rustc_middle[df447d51b984e2b2]::infer::unify_key::RegionVidKey>>, &mut rustc_infer[a1a39e651f51b0c1]::infer::undo_log::InferCtxtUndoLogs> as core[c1e30f1bd259d119]::ops::index::Index<usize>>::index
  53:     0x7f2c5181a3d1 - <ena[4ab445f0354216d2]::unify::UnificationTable<ena[4ab445f0354216d2]::unify::backing_vec::InPlace<rustc_middle[df447d51b984e2b2]::infer::unify_key::RegionVidKey, &mut alloc[45623a189840f9f9]::vec::Vec<ena[4ab445f0354216d2]::unify::VarValue<rustc_middle[df447d51b984e2b2]::infer::unify_key::RegionVidKey>>, &mut rustc_infer[a1a39e651f51b0c1]::infer::undo_log::InferCtxtUndoLogs>>>::uninlined_get_root_key
  54:     0x7f2c5177393e - <rustc_infer[a1a39e651f51b0c1]::infer::region_constraints::RegionConstraintCollector>::opportunistic_resolve_var
  55:     0x7f2c51750896 - <rustc_infer[a1a39e651f51b0c1]::infer::resolve::OpportunisticRegionResolver as rustc_middle[df447d51b984e2b2]::ty::fold::TypeFolder>::fold_region
  56:     0x7f2c51758a1f - <rustc_middle[df447d51b984e2b2]::ty::Ty as rustc_middle[df447d51b984e2b2]::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_infer[a1a39e651f51b0c1]::infer::resolve::OpportunisticRegionResolver>
  57:     0x7f2c51750822 - <rustc_infer[a1a39e651f51b0c1]::infer::resolve::OpportunisticRegionResolver as rustc_middle[df447d51b984e2b2]::ty::fold::TypeFolder>::fold_ty
  58:     0x7f2c51613ba4 - <rustc_middle[df447d51b984e2b2]::ty::Term as rustc_middle[df447d51b984e2b2]::ty::fold::TypeFoldable>::fold_with::<rustc_infer[a1a39e651f51b0c1]::infer::resolve::OpportunisticRegionResolver>
  59:     0x7f2c51540f0d - rustc_trait_selection[9ce78282165ecf4e]::traits::project::confirm_candidate
  60:     0x7f2c5154d8c3 - rustc_trait_selection[9ce78282165ecf4e]::traits::project::opt_normalize_projection_type
  61:     0x7f2c5153e09a - rustc_trait_selection[9ce78282165ecf4e]::traits::project::normalize_projection_type
  62:     0x7f2c5034ac7c - <rustc_infer[a1a39e651f51b0c1]::infer::InferCtxtBuilder as rustc_trait_selection[9ce78282165ecf4e]::infer::InferCtxtBuilderExt>::enter_canonical_trait_query::<rustc_middle[df447d51b984e2b2]::ty::ParamEnvAnd<rustc_middle[df447d51b984e2b2]::ty::sty::ProjectionTy>, rustc_middle[df447d51b984e2b2]::traits::query::NormalizationResult, rustc_traits[f93aaf4a965790c7]::normalize_projection_ty::normalize_projection_ty::{closure#0}>
  63:     0x7f2c50332b38 - rustc_traits[f93aaf4a965790c7]::normalize_projection_ty::normalize_projection_ty
  64:     0x7f2c50b9b5a7 - rustc_query_system[75c107cad7cbc69e]::query::plumbing::get_query::<rustc_query_impl[42ef33d4955bd146]::queries::normalize_projection_ty, rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt>
  65:     0x7f2c50a04c0f - <rustc_query_impl[42ef33d4955bd146]::Queries as rustc_middle[df447d51b984e2b2]::ty::query::QueryEngine>::normalize_projection_ty
  66:     0x7f2c51641a8d - <rustc_trait_selection[9ce78282165ecf4e]::traits::query::normalize::QueryNormalizer as rustc_middle[df447d51b984e2b2]::ty::fold::FallibleTypeFolder>::try_fold_ty
  67:     0x7f2c5024ad68 - <&rustc_middle[df447d51b984e2b2]::ty::list::List<rustc_middle[df447d51b984e2b2]::ty::Ty> as rustc_middle[df447d51b984e2b2]::ty::fold::TypeFoldable>::try_fold_with::<rustc_trait_selection[9ce78282165ecf4e]::traits::query::normalize::QueryNormalizer>
  68:     0x7f2c502e5c46 - <rustc_middle[df447d51b984e2b2]::ty::sty::FnSig as rustc_middle[df447d51b984e2b2]::ty::fold::TypeFoldable>::try_fold_with::<rustc_trait_selection[9ce78282165ecf4e]::traits::query::normalize::QueryNormalizer>
  69:     0x7f2c5035b645 - <rustc_infer[a1a39e651f51b0c1]::infer::at::At as rustc_trait_selection[9ce78282165ecf4e]::traits::query::normalize::AtExt>::normalize::<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>
  70:     0x7f2c50279327 - rustc_traits[f93aaf4a965790c7]::type_op::type_op_normalize::<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>
  71:     0x7f2c50345de3 - <rustc_infer[a1a39e651f51b0c1]::infer::InferCtxtBuilder as rustc_trait_selection[9ce78282165ecf4e]::infer::InferCtxtBuilderExt>::enter_canonical_trait_query::<rustc_middle[df447d51b984e2b2]::ty::ParamEnvAnd<rustc_middle[df447d51b984e2b2]::traits::query::type_op::Normalize<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>>, rustc_middle[df447d51b984e2b2]::ty::sty::FnSig, rustc_traits[f93aaf4a965790c7]::type_op::type_op_normalize<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>>
  72:     0x7f2c50279527 - rustc_traits[f93aaf4a965790c7]::type_op::type_op_normalize_fn_sig
  73:     0x7f2c50b9f7f7 - rustc_query_system[75c107cad7cbc69e]::query::plumbing::get_query::<rustc_query_impl[42ef33d4955bd146]::queries::type_op_normalize_fn_sig, rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt>
  74:     0x7f2c50a0b63f - <rustc_query_impl[42ef33d4955bd146]::Queries as rustc_middle[df447d51b984e2b2]::ty::query::QueryEngine>::type_op_normalize_fn_sig
  75:     0x7f2c51687cd6 - <rustc_middle[df447d51b984e2b2]::ty::sty::FnSig as rustc_trait_selection[9ce78282165ecf4e]::traits::query::type_op::normalize::Normalizable>::type_op_method
  76:     0x7f2c4ff7a019 - <rustc_middle[df447d51b984e2b2]::traits::query::type_op::Normalize<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig> as rustc_trait_selection[9ce78282165ecf4e]::traits::query::type_op::QueryTypeOp>::fully_perform_into
  77:     0x7f2c500bcc5e - <rustc_middle[df447d51b984e2b2]::ty::ParamEnvAnd<rustc_middle[df447d51b984e2b2]::traits::query::type_op::Normalize<rustc_middle[df447d51b984e2b2]::ty::sty::FnSig>> as rustc_trait_selection[9ce78282165ecf4e]::traits::query::type_op::TypeOp>::fully_perform
  78:     0x7f2c50129e4a - <rustc_borrowck[cd0647314e777f4b]::type_check::TypeChecker>::check_terminator
  79:     0x7f2c50133768 - <rustc_borrowck[cd0647314e777f4b]::type_check::TypeChecker>::typeck_mir
  80:     0x7f2c50116a0f - rustc_borrowck[cd0647314e777f4b]::type_check::type_check
  81:     0x7f2c4ffb8895 - rustc_borrowck[cd0647314e777f4b]::nll::compute_regions
  82:     0x7f2c5017a3af - rustc_borrowck[cd0647314e777f4b]::do_mir_borrowck
  83:     0x7f2c4ff49b55 - <rustc_infer[a1a39e651f51b0c1]::infer::InferCtxtBuilder>::enter::<rustc_middle[df447d51b984e2b2]::mir::query::BorrowCheckResult, rustc_borrowck[cd0647314e777f4b]::mir_borrowck::{closure#0}>
  84:     0x7f2c5016a1d7 - rustc_borrowck[cd0647314e777f4b]::mir_borrowck
  85:     0x7f2c5013b55e - <rustc_borrowck[cd0647314e777f4b]::provide::{closure#0} as core[c1e30f1bd259d119]::ops::function::FnOnce<(rustc_middle[df447d51b984e2b2]::ty::context::TyCtxt, rustc_span[8c7477ded0a91ee5]::def_id::LocalDefId)>>::call_once
  86:     0x7f2c50a978c5 - rustc_query_system[75c107cad7cbc69e]::query::plumbing::try_execute_query::<rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt, rustc_query_system[75c107cad7cbc69e]::query::caches::DefaultCache<rustc_span[8c7477ded0a91ee5]::def_id::LocalDefId, &rustc_middle[df447d51b984e2b2]::mir::query::BorrowCheckResult>>
  87:     0x7f2c50b68cdc - rustc_query_system[75c107cad7cbc69e]::query::plumbing::get_query::<rustc_query_impl[42ef33d4955bd146]::queries::mir_borrowck, rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt>
  88:     0x7f2c509c19d0 - <rustc_query_impl[42ef33d4955bd146]::Queries as rustc_middle[df447d51b984e2b2]::ty::query::QueryEngine>::mir_borrowck
  89:     0x7f2c4f07b496 - <core[c1e30f1bd259d119]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b0c0ad648e9499f6]::sync::par_for_each_in<&[rustc_span[8c7477ded0a91ee5]::def_id::LocalDefId], <rustc_middle[df447d51b984e2b2]::hir::map::Map>::par_body_owners<rustc_interface[4989f6b1fe17a7df]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c1e30f1bd259d119]::ops::function::FnOnce<()>>::call_once
  90:     0x7f2c4efe3e5b - rustc_data_structures[b0c0ad648e9499f6]::sync::par_for_each_in::<&[rustc_span[8c7477ded0a91ee5]::def_id::LocalDefId], <rustc_middle[df447d51b984e2b2]::hir::map::Map>::par_body_owners<rustc_interface[4989f6b1fe17a7df]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  91:     0x7f2c4f063625 - <rustc_session[5e51afea9c8b884e]::session::Session>::time::<(), rustc_interface[4989f6b1fe17a7df]::passes::analysis::{closure#2}>
  92:     0x7f2c4f01915b - rustc_interface[4989f6b1fe17a7df]::passes::analysis
  93:     0x7f2c50ad9775 - rustc_query_system[75c107cad7cbc69e]::query::plumbing::try_execute_query::<rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt, rustc_query_system[75c107cad7cbc69e]::query::caches::DefaultCache<(), core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>>
  94:     0x7f2c50bba240 - rustc_query_system[75c107cad7cbc69e]::query::plumbing::get_query::<rustc_query_impl[42ef33d4955bd146]::queries::analysis, rustc_query_impl[42ef33d4955bd146]::plumbing::QueryCtxt>
  95:     0x7f2c509973da - <rustc_query_impl[42ef33d4955bd146]::Queries as rustc_middle[df447d51b984e2b2]::ty::query::QueryEngine>::analysis
  96:     0x7f2c4ef2ce8b - <rustc_interface[4989f6b1fe17a7df]::passes::QueryContext>::enter::<rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  97:     0x7f2c4eed8cff - <rustc_interface[4989f6b1fe17a7df]::interface::Compiler>::enter::<rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}::{closure#2}, core[c1e30f1bd259d119]::result::Result<core[c1e30f1bd259d119]::option::Option<rustc_interface[4989f6b1fe17a7df]::queries::Linker>, rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  98:     0x7f2c4eec5dc6 - rustc_span[8c7477ded0a91ee5]::with_source_map::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_interface[4989f6b1fe17a7df]::interface::create_compiler_and_run<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#1}>
  99:     0x7f2c4eeda12e - rustc_interface[4989f6b1fe17a7df]::interface::create_compiler_and_run::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>
 100:     0x7f2c4eebce92 - <scoped_tls[5efd78fa53ce51fd]::ScopedKey<rustc_span[8c7477ded0a91ee5]::SessionGlobals>>::set::<rustc_interface[4989f6b1fe17a7df]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
 101:     0x7f2c4ef3746f - std[8a3c335779a4ef7b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4989f6b1fe17a7df]::util::run_in_thread_pool_with_globals<rustc_interface[4989f6b1fe17a7df]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
 102:     0x7f2c4eec754e - std[8a3c335779a4ef7b]::panic::catch_unwind::<core[c1e30f1bd259d119]::panic::unwind_safe::AssertUnwindSafe<<std[8a3c335779a4ef7b]::thread::Builder>::spawn_unchecked_<rustc_interface[4989f6b1fe17a7df]::util::run_in_thread_pool_with_globals<rustc_interface[4989f6b1fe17a7df]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
 103:     0x7f2c4ef38c70 - <<std[8a3c335779a4ef7b]::thread::Builder>::spawn_unchecked_<rustc_interface[4989f6b1fe17a7df]::util::run_in_thread_pool_with_globals<rustc_interface[4989f6b1fe17a7df]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#1} as core[c1e30f1bd259d119]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 104:     0x7f2c4e529125 - std::sys::unix::thread::Thread::new::thread_start::h3a069647ae68a533
 105:     0x7f2c4e2cab43 - <unknown>
 106:     0x7f2c4e35ca00 - <unknown>
 107:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (632a942a2 2022-09-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
