plain
    |
132 |     #[default_method_body_is_const]
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: internal compiler error: compiler/rustc_middle/src/hir/map/mod.rs:474:19: DefId(0:2108 ~ core[1bb8]::mem::Discriminant::0) is not a body node: Field
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1335:9
stack backtrace:
   0:     0x7f57fae90d02 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha288dc719902e24d
   0:     0x7f57fae90d02 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha288dc719902e24d
   1:     0x7f57faef8648 - core::fmt::write::h42234c3e51154f4c
   2:     0x7f57fae81051 - std::io::Write::write_fmt::h74fbb9643da2d185
   3:     0x7f57fae94046 - std::panicking::default_hook::{{closure}}::h3bfe018301273550
   4:     0x7f57fae93c3d - std::panicking::default_hook::h4173afa32faa81d9
   5:     0x7f57fb9e3a71 - rustc_driver[87b42345fa270eee]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f57fae949e0 - std::panicking::rust_panic_with_hook::h59cc3082e9104592
   7:     0x7f57fe330bb3 - std[eba90a372f7a1edd]::panicking::begin_panic::<rustc_errors[984494b0cf0e650]::ExplicitBug>::{closure#0}
   8:     0x7f57fe32e746 - std[eba90a372f7a1edd]::sys_common::backtrace::__rust_end_short_backtrace::<std[eba90a372f7a1edd]::panicking::begin_panic<rustc_errors[984494b0cf0e650]::ExplicitBug>::{closure#0}, !>
   9:     0x7f57fb90d0c6 - std[eba90a372f7a1edd]::panicking::begin_panic::<rustc_errors[984494b0cf0e650]::ExplicitBug>
  10:     0x7f57fe25dc76 - std[eba90a372f7a1edd]::panic::panic_any::<rustc_errors[984494b0cf0e650]::ExplicitBug>
  11:     0x7f57fe253146 - <rustc_errors[984494b0cf0e650]::HandlerInner>::bug::<&alloc[24f448636cd10183]::string::String>
  12:     0x7f57fe252e10 - <rustc_errors[984494b0cf0e650]::Handler>::bug::<&alloc[24f448636cd10183]::string::String>
  13:     0x7f57fe347dce - rustc_middle[8d4dc3708b593ac1]::ty::context::tls::with_opt::<rustc_middle[8d4dc3708b593ac1]::util::bug::opt_span_bug_fmt<rustc_span[5c736203a6ab5594]::span_encoding::Span>::{closure#0}, ()>
  14:     0x7f57fe3493f9 - rustc_middle[8d4dc3708b593ac1]::util::bug::opt_span_bug_fmt::<rustc_span[5c736203a6ab5594]::span_encoding::Span>
  15:     0x7f57fb913665 - rustc_middle[8d4dc3708b593ac1]::util::bug::bug_fmt
  16:     0x7f57fe2cc1d7 - <rustc_middle[8d4dc3708b593ac1]::hir::map::Map>::body_owner_kind
  17:     0x7f57fe2cc1f9 - <rustc_middle[8d4dc3708b593ac1]::hir::map::Map>::body_const_context
  18:     0x7f57fc485a42 - <<dyn rustc_typeck[759fce67295582a0]::astconv::AstConv>::create_substs_for_ast_path::SubstsForAstPathCtxt as rustc_typeck[759fce67295582a0]::astconv::CreateSubstsForGenericArgsCtxt>::inferred_kind
  19:     0x7f57fc482aca - <dyn rustc_typeck[759fce67295582a0]::astconv::AstConv>::create_substs_for_ast_path
  20:     0x7f57fc4614a4 - <dyn rustc_typeck[759fce67295582a0]::astconv::AstConv>::ast_path_to_mono_trait_ref
  21:     0x7f57fc47105b - <dyn rustc_typeck[759fce67295582a0]::astconv::AstConv>::res_to_ty
  22:     0x7f57fc4884fb - <dyn rustc_typeck[759fce67295582a0]::astconv::AstConv>::ast_ty_to_ty_inner
  23:     0x7f57fc52a1a8 - rustc_typeck[759fce67295582a0]::collect::type_of::type_of
  24:     0x7f57fd0d21dd - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::DefId, rustc_middle[8d4dc3708b593ac1]::ty::Ty>>
  25:     0x7f57fd1c15c1 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::type_of, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  26:     0x7f57fd50dce9 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::type_of
  27:     0x7f57fc304b14 - rustc_typeck[759fce67295582a0]::outlives::implicit_infer::infer_predicates
  28:     0x7f57fc552d7c - rustc_typeck[759fce67295582a0]::outlives::inferred_outlives_crate
  29:     0x7f57fd0a21c8 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::ArenaCache<(), rustc_middle[8d4dc3708b593ac1]::ty::CratePredicatesMap>>
  30:     0x7f57fd1a34c5 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::inferred_outlives_crate, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  31:     0x7f57fd51fcbe - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::inferred_outlives_crate
  32:     0x7f57fc5526e5 - rustc_typeck[759fce67295582a0]::outlives::inferred_outlives_of
  33:     0x7f57fd0e2ff1 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::DefId, &[(rustc_middle[8d4dc3708b593ac1]::ty::Predicate, rustc_span[5c736203a6ab5594]::span_encoding::Span)]>>
  34:     0x7f57fd18f92d - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::inferred_outlives_of, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  35:     0x7f57fd51a539 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::inferred_outlives_of
  36:     0x7f57fc382a02 - rustc_typeck[759fce67295582a0]::collect::predicates_defined_on
  37:     0x7f57fd0d767a - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::DefId, rustc_middle[8d4dc3708b593ac1]::ty::generics::GenericPredicates>>
  38:     0x7f57fd19345f - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::predicates_defined_on, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  39:     0x7f57fd5194bb - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::predicates_defined_on
  40:     0x7f57fc383445 - rustc_typeck[759fce67295582a0]::collect::predicates_of
  41:     0x7f57fd0d767a - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::DefId, rustc_middle[8d4dc3708b593ac1]::ty::generics::GenericPredicates>>
  42:     0x7f57fd178ef5 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::predicates_of, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  43:     0x7f57fd50f28b - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::predicates_of
  44:     0x7f57fc58d153 - rustc_ty_utils[d8b30179abf20fd3]::ty::param_env
  45:     0x7f57fd0d2fbd - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::DefId, rustc_middle[8d4dc3708b593ac1]::ty::ParamEnv>>
  46:     0x7f57fd1c4bbf - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::param_env, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  47:     0x7f57fd53d0b9 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::param_env
  48:     0x7f57fc830294 - <rustc_passes[d57e1aa5a6818bed]::stability::Checker as rustc_hir[78a3789577f2fa25]::intravisit::Visitor>::visit_item
  49:     0x7f57fc8354b6 - <rustc_middle[8d4dc3708b593ac1]::hir::map::Map>::visit_item_likes_in_module::<rustc_hir[78a3789577f2fa25]::intravisit::DeepVisitor<rustc_passes[d57e1aa5a6818bed]::stability::Checker>>
  50:     0x7f57fc82f6ed - rustc_passes[d57e1aa5a6818bed]::stability::check_mod_unstable_api_usage
  51:     0x7f57fd0bbcda - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::LocalDefId, ()>>
  52:     0x7f57fd1b3074 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::check_mod_unstable_api_usage, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  53:     0x7f57fd525e34 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::check_mod_unstable_api_usage
  54:     0x7f57fbabd486 - <core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[fc3bf7b819dbb0d8]::passes::analysis::{closure#0}::{closure#1}> as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once
  55:     0x7f57fbb023a4 - <rustc_session[2a929b385c5bc398]::session::Session>::time::<(), rustc_interface[fc3bf7b819dbb0d8]::passes::analysis::{closure#0}>
  56:     0x7f57fbaf4476 - rustc_interface[fc3bf7b819dbb0d8]::passes::analysis
  57:     0x7f57fd0f312c - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<(), core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>>
  58:     0x7f57fd1c16e2 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::analysis, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  59:     0x7f57fd50e24e - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::analysis
  60:     0x7f57fb9d395a - <rustc_interface[fc3bf7b819dbb0d8]::passes::QueryContext>::enter::<rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  61:     0x7f57fb9794b6 - <rustc_interface[fc3bf7b819dbb0d8]::interface::Compiler>::enter::<rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[fc3bf7b819dbb0d8]::queries::Linker>, rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  62:     0x7f57fb95b5b6 - rustc_span[5c736203a6ab5594]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_interface[fc3bf7b819dbb0d8]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#1}>
  63:     0x7f57fb97a7af - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[5c736203a6ab5594]::SessionGlobals>>::set::<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  64:     0x7f57fb9cf449 - std[eba90a372f7a1edd]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  65:     0x7f57fb98de01 - std[eba90a372f7a1edd]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  66:     0x7f57fb9d16c2 - <<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  67:     0x7f57faea13e3 - std::sys::unix::thread::Thread::new::thread_start::h6bc2e8f9e4e3d29f
  68:     0x7f57f53f1609 - start_thread
  69:     0x7f57fad04163 - clone
  70:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.62.0-nightly (c3818fbf8 2022-05-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [type_of] computing type of `mem::Discriminant::0`
#1 [inferred_outlives_crate] computing the inferred outlives predicates for items in this crate
#2 [inferred_outlives_of] computing inferred outlives predicates of `mem::maybe_uninit::MaybeUninit`
#3 [predicates_defined_on] computing predicates of `mem::maybe_uninit::MaybeUninit`
#4 [predicates_of] computing predicates of `mem::maybe_uninit::MaybeUninit`
#5 [param_env] computing normalized predicates of `mem::maybe_uninit::MaybeUninit`
#6 [check_mod_unstable_api_usage] checking for unstable API usage in module `mem::maybe_uninit`
#7 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: compiler/rustc_middle/src/hir/map/mod.rs:474:19: DefId(0:319 ~ core[1bb8]::num::bignum::FullOps) is not a body node: Trait
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1335:9
stack backtrace:
   0:     0x7f57fae90d02 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha288dc719902e24d
   0:     0x7f57fae90d02 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha288dc719902e24d
   1:     0x7f57faef8648 - core::fmt::write::h42234c3e51154f4c
   2:     0x7f57fae81051 - std::io::Write::write_fmt::h74fbb9643da2d185
   3:     0x7f57fae94046 - std::panicking::default_hook::{{closure}}::h3bfe018301273550
   4:     0x7f57fae93c3d - std::panicking::default_hook::h4173afa32faa81d9
   5:     0x7f57fb9e3a71 - rustc_driver[87b42345fa270eee]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f57fae949e0 - std::panicking::rust_panic_with_hook::h59cc3082e9104592
   7:     0x7f57fe330bb3 - std[eba90a372f7a1edd]::panicking::begin_panic::<rustc_errors[984494b0cf0e650]::ExplicitBug>::{closure#0}
   8:     0x7f57fe32e746 - std[eba90a372f7a1edd]::sys_common::backtrace::__rust_end_short_backtrace::<std[eba90a372f7a1edd]::panicking::begin_panic<rustc_errors[984494b0cf0e650]::ExplicitBug>::{closure#0}, !>
   9:     0x7f57fb90d0c6 - std[eba90a372f7a1edd]::panicking::begin_panic::<rustc_errors[984494b0cf0e650]::ExplicitBug>
  10:     0x7f57fe25dc76 - std[eba90a372f7a1edd]::panic::panic_any::<rustc_errors[984494b0cf0e650]::ExplicitBug>
  11:     0x7f57fe253146 - <rustc_errors[984494b0cf0e650]::HandlerInner>::bug::<&alloc[24f448636cd10183]::string::String>
  12:     0x7f57fe252e10 - <rustc_errors[984494b0cf0e650]::Handler>::bug::<&alloc[24f448636cd10183]::string::String>
  13:     0x7f57fe347dce - rustc_middle[8d4dc3708b593ac1]::ty::context::tls::with_opt::<rustc_middle[8d4dc3708b593ac1]::util::bug::opt_span_bug_fmt<rustc_span[5c736203a6ab5594]::span_encoding::Span>::{closure#0}, ()>
  14:     0x7f57fe3493f9 - rustc_middle[8d4dc3708b593ac1]::util::bug::opt_span_bug_fmt::<rustc_span[5c736203a6ab5594]::span_encoding::Span>
  15:     0x7f57fb913665 - rustc_middle[8d4dc3708b593ac1]::util::bug::bug_fmt
  16:     0x7f57fe2cc1d7 - <rustc_middle[8d4dc3708b593ac1]::hir::map::Map>::body_owner_kind
  17:     0x7f57fe2cc1f9 - <rustc_middle[8d4dc3708b593ac1]::hir::map::Map>::body_const_context
  18:     0x7f57fc485a42 - <<dyn rustc_typeck[759fce67295582a0]::astconv::AstConv>::create_substs_for_ast_path::SubstsForAstPathCtxt as rustc_typeck[759fce67295582a0]::astconv::CreateSubstsForGenericArgsCtxt>::inferred_kind
  19:     0x7f57fc482aca - <dyn rustc_typeck[759fce67295582a0]::astconv::AstConv>::create_substs_for_ast_path
  20:     0x7f57fc45c17f - <dyn rustc_typeck[759fce67295582a0]::astconv::AstConv>::instantiate_poly_trait_ref_inner
  21:     0x7f57fc4863b8 - <dyn rustc_typeck[759fce67295582a0]::astconv::AstConv>::instantiate_poly_trait_ref
  22:     0x7f57fc4872cc - <dyn rustc_typeck[759fce67295582a0]::astconv::AstConv>::add_bounds::<core[10878fb91fc84a80]::slice::iter::Iter<rustc_hir[78a3789577f2fa25]::hir::GenericBound>>
  23:     0x7f57fc4626c1 - <dyn rustc_typeck[759fce67295582a0]::astconv::AstConv>::compute_bounds_inner
  24:     0x7f57fc37d40b - rustc_typeck[759fce67295582a0]::collect::super_predicates_that_define_assoc_type
  25:     0x7f57fd1bcdc9 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::super_predicates_that_define_assoc_type, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  26:     0x7f57fd51b056 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::super_predicates_that_define_assoc_type
  27:     0x7f57fc37cf65 - rustc_typeck[759fce67295582a0]::collect::super_predicates_of
  28:     0x7f57fd0d767a - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::DefId, rustc_middle[8d4dc3708b593ac1]::ty::generics::GenericPredicates>>
  29:     0x7f57fd18e72f - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::super_predicates_of, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  30:     0x7f57fd51aabb - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::super_predicates_of
  31:     0x7f57fc383ded - rustc_typeck[759fce67295582a0]::collect::gather_explicit_predicates_of
  32:     0x7f57fc38618b - rustc_typeck[759fce67295582a0]::collect::trait_explicit_predicates_and_bounds
  33:     0x7f57fd0b5076 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::LocalDefId, rustc_middle[8d4dc3708b593ac1]::ty::generics::GenericPredicates>>
  34:     0x7f57fd1bb4ec - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::trait_explicit_predicates_and_bounds, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  35:     0x7f57fd519a3e - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::trait_explicit_predicates_and_bounds
  36:     0x7f57fc3864fb - rustc_typeck[759fce67295582a0]::collect::explicit_predicates_of
  37:     0x7f57fd0d767a - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::DefId, rustc_middle[8d4dc3708b593ac1]::ty::generics::GenericPredicates>>
  38:     0x7f57fd19a37f - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::explicit_predicates_of, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  39:     0x7f57fd519fbb - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::explicit_predicates_of
  40:     0x7f57fc382970 - rustc_typeck[759fce67295582a0]::collect::predicates_defined_on
  41:     0x7f57fd0d767a - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::DefId, rustc_middle[8d4dc3708b593ac1]::ty::generics::GenericPredicates>>
  42:     0x7f57fd19345f - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::predicates_defined_on, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  43:     0x7f57fd5194bb - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::predicates_defined_on
  44:     0x7f57fc383445 - rustc_typeck[759fce67295582a0]::collect::predicates_of
  45:     0x7f57fd0d767a - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::DefId, rustc_middle[8d4dc3708b593ac1]::ty::generics::GenericPredicates>>
  46:     0x7f57fd178ef5 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::predicates_of, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  47:     0x7f57fd50f28b - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::predicates_of
  48:     0x7f57fbe3f4c5 - <rustc_privacy[de044dad0112bda]::ReachEverythingInTheInterfaceVisitor>::predicates
  49:     0x7f57fbe3e38c - <rustc_privacy[de044dad0112bda]::EmbargoVisitor as rustc_hir[78a3789577f2fa25]::intravisit::Visitor>::visit_item
  50:     0x7f57fbe3471e - rustc_hir[78a3789577f2fa25]::intravisit::walk_item::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  51:     0x7f57fbe3e9fa - <rustc_privacy[de044dad0112bda]::EmbargoVisitor as rustc_hir[78a3789577f2fa25]::intravisit::Visitor>::visit_item
  52:     0x7f57fbe3471e - rustc_hir[78a3789577f2fa25]::intravisit::walk_item::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  53:     0x7f57fbe3e9fa - <rustc_privacy[de044dad0112bda]::EmbargoVisitor as rustc_hir[78a3789577f2fa25]::intravisit::Visitor>::visit_item
  54:     0x7f57fbe30cea - rustc_hir[78a3789577f2fa25]::intravisit::walk_mod::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  55:     0x7f57fbe4546b - rustc_privacy[de044dad0112bda]::privacy_access_levels
  56:     0x7f57fd0fa12c - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<(), &rustc_middle[8d4dc3708b593ac1]::middle::privacy::AccessLevels>>
  57:     0x7f57fd1935b5 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::privacy_access_levels, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  58:     0x7f57fd53226e - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::privacy_access_levels
  59:     0x7f57fc8312a7 - rustc_passes[d57e1aa5a6818bed]::stability::check_unused_or_stable_features
  60:     0x7f57fbb012ce - <rustc_session[2a929b385c5bc398]::session::Session>::time::<(), rustc_interface[fc3bf7b819dbb0d8]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
  61:     0x7f57fbb023be - <rustc_session[2a929b385c5bc398]::session::Session>::time::<(), rustc_interface[fc3bf7b819dbb0d8]::passes::analysis::{closure#0}>
  62:     0x7f57fbaf4476 - rustc_interface[fc3bf7b819dbb0d8]::passes::analysis
  63:     0x7f57fd0f312c - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<(), core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>>
  64:     0x7f57fd1c16e2 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::analysis, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  65:     0x7f57fd50e24e - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::analysis
  66:     0x7f57fb9d395a - <rustc_interface[fc3bf7b819dbb0d8]::passes::QueryContext>::enter::<rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  67:     0x7f57fb9794b6 - <rustc_interface[fc3bf7b819dbb0d8]::interface::Compiler>::enter::<rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[fc3bf7b819dbb0d8]::queries::Linker>, rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  68:     0x7f57fb95b5b6 - rustc_span[5c736203a6ab5594]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_interface[fc3bf7b819dbb0d8]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#1}>
  69:     0x7f57fb97a7af - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[5c736203a6ab5594]::SessionGlobals>>::set::<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  70:     0x7f57fb9cf449 - std[eba90a372f7a1edd]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  71:     0x7f57fb98de01 - std[eba90a372f7a1edd]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  72:     0x7f57fb9d16c2 - <<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  73:     0x7f57faea13e3 - std::sys::unix::thread::Thread::new::thread_start::h6bc2e8f9e4e3d29f
  74:     0x7f57f53f1609 - start_thread
  75:     0x7f57fad04163 - clone
  76:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.62.0-nightly (c3818fbf8 2022-05-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [super_predicates_that_define_assoc_type] computing the super traits of `num::bignum::FullOps`
#1 [super_predicates_of] computing the super predicates of `num::bignum::FullOps`
#2 [trait_explicit_predicates_and_bounds] computing explicit predicates of trait `num::bignum::FullOps`
#3 [explicit_predicates_of] computing explicit predicates of `num::bignum::FullOps`
#4 [predicates_defined_on] computing predicates of `num::bignum::FullOps`
#5 [predicates_of] computing predicates of `num::bignum::FullOps`
#6 [privacy_access_levels] privacy access levels
#7 [analysis] running analysis passes on this crate
error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:03:30
