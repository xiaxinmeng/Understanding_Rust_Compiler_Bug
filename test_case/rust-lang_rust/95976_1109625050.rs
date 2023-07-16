plain
............................................i........................................... 1936/13013
........................................................................................ 2024/13013
........................................................................................ 2112/13013
........................................................................................ 2200/13013
....................................................................F................... 2288/13013
...............F........................................................................ 2376/13013
........................F............................................................... 2464/13013
.F.....F.................F..FF.....F.F.F......F......................................... 2552/13013
...............................................F...........................F............ 2640/13013
.........................................F..................F........................... 2728/13013
..........................................F...............F...F......................... 2816/13013
.......................F............................................i................... 2904/13013
........................................................................................ 3080/13013
.....................................................................................iii 3168/13013
ii...................................................................................... 3256/13013
........................................................................................ 3344/13013
---
........................................................................................ 10824/13013
........................................................................................ 10912/13013
........................................................................................ 11000/13013
........................................................................................ 11088/13013
........................F...............................F............................... 11176/13013
........................................................................................ 11352/13013
........................................................................................ 11440/13013
........................................................................................ 11528/13013
.....................i........i.....i............................i...................... 11616/13013
---
---- [ui] src/test/ui/coercion/issue-39823.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coercion/issue-39823.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/issue-39823/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/issue-39823/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_const_eval/src/const_eval/valtrees.rs:449:14: shouldn't have created a ValTree for fn(u32) -> LocalC {LocalC}
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1313:9
stack backtrace:
stack backtrace:
   0:     0x7f3eab28e9ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7a8d9e17c5c41d63
   1:     0x7f3eab2f4ff8 - core::fmt::write::he96e6c1da7790e01
   2:     0x7f3eab27e811 - std::io::Write::write_fmt::he213735f9fc023bf
   3:     0x7f3eab2919de - std::panicking::default_hook::{{closure}}::h1db1d872e7cc6dba
   4:     0x7f3eab29160c - std::panicking::default_hook::h117fb00cf01e902c
   5:     0x7f3eabdf2ba1 - rustc_driver[f41c55b70dce3ba2]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f3eab29223e - std::panicking::rust_panic_with_hook::hfb6238818b193c9a
   7:     0x7f3eae5c2503 - std[8b1df1c4c90d5389]::panicking::begin_panic::<rustc_errors[b2d4e9a5f061dc95]::ExplicitBug>::{closure#0}
   8:     0x7f3eae5c1366 - std[8b1df1c4c90d5389]::sys_common::backtrace::__rust_end_short_backtrace::<std[8b1df1c4c90d5389]::panicking::begin_panic<rustc_errors[b2d4e9a5f061dc95]::ExplicitBug>::{closure#0}, !>
   9:     0x7f3eabd11d1f - std[8b1df1c4c90d5389]::panicking::begin_panic::<rustc_errors[b2d4e9a5f061dc95]::ExplicitBug>
  10:     0x7f3eae6c16c6 - std[8b1df1c4c90d5389]::panic::panic_any::<rustc_errors[b2d4e9a5f061dc95]::ExplicitBug>
  11:     0x7f3eae6c0186 - <rustc_errors[b2d4e9a5f061dc95]::HandlerInner>::bug::<&alloc[57f2ea052be60ee9]::string::String>
  12:     0x7f3eae6bfe40 - <rustc_errors[b2d4e9a5f061dc95]::Handler>::bug::<&alloc[57f2ea052be60ee9]::string::String>
  13:     0x7f3eae79f3d5 - rustc_middle[95761bf3c41b10cf]::util::bug::opt_span_bug_fmt::<rustc_span[3e9cd59107943c18]::span_encoding::Span>::{closure#0}
  14:     0x7f3eae79f2db - rustc_middle[95761bf3c41b10cf]::ty::context::tls::with_opt::<rustc_middle[95761bf3c41b10cf]::util::bug::opt_span_bug_fmt<rustc_span[3e9cd59107943c18]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7f3eae79f28c - rustc_middle[95761bf3c41b10cf]::ty::context::tls::with_opt::<rustc_middle[95761bf3c41b10cf]::util::bug::opt_span_bug_fmt<rustc_span[3e9cd59107943c18]::span_encoding::Span>::{closure#0}, ()>
  16:     0x7f3eae79f319 - rustc_middle[95761bf3c41b10cf]::util::bug::opt_span_bug_fmt::<rustc_span[3e9cd59107943c18]::span_encoding::Span>
  17:     0x7f3eabd1876e - rustc_middle[95761bf3c41b10cf]::util::bug::bug_fmt
  18:     0x7f3ead37c291 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::fill_place_recursively
  19:     0x7f3ead37767d - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::valtree_to_const_value
  20:     0x7f3ead321e4b - <rustc_const_eval[3bdf8a288a416b24]::provide::{closure#2} as core[f370780f52494a52]::ops::function::FnOnce<(rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt, (rustc_middle[95761bf3c41b10cf]::ty::Ty, rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree))>>::call_once
  21:     0x7f3ead5e2203 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::valtree_to_const_val, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  22:     0x7f3ead96edeb - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::valtree_to_const_val
  23:     0x7f3ead36983d - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_const_value_raw_provider
  24:     0x7f3ead5f78e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_const_value_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  25:     0x7f3ead96ed45 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_const_value_raw
  26:     0x7f3ead368dc0 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_const_value_raw_provider
  27:     0x7f3ead5f78e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_const_value_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  28:     0x7f3ead96ed45 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_const_value_raw
  29:     0x7f3eae5fce2d - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::const_eval_global_id
  30:     0x7f3eae62498a - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::const_eval_resolve
  31:     0x7f3eae26a51e - <rustc_trait_selection[561e35c8266e314f]::traits::query::normalize::QueryNormalizer as rustc_middle[95761bf3c41b10cf]::ty::fold::FallibleTypeFolder>::try_fold_const
  32:     0x7f3eae26a856 - <rustc_trait_selection[561e35c8266e314f]::traits::query::normalize::QueryNormalizer as rustc_middle[95761bf3c41b10cf]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  33:     0x7f3ead0ffa5f - <rustc_infer[6814aa6b2eed40e]::infer::at::At as rustc_trait_selection[561e35c8266e314f]::traits::query::normalize::AtExt>::normalize::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  34:     0x7f3ead10da71 - <rustc_infer[6814aa6b2eed40e]::infer::InferCtxtBuilder>::enter::<core[f370780f52494a52]::result::Result<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind, rustc_middle[95761bf3c41b10cf]::traits::query::NoSolution>, rustc_traits[7167181ae6d12e6e]::normalize_erasing_regions::try_normalize_after_erasing_regions<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>::{closure#0}>
  35:     0x7f3ead20e8fe - <rustc_traits[7167181ae6d12e6e]::normalize_erasing_regions::provide::{closure#1} as core[f370780f52494a52]::ops::function::FnOnce<(rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt, rustc_middle[95761bf3c41b10cf]::ty::ParamEnvAnd<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>)>>::call_once
  36:     0x7f3ead621ec0 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::try_normalize_mir_const_after_erasing_regions, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  37:     0x7f3ead96fcee - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::try_normalize_mir_const_after_erasing_regions
  38:     0x7f3eae79d314 - <rustc_middle[95761bf3c41b10cf]::ty::normalize_erasing_regions::TryNormalizeAfterErasingRegionsFolder as rustc_middle[95761bf3c41b10cf]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  39:     0x7f3eac5051b3 - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::try_normalize_erasing_regions::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  40:     0x7f3eac506bc1 - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::try_subst_and_normalize_erasing_regions::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  41:     0x7f3eac2b6c03 - <rustc_const_eval[3bdf8a288a416b24]::interpret::eval_context::InterpCx<rustc_mir_transform[596f0f775876c0b2]::const_prop_lint::ConstPropMachine>>::subst_from_current_frame_and_normalize_erasing_regions::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  42:     0x7f3eac2feb32 - <rustc_const_eval[3bdf8a288a416b24]::interpret::eval_context::InterpCx<rustc_mir_transform[596f0f775876c0b2]::const_prop_lint::ConstPropMachine>>::eval_operand
  43:     0x7f3eac2d56b4 - <rustc_const_eval[3bdf8a288a416b24]::interpret::eval_context::InterpCx<rustc_mir_transform[596f0f775876c0b2]::const_prop_lint::ConstPropMachine>>::eval_rvalue_into_place
  44:     0x7f3eac43de63 - <rustc_mir_transform[596f0f775876c0b2]::const_prop_lint::ConstPropagator as rustc_middle[95761bf3c41b10cf]::mir::visit::Visitor>::visit_statement
  45:     0x7f3eac43d8ff - <rustc_mir_transform[596f0f775876c0b2]::const_prop_lint::ConstPropagator as rustc_middle[95761bf3c41b10cf]::mir::visit::Visitor>::visit_body
  46:     0x7f3eac439ebd - <rustc_mir_transform[596f0f775876c0b2]::const_prop_lint::ConstProp as rustc_mir_transform[596f0f775876c0b2]::pass_manager::MirLint>::run_lint
  47:     0x7f3eac38f9f2 - rustc_mir_transform[596f0f775876c0b2]::pass_manager::run_passes
  48:     0x7f3eac480e09 - rustc_mir_transform[596f0f775876c0b2]::run_post_borrowck_cleanup_passes
  49:     0x7f3eac4806d9 - rustc_mir_transform[596f0f775876c0b2]::mir_drops_elaborated_and_const_checked
  50:     0x7f3ead4dc878 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::try_execute_query::<rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt, rustc_query_system[839b2ca9f9a29e23]::query::caches::DefaultCache<rustc_middle[95761bf3c41b10cf]::ty::WithOptConstParam<rustc_span[3e9cd59107943c18]::def_id::LocalDefId>, &rustc_data_structures[3cf013d82b52e206]::steal::Steal<rustc_middle[95761bf3c41b10cf]::mir::Body>>>
  51:     0x7f3ead61f9ea - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  52:     0x7f3ead96e556 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  53:     0x7f3eac481398 - rustc_mir_transform[596f0f775876c0b2]::optimized_mir
  54:     0x7f3ead510cc8 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::try_execute_query::<rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt, rustc_query_system[839b2ca9f9a29e23]::query::caches::DefaultCache<rustc_span[3e9cd59107943c18]::def_id::DefId, &rustc_middle[95761bf3c41b10cf]::mir::Body>>
  55:     0x7f3ead5b8903 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::optimized_mir, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  56:     0x7f3eae622b4c - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::instance_mir
  57:     0x7f3eac270b87 - rustc_monomorphize[c0e6daa8b6483308]::collector::collect_neighbours
  58:     0x7f3eac269315 - rustc_monomorphize[c0e6daa8b6483308]::collector::collect_items_rec
  59:     0x7f3eac284d91 - <rustc_session[9ce44d69e2e9f546]::session::Session>::time::<(), rustc_monomorphize[c0e6daa8b6483308]::collector::collect_crate_mono_items::{closure#1}>
  60:     0x7f3eac265d7f - rustc_monomorphize[c0e6daa8b6483308]::collector::collect_crate_mono_items
  61:     0x7f3eac283b19 - rustc_monomorphize[c0e6daa8b6483308]::partitioning::collect_and_partition_mono_items
  62:     0x7f3ead53323a - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::try_execute_query::<rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt, rustc_query_system[839b2ca9f9a29e23]::query::caches::DefaultCache<(), (&std[8b1df1c4c90d5389]::collections::hash::set::HashSet<rustc_span[3e9cd59107943c18]::def_id::DefId, core[f370780f52494a52]::hash::BuildHasherDefault<rustc_hash[dc8b4bbe1f5b965a]::FxHasher>>, &[rustc_middle[95761bf3c41b10cf]::mir::mono::CodegenUnit])>>
  63:     0x7f3ead6192e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::collect_and_partition_mono_items, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  64:     0x7f3ead96fb22 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::collect_and_partition_mono_items
  65:     0x7f3eabf7d02a - rustc_codegen_ssa[ed52ad029bc3d7ac]::base::codegen_crate::<rustc_codegen_llvm[8f4a8c0c64236530]::LlvmCodegenBackend>
  66:     0x7f3eac07a83b - <rustc_codegen_llvm[8f4a8c0c64236530]::LlvmCodegenBackend as rustc_codegen_ssa[ed52ad029bc3d7ac]::traits::backend::CodegenBackend>::codegen_crate
  67:     0x7f3eabf04e91 - <rustc_session[9ce44d69e2e9f546]::session::Session>::time::<alloc[57f2ea052be60ee9]::boxed::Box<dyn core[f370780f52494a52]::any::Any>, rustc_interface[cf714bd2a8aae84d]::passes::start_codegen::{closure#0}>
  68:     0x7f3eabeecfe4 - <rustc_interface[cf714bd2a8aae84d]::passes::QueryContext>::enter::<<rustc_interface[cf714bd2a8aae84d]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[f370780f52494a52]::result::Result<alloc[57f2ea052be60ee9]::boxed::Box<dyn core[f370780f52494a52]::any::Any>, rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  69:     0x7f3eabed85ce - <rustc_interface[cf714bd2a8aae84d]::queries::Queries>::ongoing_codegen
  70:     0x7f3eabd830d0 - <rustc_interface[cf714bd2a8aae84d]::interface::Compiler>::enter::<rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}::{closure#2}, core[f370780f52494a52]::result::Result<core[f370780f52494a52]::option::Option<rustc_interface[cf714bd2a8aae84d]::queries::Linker>, rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  71:     0x7f3eabd65b9b - rustc_span[3e9cd59107943c18]::with_source_map::<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_interface[cf714bd2a8aae84d]::interface::create_compiler_and_run<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#1}>
  72:     0x7f3eabd84209 - <scoped_tls[763d6a33bb6869db]::ScopedKey<rustc_span[3e9cd59107943c18]::SessionGlobals>>::set::<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  73:     0x7f3eabdddec9 - std[8b1df1c4c90d5389]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf714bd2a8aae84d]::util::run_in_thread_pool_with_globals<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  74:     0x7f3eabd855c1 - std[8b1df1c4c90d5389]::panicking::try::<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, core[f370780f52494a52]::panic::unwind_safe::AssertUnwindSafe<<std[8b1df1c4c90d5389]::thread::Builder>::spawn_unchecked_<rustc_interface[cf714bd2a8aae84d]::util::run_in_thread_pool_with_globals<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  75:     0x7f3eabdd8e02 - <<std[8b1df1c4c90d5389]::thread::Builder>::spawn_unchecked_<rustc_interface[cf714bd2a8aae84d]::util::run_in_thread_pool_with_globals<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#1} as core[f370780f52494a52]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  76:     0x7f3eab29d593 - std::sys::unix::thread::Thread::new::thread_start::h38917ce570c7f83a
  77:     0x7f3ea57f0609 - start_thread
  78:     0x7f3eab103163 - clone
  79:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.62.0-nightly (4d1d3cedd 2022-04-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [valtree_to_const_val] convert type-level constant value to mir constant value
#1 [eval_to_const_value_raw] simplifying constant for the type system `main::promoted[7]`
#2 [eval_to_const_value_raw] simplifying constant for the type system `main::promoted[7]`
#3 [try_normalize_mir_const_after_erasing_regions] normalizing `main::promoted[7]`
#4 [mir_drops_elaborated_and_const_checked] elaborating drops for `main`
#5 [optimized_mir] optimizing MIR for `main`
#6 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/min_const_generics/invalid-patterns.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/invalid-patterns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/invalid-patterns/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:29:21
   |
   |
LL |   get_flag::<false, 0xFF>();
   |                     ^^^^ expected `char`, found `u8`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:31:14
   |
   |
LL |   get_flag::<7, 'c'>();
   |              ^ expected `bool`, found integer
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:33:14
   |
   |
LL |   get_flag::<42, 0x5ad>();
   |              ^^ expected `bool`, found integer
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:33:18
   |
   |
LL |   get_flag::<42, 0x5ad>();
   |                  ^^^^^ expected `char`, found `u8`

thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: InterpErrorInfo(InterpErrorInfoInner { kind: using uninitialized data, but this operation requires initialized memory, backtrace: None })', compiler/rustc_const_eval/src/const_eval/valtrees.rs:79:39
stack backtrace:
   0:     0x7f33c07959ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7a8d9e17c5c41d63
   1:     0x7f33c07fbff8 - core::fmt::write::he96e6c1da7790e01
   2:     0x7f33c0785811 - std::io::Write::write_fmt::he213735f9fc023bf
   3:     0x7f33c07989de - std::panicking::default_hook::{{closure}}::h1db1d872e7cc6dba
   4:     0x7f33c079860c - std::panicking::default_hook::h117fb00cf01e902c
   5:     0x7f33c12f9ba1 - rustc_driver[f41c55b70dce3ba2]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f33c079923e - std::panicking::rust_panic_with_hook::hfb6238818b193c9a
   7:     0x7f33c0799037 - std::panicking::begin_panic_handler::{{closure}}::h1c134276a38e45ce
   8:     0x7f33c0795ec4 - std::sys_common::backtrace::__rust_end_short_backtrace::h3511975eefed2e40
  10:     0x7f33c074d073 - core::panicking::panic_fmt::hb74e78879777d5d2
  11:     0x7f33c074d273 - core::result::unwrap_failed::h943c676139e14f19
  11:     0x7f33c074d273 - core::result::unwrap_failed::h943c676139e14f19
  12:     0x7f33c287b19d - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::const_to_valtree_inner
  13:     0x7f33c27fdd58 - <core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::try_fold::<(), <core[f370780f52494a52]::iter::adapters::GenericShunt<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::try_fold<(), core[f370780f52494a52]::iter::traits::iterator::Iterator::try_for_each::call<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>::Break>::{closure#0}, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>::{closure#0}, core[f370780f52494a52]::ops::control_flow::ControlFlow<core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>>
  14:     0x7f33c286602b - <core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::try_fold::<(), <core[f370780f52494a52]::iter::adapters::GenericShunt<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::try_fold<(), core[f370780f52494a52]::iter::traits::iterator::Iterator::try_for_each::call<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>::Break>::{closure#0}, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>::{closure#0}, core[f370780f52494a52]::ops::control_flow::ControlFlow<core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>>
  15:     0x7f33c280245b - <core[f370780f52494a52]::iter::adapters::GenericShunt<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::next
  16:     0x7f33c28e94ae - <alloc[57f2ea052be60ee9]::vec::Vec<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree> as alloc[57f2ea052be60ee9]::vec::spec_from_iter::SpecFromIter<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree, core[f370780f52494a52]::iter::adapters::GenericShunt<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>>>>::from_iter
  17:     0x7f33c27fab49 - core[f370780f52494a52]::iter::adapters::try_process::<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>, <core[f370780f52494a52]::option::Option<alloc[57f2ea052be60ee9]::vec::Vec<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>> as core[f370780f52494a52]::iter::traits::collect::FromIterator<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>>::from_iter<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>>::{closure#0}, alloc[57f2ea052be60ee9]::vec::Vec<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>
  18:     0x7f33c2879964 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches
  19:     0x7f33c287a9a2 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::const_to_valtree_inner
  20:     0x7f33c2879269 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::const_to_valtree
  21:     0x7f33c2870630 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_const_value_raw_provider
  22:     0x7f33c2afe8e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_const_value_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  23:     0x7f33c2e75d45 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_const_value_raw
  24:     0x7f33c286fdc0 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_const_value_raw_provider
  25:     0x7f33c2afe8e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_const_value_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  26:     0x7f33c2e75d45 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_const_value_raw
  27:     0x7f33c3b03e2d - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::const_eval_global_id
  28:     0x7f33c3b2b98a - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::const_eval_resolve
  29:     0x7f33c377151e - <rustc_trait_selection[561e35c8266e314f]::traits::query::normalize::QueryNormalizer as rustc_middle[95761bf3c41b10cf]::ty::fold::FallibleTypeFolder>::try_fold_const
  30:     0x7f33c3771856 - <rustc_trait_selection[561e35c8266e314f]::traits::query::normalize::QueryNormalizer as rustc_middle[95761bf3c41b10cf]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  31:     0x7f33c2606a5f - <rustc_infer[6814aa6b2eed40e]::infer::at::At as rustc_trait_selection[561e35c8266e314f]::traits::query::normalize::AtExt>::normalize::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  32:     0x7f33c2614a71 - <rustc_infer[6814aa6b2eed40e]::infer::InferCtxtBuilder>::enter::<core[f370780f52494a52]::result::Result<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind, rustc_middle[95761bf3c41b10cf]::traits::query::NoSolution>, rustc_traits[7167181ae6d12e6e]::normalize_erasing_regions::try_normalize_after_erasing_regions<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>::{closure#0}>
  33:     0x7f33c27158fe - <rustc_traits[7167181ae6d12e6e]::normalize_erasing_regions::provide::{closure#1} as core[f370780f52494a52]::ops::function::FnOnce<(rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt, rustc_middle[95761bf3c41b10cf]::ty::ParamEnvAnd<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>)>>::call_once
  34:     0x7f33c2b28ec0 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::try_normalize_mir_const_after_erasing_regions, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  35:     0x7f33c2e76cee - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::try_normalize_mir_const_after_erasing_regions
  36:     0x7f33c3ca4314 - <rustc_middle[95761bf3c41b10cf]::ty::normalize_erasing_regions::TryNormalizeAfterErasingRegionsFolder as rustc_middle[95761bf3c41b10cf]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  37:     0x7f33c28f5703 - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::try_normalize_erasing_regions::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  38:     0x7f33c28f6b51 - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::try_subst_and_normalize_erasing_regions::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  39:     0x7f33c279a273 - <rustc_const_eval[3bdf8a288a416b24]::interpret::eval_context::InterpCx<rustc_const_eval[3bdf8a288a416b24]::const_eval::machine::CompileTimeInterpreter>>::subst_from_current_frame_and_normalize_erasing_regions::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  40:     0x7f33c27c9eca - <rustc_const_eval[3bdf8a288a416b24]::interpret::eval_context::InterpCx<rustc_const_eval[3bdf8a288a416b24]::const_eval::machine::CompileTimeInterpreter>>::push_stack_frame
  41:     0x7f33c28724da - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_allocation_raw_provider
  42:     0x7f33c2af48be - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_allocation_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  43:     0x7f33c2e75d05 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_allocation_raw
  44:     0x7f33c2870151 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_const_value_raw_provider
  45:     0x7f33c2afe8e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_const_value_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  46:     0x7f33c2e75d45 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_const_value_raw
  47:     0x7f33c286fdc0 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_const_value_raw_provider
  48:     0x7f33c2afe8e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_const_value_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  49:     0x7f33c2e75d45 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_const_value_raw
  50:     0x7f33c3b03e2d - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::const_eval_global_id
  51:     0x7f33c3b2b98a - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::const_eval_resolve
  52:     0x7f33c37b7f7a - <rustc_trait_selection[561e35c8266e314f]::traits::project::AssocTypeNormalizer as rustc_middle[95761bf3c41b10cf]::ty::fold::TypeFolder>::fold_const
  53:     0x7f33c377e997 - <rustc_middle[95761bf3c41b10cf]::ty::subst::GenericArg as rustc_middle[95761bf3c41b10cf]::ty::fold::TypeFoldable>::try_fold_with::<rustc_trait_selection[561e35c8266e314f]::traits::project::AssocTypeNormalizer>
  54:     0x7f33c3868735 - <&rustc_middle[95761bf3c41b10cf]::ty::list::List<rustc_middle[95761bf3c41b10cf]::ty::subst::GenericArg> as rustc_middle[95761bf3c41b10cf]::ty::fold::TypeFoldable>::try_super_fold_with::<rustc_trait_selection[561e35c8266e314f]::traits::project::AssocTypeNormalizer>
  55:     0x7f33c388e1a7 - <rustc_middle[95761bf3c41b10cf]::ty::Ty as rustc_middle[95761bf3c41b10cf]::ty::fold::TypeFoldable>::super_fold_with::<rustc_trait_selection[561e35c8266e314f]::traits::project::AssocTypeNormalizer>
  56:     0x7f33c37b6ac6 - <rustc_trait_selection[561e35c8266e314f]::traits::project::AssocTypeNormalizer as rustc_middle[95761bf3c41b10cf]::ty::fold::TypeFolder>::fold_ty
  57:     0x7f33c1e1bcf3 - <rustc_trait_selection[561e35c8266e314f]::traits::project::AssocTypeNormalizer>::fold::<rustc_middle[95761bf3c41b10cf]::ty::Ty>
  58:     0x7f33c1e24cee - rustc_trait_selection[561e35c8266e314f]::traits::project::normalize::<rustc_middle[95761bf3c41b10cf]::ty::Ty>
  59:     0x7f33c1d4524e - <rustc_infer[6814aa6b2eed40e]::infer::InferCtxt as rustc_trait_selection[561e35c8266e314f]::infer::InferCtxtExt>::partially_normalize_associated_types_in::<rustc_middle[95761bf3c41b10cf]::ty::Ty>
  60:     0x7f33c1e42582 - <rustc_typeck[4d2a09a37465052a]::check::inherited::Inherited>::normalize_associated_types_in_with_cause::<rustc_middle[95761bf3c41b10cf]::ty::Ty>
  61:     0x7f33c1b2bd66 - <rustc_typeck[4d2a09a37465052a]::check::fn_ctxt::FnCtxt>::instantiate_type_scheme::<rustc_middle[95761bf3c41b10cf]::ty::Ty>
  62:     0x7f33c1b85582 - <rustc_typeck[4d2a09a37465052a]::check::fn_ctxt::FnCtxt>::instantiate_value_path
  63:     0x7f33c1b20175 - <rustc_typeck[4d2a09a37465052a]::check::fn_ctxt::FnCtxt>::check_expr_path
  64:     0x7f33c1b1f373 - <rustc_typeck[4d2a09a37465052a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  65:     0x7f33c1b097b7 - <rustc_typeck[4d2a09a37465052a]::check::fn_ctxt::FnCtxt>::check_call
  66:     0x7f33c1b77177 - <rustc_typeck[4d2a09a37465052a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  67:     0x7f33c1b1f3e7 - <rustc_typeck[4d2a09a37465052a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  68:     0x7f33c1b75fa9 - <rustc_typeck[4d2a09a37465052a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  69:     0x7f33c1b3a1a2 - <rustc_typeck[4d2a09a37465052a]::check::fn_ctxt::FnCtxt>::check_stmt
  70:     0x7f33c1b3a794 - <rustc_typeck[4d2a09a37465052a]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  71:     0x7f33c1b77422 - <rustc_typeck[4d2a09a37465052a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  72:     0x7f33c1b1f3e7 - <rustc_typeck[4d2a09a37465052a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  73:     0x7f33c1b75fa9 - <rustc_typeck[4d2a09a37465052a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  74:     0x7f33c1b207e7 - <rustc_typeck[4d2a09a37465052a]::check::fn_ctxt::FnCtxt>::check_return_expr
  75:     0x7f33c1c002e2 - rustc_typeck[4d2a09a37465052a]::check::check::check_fn
  76:     0x7f33c1d23b94 - <rustc_infer[6814aa6b2eed40e]::infer::InferCtxtBuilder>::enter::<&rustc_middle[95761bf3c41b10cf]::ty::context::TypeckResults, <rustc_typeck[4d2a09a37465052a]::check::inherited::InheritedBuilder>::enter<rustc_typeck[4d2a09a37465052a]::check::typeck_with_fallback<rustc_typeck[4d2a09a37465052a]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[95761bf3c41b10cf]::ty::context::TypeckResults>::{closure#0}>
  77:     0x7f33c1e3f54e - <rustc_typeck[4d2a09a37465052a]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[4d2a09a37465052a]::check::typeck_with_fallback<rustc_typeck[4d2a09a37465052a]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[95761bf3c41b10cf]::ty::context::TypeckResults>
  78:     0x7f33c1c848e6 - rustc_typeck[4d2a09a37465052a]::check::typeck
  79:     0x7f33c29f35e6 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::try_execute_query::<rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt, rustc_query_system[839b2ca9f9a29e23]::query::caches::DefaultCache<rustc_span[3e9cd59107943c18]::def_id::LocalDefId, &rustc_middle[95761bf3c41b10cf]::ty::context::TypeckResults>>
  80:     0x7f33c2b2d133 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::typeck, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  81:     0x7f33c1e50414 - rustc_typeck[4d2a09a37465052a]::collect::type_of::opt_const_param_of
  82:     0x7f33c29e624a - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::try_execute_query::<rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt, rustc_query_system[839b2ca9f9a29e23]::query::caches::DefaultCache<rustc_span[3e9cd59107943c18]::def_id::LocalDefId, core[f370780f52494a52]::option::Option<rustc_span[3e9cd59107943c18]::def_id::DefId>>>
  83:     0x7f33c2ade70a - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::opt_const_param_of, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  84:     0x7f33c1c84240 - rustc_typeck[4d2a09a37465052a]::check::typeck
  85:     0x7f33c29f35e6 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::try_execute_query::<rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt, rustc_query_system[839b2ca9f9a29e23]::query::caches::DefaultCache<rustc_span[3e9cd59107943c18]::def_id::LocalDefId, &rustc_middle[95761bf3c41b10cf]::ty::context::TypeckResults>>
  86:     0x7f33c2b2d133 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::typeck, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  87:     0x7f33c1bd069b - <rustc_middle[95761bf3c41b10cf]::hir::map::Map>::par_body_owners::<rustc_typeck[4d2a09a37465052a]::check::typeck_item_bodies::{closure#0}>
  88:     0x7f33c1c88dfd - rustc_typeck[4d2a09a37465052a]::check::typeck_item_bodies
  89:     0x7f33c2a3b07a - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::try_execute_query::<rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt, rustc_query_system[839b2ca9f9a29e23]::query::caches::DefaultCache<(), ()>>
  90:     0x7f33c2adee67 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::typeck_item_bodies, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  91:     0x7f33c1ca9027 - rustc_typeck[4d2a09a37465052a]::check_crate
  92:     0x7f33c13f50a1 - rustc_interface[cf714bd2a8aae84d]::passes::analysis
  93:     0x7f33c2a2f73e - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::try_execute_query::<rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt, rustc_query_system[839b2ca9f9a29e23]::query::caches::DefaultCache<(), core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>>
  94:     0x7f33c2b2dc78 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::analysis, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  95:     0x7f33c12dc2d4 - <rustc_interface[cf714bd2a8aae84d]::passes::QueryContext>::enter::<rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  96:     0x7f33c128a06e - <rustc_interface[cf714bd2a8aae84d]::interface::Compiler>::enter::<rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}::{closure#2}, core[f370780f52494a52]::result::Result<core[f370780f52494a52]::option::Option<rustc_interface[cf714bd2a8aae84d]::queries::Linker>, rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  97:     0x7f33c126cb9b - rustc_span[3e9cd59107943c18]::with_source_map::<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_interface[cf714bd2a8aae84d]::interface::create_compiler_and_run<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#1}>
  98:     0x7f33c128b209 - <scoped_tls[763d6a33bb6869db]::ScopedKey<rustc_span[3e9cd59107943c18]::SessionGlobals>>::set::<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  99:     0x7f33c12e4ec9 - std[8b1df1c4c90d5389]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf714bd2a8aae84d]::util::run_in_thread_pool_with_globals<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
 100:     0x7f33c128c5c1 - std[8b1df1c4c90d5389]::panicking::try::<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, core[f370780f52494a52]::panic::unwind_safe::AssertUnwindSafe<<std[8b1df1c4c90d5389]::thread::Builder>::spawn_unchecked_<rustc_interface[cf714bd2a8aae84d]::util::run_in_thread_pool_with_globals<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
 101:     0x7f33c12dfe02 - <<std[8b1df1c4c90d5389]::thread::Builder>::spawn_unchecked_<rustc_interface[cf714bd2a8aae84d]::util::run_in_thread_pool_with_globals<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#1} as core[f370780f52494a52]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 102:     0x7f33c07a4593 - std::sys::unix::thread::Thread::new::thread_start::h38917ce570c7f83a
 103:     0x7f33bacf7609 - start_thread
 104:     0x7f33c060a163 - clone
 105:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (4d1d3cedd 2022-04-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [eval_to_const_value_raw] simplifying constant for the type system `char_raw`
#1 [eval_to_const_value_raw] simplifying constant for the type system `char_raw`
#2 [try_normalize_mir_const_after_erasing_regions] normalizing `char_raw`
#3 [eval_to_allocation_raw] const-evaluating + checking `main::{constant#7}`
#4 [eval_to_const_value_raw] simplifying constant for the type system `main::{constant#7}`
#5 [eval_to_const_value_raw] simplifying constant for the type system `main::{constant#7}`
#6 [typeck] type-checking `main`
#7 [opt_const_param_of] computing the optional const parameter of `main::{constant#12}`
#8 [typeck] type-checking `main::{constant#12}`
#9 [typeck_item_bodies] type-checking all item bodies
#10 [analysis] running analysis passes on this crate
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
---

4 LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
5    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |         |
-    |         memory access failed: alloc7 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
+    |         memory access failed: alloc8 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
8    |         inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
10   ::: $DIR/out_of_bounds_read.rs:12:33

18 LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
19    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
19    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
20    |         |
-    |         memory access failed: alloc7 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
+    |         memory access failed: alloc8 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
22    |         inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
24   ::: $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

37 LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
38    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
38    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
39    |         |
-    |         memory access failed: alloc7 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
+    |         memory access failed: alloc8 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
41    |         inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
43   ::: $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read/out_of_bounds_read.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-ptr/out_of_bounds_read.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-ptr/out_of_bounds_read.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
   |
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         |
   |         memory access failed: alloc8 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
   |         inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:1096:9
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:12:33
   |
   |
LL |     const _READ: u32 = unsafe { ptr::read(PAST_END_PTR) };
   |                                 ----------------------- inside `_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:12:33
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/mod.rs:1096:9
   |
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         memory access failed: alloc8 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
   |         inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:1096:9
  ::: /checkout/library/core/src/ptr/const_ptr.rs:941:18
   |
   |
LL |         unsafe { read(self) }
   |                  ---------- inside `ptr::const_ptr::<impl *const u32>::read` at /checkout/library/core/src/ptr/const_ptr.rs:941:18
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:13:39
   |
   |
LL |     const _CONST_READ: u32 = unsafe { PAST_END_PTR.read() };
   |                                       ------------------- inside `_CONST_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:13:39
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/mod.rs:1096:9
   |
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         memory access failed: alloc8 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
   |         inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:1096:9
  ::: /checkout/library/core/src/ptr/mut_ptr.rs:1055:18
   |
   |
LL |         unsafe { read(self) }
   |                  ---------- inside `ptr::mut_ptr::<impl *mut u32>::read` at /checkout/library/core/src/ptr/mut_ptr.rs:1055:18
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:14:37
   |
   |
LL |     const _MUT_READ: u32 = unsafe { (PAST_END_PTR as *mut u32).read() };
   |                                     --------------------------------- inside `_MUT_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:14:37
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs stdout ----
diff of 64bit.stderr:

47   --> $DIR/const-pointer-values-in-various-types.rs:41:5
48    |
49 LL |     const I32_REF_U64_UNION: u64 = unsafe { Nonsense { int_32_ref: &3 }.uint_64 };
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc19, but expected initialized plain (non-pointer) bytes
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc20, but expected initialized plain (non-pointer) bytes
51    |
52    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
53    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc19───────╼                         │ ╾──────╼
+                ╾───────alloc20───────╼                         │ ╾──────╼
55            }
56 
56 
57 error[E0080]: it is undefined behavior to use this value

102   --> $DIR/const-pointer-values-in-various-types.rs:59:5
103    |
104 LL |     const I32_REF_I64_UNION: i64 = unsafe { Nonsense { int_32_ref: &3 }.int_64 };
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc39, but expected initialized plain (non-pointer) bytes
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc40, but expected initialized plain (non-pointer) bytes
106    |
107    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
108    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc39───────╼                         │ ╾──────╼
+                ╾───────alloc40───────╼                         │ ╾──────╼
110            }
111 
111 
112 error[E0080]: it is undefined behavior to use this value

135   --> $DIR/const-pointer-values-in-various-types.rs:69:5
136    |
137 LL |     const I32_REF_F64_UNION: f64 = unsafe { Nonsense { int_32_ref: &3 }.float_64 };
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc51, but expected initialized plain (non-pointer) bytes
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc52, but expected initialized plain (non-pointer) bytes
139    |
140    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
141    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc51───────╼                         │ ╾──────╼
+                ╾───────alloc52───────╼                         │ ╾──────╼
143            }
144 
144 
145 error: any use of this value will cause an error

201   --> $DIR/const-pointer-values-in-various-types.rs:92:5
202    |
203 LL |     const STR_U64_UNION: u64 = unsafe { Nonsense { stringy: "3" }.uint_64 };
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc72, but expected initialized plain (non-pointer) bytes
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc73, but expected initialized plain (non-pointer) bytes
205    |
206    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
207    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc72───────╼                         │ ╾──────╼
+                ╾───────alloc73───────╼                         │ ╾──────╼
209            }
210 
210 
211 error: any use of this value will cause an error

256   --> $DIR/const-pointer-values-in-various-types.rs:111:5
257    |
258 LL |     const STR_I64_UNION: i64 = unsafe { Nonsense { stringy: "3" }.int_64 };
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc87, but expected initialized plain (non-pointer) bytes
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc88, but expected initialized plain (non-pointer) bytes
260    |
261    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
262    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc87───────╼                         │ ╾──────╼
+                ╾───────alloc88───────╼                         │ ╾──────╼
264            }
265 
265 
266 error: any use of this value will cause an error

289   --> $DIR/const-pointer-values-in-various-types.rs:122:5
290    |
291 LL |     const STR_F64_UNION: f64 = unsafe { Nonsense { stringy: "3" }.float_64 };
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc96, but expected initialized plain (non-pointer) bytes
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc97, but expected initialized plain (non-pointer) bytes
293    |
294    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
295    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc96───────╼                         │ ╾──────╼
+                ╾───────alloc97───────╼                         │ ╾──────╼
297            }
298 
298 
299 error: any use of this value will cause an error


The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-pointer-values-in-various-types/const-pointer-values-in-various-types.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/const-pointer-values-in-various-types.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-pointer-values-in-various-types" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-pointer-values-in-various-types/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL |     const I32_REF_USIZE_UNION: usize = unsafe { Nonsense { int_32_ref: &3 }.u };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc3, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:29:43
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:29:43
   |
LL |     const I32_REF_U8_UNION: u8 = unsafe { Nonsense { int_32_ref: &3 }.uint_8 };
   |                                           |
   |                                           |
   |                                           unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:33:45
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:33:45
   |
LL |     const I32_REF_U16_UNION: u16 = unsafe { Nonsense { int_32_ref: &3 }.uint_16 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:37:45
   |
LL |     const I32_REF_U32_UNION: u32 = unsafe { Nonsense { int_32_ref: &3 }.uint_32 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:41:5
   |
LL |     const I32_REF_U64_UNION: u64 = unsafe { Nonsense { int_32_ref: &3 }.uint_64 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc20, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:44:5
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:44:5
   |
LL |     const I32_REF_U128_UNION: u128 = unsafe { Nonsense { int_32_ref: &3 }.uint_128 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ │ ░░░░░░░░░░░░░░░░

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:47:43
   |
   |
LL |     const I32_REF_I8_UNION: i8 = unsafe { Nonsense { int_32_ref: &3 }.int_8 };
   |                                           |
   |                                           |
   |                                           unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:51:45
   |
LL |     const I32_REF_I16_UNION: i16 = unsafe { Nonsense { int_32_ref: &3 }.int_16 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:55:45
   |
LL |     const I32_REF_I32_UNION: i32 = unsafe { Nonsense { int_32_ref: &3 }.int_32 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:59:5
   |
LL |     const I32_REF_I64_UNION: i64 = unsafe { Nonsense { int_32_ref: &3 }.int_64 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc40, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:62:5
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:62:5
   |
LL |     const I32_REF_I128_UNION: i128 = unsafe { Nonsense { int_32_ref: &3 }.int_128 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ │ ░░░░░░░░░░░░░░░░

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:65:45
   |
   |
LL |     const I32_REF_F32_UNION: f32 = unsafe { Nonsense { int_32_ref: &3 }.float_32 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:69:5
   |
LL |     const I32_REF_F64_UNION: f64 = unsafe { Nonsense { int_32_ref: &3 }.float_64 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc52, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:72:47
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:72:47
   |
LL |     const I32_REF_BOOL_UNION: bool = unsafe { Nonsense { int_32_ref: &3 }.truthy_falsey };
   |                                               |
   |                                               |
   |                                               unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:76:47
   |
LL |     const I32_REF_CHAR_UNION: char = unsafe { Nonsense { int_32_ref: &3 }.character };
   |                                               |
   |                                               |
   |                                               unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:80:39
   |
LL |     const STR_U8_UNION: u8 = unsafe { Nonsense { stringy: "3" }.uint_8 };
   |                                       |
   |                                       |
   |                                       unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:84:41
   |
LL |     const STR_U16_UNION: u16 = unsafe { Nonsense { stringy: "3" }.uint_16 };
   |                                         |
   |                                         |
   |                                         unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:88:41
   |
LL |     const STR_U32_UNION: u32 = unsafe { Nonsense { stringy: "3" }.uint_32 };
   |                                         |
   |                                         |
   |                                         unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:92:5
   |
LL |     const STR_U64_UNION: u64 = unsafe { Nonsense { stringy: "3" }.uint_64 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc73, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:95:43
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:95:43
   |
LL |     const STR_U128_UNION: u128 = unsafe { Nonsense { stringy: "3" }.uint_128 };
   |                                           |
   |                                           |
   |                                           unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:99:39
   |
LL |     const STR_I8_UNION: i8 = unsafe { Nonsense { stringy: "3" }.int_8 };
   |                                       |
   |                                       |
   |                                       unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:103:41
   |
LL |     const STR_I16_UNION: i16 = unsafe { Nonsense { stringy: "3" }.int_16 };
   |                                         |
   |                                         |
   |                                         unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:107:41
   |
LL |     const STR_I32_UNION: i32 = unsafe { Nonsense { stringy: "3" }.int_32 };
   |                                         |
   |                                         |
   |                                         unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:111:5
   |
LL |     const STR_I64_UNION: i64 = unsafe { Nonsense { stringy: "3" }.int_64 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc88, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:114:43
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:114:43
   |
LL |     const STR_I128_UNION: i128 = unsafe { Nonsense { stringy: "3" }.int_128 };
   |                                           |
   |                                           |
   |                                           unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:118:41
   |
LL |     const STR_F32_UNION: f32 = unsafe { Nonsense { stringy: "3" }.float_32 };
   |                                         |
   |                                         |
   |                                         unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:122:5
   |
LL |     const STR_F64_UNION: f64 = unsafe { Nonsense { stringy: "3" }.float_64 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc97, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:125:43
---

---- [ui] src/test/ui/consts/const-eval/partial_ptr_overwrite.rs stdout ----
diff of stderr:

6 LL | |     unsafe {
7 LL | |         let ptr: *mut _ = &mut p;
8 LL | |         *(ptr as *mut u8) = 123;
-    | |         ^^^^^^^^^^^^^^^^^^^^^^^ unable to overwrite parts of a pointer in memory at alloc4
+    | |         ^^^^^^^^^^^^^^^^^^^^^^^ unable to overwrite parts of a pointer in memory at alloc5
11 LL | |     let x = *p;
12 LL | | };



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/partial_ptr_overwrite/partial_ptr_overwrite.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/partial_ptr_overwrite.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/partial_ptr_overwrite.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/partial_ptr_overwrite" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/partial_ptr_overwrite/auxiliary"
stdout: none
--- stderr -------------------------------
error: any use of this value will cause an error
   |
   |
LL | / const PARTIAL_OVERWRITE: () = {
LL | |     let mut p = &42;
LL | |     unsafe {
LL | |         let ptr: *mut _ = &mut p;
LL | |         *(ptr as *mut u8) = 123; //~ ERROR any use of this value
   | |         ^^^^^^^^^^^^^^^^^^^^^^^ unable to overwrite parts of a pointer in memory at alloc5
LL | |     let x = *p;
LL | | };
   | |__-
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to previous error
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/promote-static.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promote-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promote-static" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promote-static/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: InterpErrorInfo(InterpErrorInfoInner { kind: constant accesses static, backtrace: None })', compiler/rustc_const_eval/src/const_eval/valtrees.rs:115:64
stack backtrace:
   0:     0x7f11b4cf09ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7a8d9e17c5c41d63
   1:     0x7f11b4d56ff8 - core::fmt::write::he96e6c1da7790e01
   2:     0x7f11b4ce0811 - std::io::Write::write_fmt::he213735f9fc023bf
   3:     0x7f11b4cf39de - std::panicking::default_hook::{{closure}}::h1db1d872e7cc6dba
   4:     0x7f11b4cf360c - std::panicking::default_hook::h117fb00cf01e902c
   5:     0x7f11b5854ba1 - rustc_driver[f41c55b70dce3ba2]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f11b4cf423e - std::panicking::rust_panic_with_hook::hfb6238818b193c9a
   7:     0x7f11b4cf4037 - std::panicking::begin_panic_handler::{{closure}}::h1c134276a38e45ce
   8:     0x7f11b4cf0ec4 - std::sys_common::backtrace::__rust_end_short_backtrace::h3511975eefed2e40
   9:     0x7f11b4cf3d19 - rust_begin_unwind
  10:     0x7f11b4ca8073 - core::panicking::panic_fmt::hb74e78879777d5d2
  11:     0x7f11b4ca8273 - core::result::unwrap_failed::h943c676139e14f19
  12:     0x7f11b6dd6435 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::const_to_valtree_inner
  13:     0x7f11b6dd58d9 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::const_to_valtree_inner
  14:     0x7f11b6dd58d9 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::const_to_valtree_inner
  15:     0x7f11b6dd4269 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::const_to_valtree
  16:     0x7f11b6dcb630 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_const_value_raw_provider
  17:     0x7f11b70598e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_const_value_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  18:     0x7f11b73d0d45 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_const_value_raw
  19:     0x7f11b6dcadc0 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_const_value_raw_provider
  20:     0x7f11b70598e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_const_value_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  21:     0x7f11b73d0d45 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_const_value_raw
  22:     0x7f11b805ee2d - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::const_eval_global_id
  23:     0x7f11b808698a - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::const_eval_resolve
  24:     0x7f11b7ccc51e - <rustc_trait_selection[561e35c8266e314f]::traits::query::normalize::QueryNormalizer as rustc_middle[95761bf3c41b10cf]::ty::fold::FallibleTypeFolder>::try_fold_const
  25:     0x7f11b7ccc856 - <rustc_trait_selection[561e35c8266e314f]::traits::query::normalize::QueryNormalizer as rustc_middle[95761bf3c41b10cf]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  26:     0x7f11b6b61a5f - <rustc_infer[6814aa6b2eed40e]::infer::at::At as rustc_trait_selection[561e35c8266e314f]::traits::query::normalize::AtExt>::normalize::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  27:     0x7f11b6b6fa71 - <rustc_infer[6814aa6b2eed40e]::infer::InferCtxtBuilder>::enter::<core[f370780f52494a52]::result::Result<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind, rustc_middle[95761bf3c41b10cf]::traits::query::NoSolution>, rustc_traits[7167181ae6d12e6e]::normalize_erasing_regions::try_normalize_after_erasing_regions<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>::{closure#0}>
  28:     0x7f11b6c708fe - <rustc_traits[7167181ae6d12e6e]::normalize_erasing_regions::provide::{closure#1} as core[f370780f52494a52]::ops::function::FnOnce<(rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt, rustc_middle[95761bf3c41b10cf]::ty::ParamEnvAnd<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>)>>::call_once
  29:     0x7f11b7083ec0 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::try_normalize_mir_const_after_erasing_regions, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  30:     0x7f11b73d1cee - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::try_normalize_mir_const_after_erasing_regions
  31:     0x7f11b81ff314 - <rustc_middle[95761bf3c41b10cf]::ty::normalize_erasing_regions::TryNormalizeAfterErasingRegionsFolder as rustc_middle[95761bf3c41b10cf]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  32:     0x7f11b6e50703 - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::try_normalize_erasing_regions::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  33:     0x7f11b6e51b51 - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::try_subst_and_normalize_erasing_regions::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  34:     0x7f11b6cf5273 - <rustc_const_eval[3bdf8a288a416b24]::interpret::eval_context::InterpCx<rustc_const_eval[3bdf8a288a416b24]::const_eval::machine::CompileTimeInterpreter>>::subst_from_current_frame_and_normalize_erasing_regions::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  35:     0x7f11b6d0a70f - <rustc_const_eval[3bdf8a288a416b24]::interpret::eval_context::InterpCx<rustc_const_eval[3bdf8a288a416b24]::const_eval::machine::CompileTimeInterpreter>>::eval_operand
  36:     0x7f11b6d1716d - <rustc_const_eval[3bdf8a288a416b24]::interpret::eval_context::InterpCx<rustc_const_eval[3bdf8a288a416b24]::const_eval::machine::CompileTimeInterpreter>>::eval_rvalue_into_place
  37:     0x7f11b6d13672 - <rustc_const_eval[3bdf8a288a416b24]::interpret::eval_context::InterpCx<rustc_const_eval[3bdf8a288a416b24]::const_eval::machine::CompileTimeInterpreter>>::run
  38:     0x7f11b6dcd4f3 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_allocation_raw_provider
  39:     0x7f11b704f8be - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_allocation_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  40:     0x7f11b73d0d05 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_allocation_raw
  41:     0x7f11b6dccd29 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_allocation_raw_provider
  42:     0x7f11b704f8be - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_allocation_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  43:     0x7f11b73d0d05 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_allocation_raw
  44:     0x7f11b805f7c0 - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::eval_static_initializer
  45:     0x7f11b7bdcfbe - <rustc_lint[33f320154ce7d8a2]::builtin::UnusedBrokenConst as rustc_lint[33f320154ce7d8a2]::passes::LateLintPass>::check_item
  46:     0x7f11b7b9f144 - <rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass as rustc_lint[33f320154ce7d8a2]::passes::LateLintPass>::check_item
  47:     0x7f11b59c9d8d - <rustc_lint[33f320154ce7d8a2]::late::LateContextAndPass<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass> as rustc_hir[deb440e854090368]::intravisit::Visitor>::visit_nested_item
  48:     0x7f11b59d24cc - rustc_hir[deb440e854090368]::intravisit::walk_mod::<rustc_lint[33f320154ce7d8a2]::late::LateContextAndPass<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass>>
  49:     0x7f11b59c67bc - <rustc_lint[33f320154ce7d8a2]::late::LateContextAndPass<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass> as rustc_hir[deb440e854090368]::intravisit::Visitor>::visit_mod
  50:     0x7f11b59bc998 - rustc_lint[33f320154ce7d8a2]::late::late_lint_crate::<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass>
  51:     0x7f11b596899b - <rustc_session[9ce44d69e2e9f546]::session::Session>::time::<(), rustc_lint[33f320154ce7d8a2]::late::check_crate<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass, rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}::{closure#0}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}::{closure#0}>
  52:     0x7f11b5951982 - rustc_data_structures[3cf013d82b52e206]::sync::join::<rustc_lint[33f320154ce7d8a2]::late::check_crate<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass, rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}::{closure#0}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}, rustc_lint[33f320154ce7d8a2]::late::check_crate<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass, rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}::{closure#0}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}, (), ()>
  53:     0x7f11b5968af9 - <rustc_session[9ce44d69e2e9f546]::session::Session>::time::<(), rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}::{closure#0}::{closure#2}::{closure#0}>
  54:     0x7f11b596d305 - std[8b1df1c4c90d5389]::panicking::try::<(), core[f370780f52494a52]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}::{closure#0}::{closure#2}>>
  55:     0x7f11b59221bb - <core[f370780f52494a52]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}::{closure#0}> as core[f370780f52494a52]::ops::function::FnOnce<()>>::call_once
  56:     0x7f11b596a731 - <rustc_session[9ce44d69e2e9f546]::session::Session>::time::<(), rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}>
  57:     0x7f11b595013c - rustc_interface[cf714bd2a8aae84d]::passes::analysis
  58:     0x7f11b6f8a73e - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::try_execute_query::<rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt, rustc_query_system[839b2ca9f9a29e23]::query::caches::DefaultCache<(), core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>>
  59:     0x7f11b7088c78 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::analysis, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  60:     0x7f11b58372d4 - <rustc_interface[cf714bd2a8aae84d]::passes::QueryContext>::enter::<rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  61:     0x7f11b57e506e - <rustc_interface[cf714bd2a8aae84d]::interface::Compiler>::enter::<rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}::{closure#2}, core[f370780f52494a52]::result::Result<core[f370780f52494a52]::option::Option<rustc_interface[cf714bd2a8aae84d]::queries::Linker>, rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  62:     0x7f11b57c7b9b - rustc_span[3e9cd59107943c18]::with_source_map::<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_interface[cf714bd2a8aae84d]::interface::create_compiler_and_run<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#1}>
  63:     0x7f11b57e6209 - <scoped_tls[763d6a33bb6869db]::ScopedKey<rustc_span[3e9cd59107943c18]::SessionGlobals>>::set::<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  64:     0x7f11b583fec9 - std[8b1df1c4c90d5389]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf714bd2a8aae84d]::util::run_in_thread_pool_with_globals<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  65:     0x7f11b57e75c1 - std[8b1df1c4c90d5389]::panicking::try::<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, core[f370780f52494a52]::panic::unwind_safe::AssertUnwindSafe<<std[8b1df1c4c90d5389]::thread::Builder>::spawn_unchecked_<rustc_interface[cf714bd2a8aae84d]::util::run_in_thread_pool_with_globals<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  66:     0x7f11b583ae02 - <<std[8b1df1c4c90d5389]::thread::Builder>::spawn_unchecked_<rustc_interface[cf714bd2a8aae84d]::util::run_in_thread_pool_with_globals<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#1} as core[f370780f52494a52]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  67:     0x7f11b4cff593 - std::sys::unix::thread::Thread::new::thread_start::h38917ce570c7f83a
  68:     0x7f11af252609 - start_thread
  69:     0x7f11b4b65163 - clone
  70:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (4d1d3cedd 2022-04-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [eval_to_const_value_raw] simplifying constant for the type system `NONE_REF_REF::promoted[0]`
#1 [eval_to_const_value_raw] simplifying constant for the type system `NONE_REF_REF::promoted[0]`
#2 [try_normalize_mir_const_after_erasing_regions] normalizing `NONE_REF_REF::promoted[0]`
#3 [eval_to_allocation_raw] const-evaluating + checking `NONE_REF_REF`
#4 [eval_to_allocation_raw] const-evaluating + checking `NONE_REF_REF`
#5 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/ub-enum.rs stdout ----
---- [ui] src/test/ui/consts/const-eval/ub-enum.rs stdout ----
diff of 64bit.stderr:

13   --> $DIR/ub-enum.rs:27:1
14    |
15 LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered pointer to alloc9, but expected initialized plain (non-pointer) bytes
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered pointer to alloc10, but expected initialized plain (non-pointer) bytes
17    |
18    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
19    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc9────────╼                         │ ╾──────╼
+                ╾───────alloc10───────╼                         │ ╾──────╼
21            }
22 
22 
23 error[E0080]: it is undefined behavior to use this value

24   --> $DIR/ub-enum.rs:30:1
25    |
26 LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0.<enum-tag>: encountered pointer to alloc13, but expected initialized plain (non-pointer) bytes
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0.<enum-tag>: encountered pointer to alloc15, but expected initialized plain (non-pointer) bytes
28    |
29    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
30    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc13───────╼                         │ ╾──────╼
+                ╾───────alloc15───────╼                         │ ╾──────╼
32            }
33 
33 
34 error[E0080]: it is undefined behavior to use this value

46   --> $DIR/ub-enum.rs:44:1
47    |
48 LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered pointer to alloc19, but expected initialized plain (non-pointer) bytes
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered pointer to alloc21, but expected initialized plain (non-pointer) bytes
50    |
51    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
52    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc19───────╼                         │ ╾──────╼
+                ╾───────alloc21───────╼                         │ ╾──────╼
54            }
55 
55 
56 error[E0080]: it is undefined behavior to use this value

57   --> $DIR/ub-enum.rs:47:1
58    |
59 LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0.<enum-tag>: encountered pointer to alloc23, but expected initialized plain (non-pointer) bytes
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0.<enum-tag>: encountered pointer to alloc26, but expected initialized plain (non-pointer) bytes
61    |
62    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
63    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc23───────╼                         │ ╾──────╼
+                ╾───────alloc26───────╼                         │ ╾──────╼
65            }
66 
66 
67 error[E0080]: it is undefined behavior to use this value

79   --> $DIR/ub-enum.rs:60:1
80    |
81 LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered pointer to alloc30, but expected initialized plain (non-pointer) bytes
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered pointer to alloc33, but expected initialized plain (non-pointer) bytes
83    |
84    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
85    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc30───────╼                         │ ╾──────╼
+                ╾───────alloc33───────╼                         │ ╾──────╼
87            }
88 
88 
89 error[E0080]: it is undefined behavior to use this value


The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/ub-enum.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const BAD_ENUM: Enum = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered 0x0000000000000001, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:27:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:27:1
   |
LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered pointer to alloc10, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:30:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:30:1
   |
LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0.<enum-tag>: encountered pointer to alloc15, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:42:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:42:1
   |
LL | const BAD_ENUM2: Enum2 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered 0x0000000000000000, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:44:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:44:1
   |
LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered pointer to alloc21, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:47:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:47:1
   |
LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0.<enum-tag>: encountered pointer to alloc26, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:56:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:56:1
   |
LL | const BAD_ENUM2_UNDEF : Enum2 = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               __ __ __ __ __ __ __ __                         │ ░░░░░░░░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:60:1
   |
   |
LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered pointer to alloc33, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:77:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:77:1
   |
LL | const BAD_UNINHABITED_VARIANT1: UninhDiscriminant = unsafe { mem::transmute(1u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-variant(B)>.0: encountered a value of the never type `!`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:79:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:79:1
   |
LL | const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-variant(D)>.0: encountered a value of uninhabited type Never
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:87:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:87:1
   |
LL | const BAD_OPTION_CHAR: Option<(char, char)> = Some(('x', unsafe { mem::transmute(!0u32) }));
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-variant(Some)>.0.1: encountered 0xffffffff, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               78 00 00 00 ff ff ff ff                         │ x.......

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:92:77
   |
   |
LL | const BAD_UNINHABITED_WITH_DATA1: Result<(i32, Never), (i32, !)> = unsafe { mem::transmute(0u64) };
   |                                                                             ^^^^^^^^^^^^^^^^^^^^ transmuting to uninhabited type
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:94:77
   |
   |
LL | const BAD_UNINHABITED_WITH_DATA2: Result<(i32, !), (i32, Never)> = unsafe { mem::transmute(0u64) };
   |                                                                             ^^^^^^^^^^^^^^^^^^^^ transmuting to uninhabited type
error: aborting due to 13 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/ub-ref-ptr.rs stdout ----
diff of 64bit.stderr:

17    |
18    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
19    = note: the raw bytes of the constant (size: 8, align: 8) {
+                ╾───────alloc8────────╼                         │ ╾──────╼
21            }
22 
23 error[E0080]: it is undefined behavior to use this value
23 error[E0080]: it is undefined behavior to use this value

46   --> $DIR/ub-ref-ptr.rs:31:1
47    |
48 LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc15, but expected initialized plain (non-pointer) bytes
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc16, but expected initialized plain (non-pointer) bytes
50    |
51    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
52    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc15───────╼                         │ ╾──────╼
+                ╾───────alloc16───────╼                         │ ╾──────╼
54            }
55 
55 
56 error[E0080]: it is undefined behavior to use this value

61    |
62    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
63    = note: the raw bytes of the constant (size: 8, align: 8) {
+                ╾───────alloc23───────╼                         │ ╾──────╼
65            }
66 
67 error[E0080]: it is undefined behavior to use this value
67 error[E0080]: it is undefined behavior to use this value

72    |
73    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
74    = note: the raw bytes of the constant (size: 8, align: 8) {
+                ╾───────alloc28───────╼                         │ ╾──────╼
76            }
77 
78 error[E0080]: it is undefined behavior to use this value
78 error[E0080]: it is undefined behavior to use this value

145   --> $DIR/ub-ref-ptr.rs:55:1
146    |
147 LL | const DATA_FN_PTR: fn() = unsafe { mem::transmute(&13) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc43, but expected a function pointer
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc45, but expected a function pointer
149    |
150    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
151    = note: the raw bytes of the constant (size: 8, align: 8) {
-                ╾───────alloc43───────╼                         │ ╾──────╼
+                ╾───────alloc45───────╼                         │ ╾──────╼
153            }
154 
154 
155 error: aborting due to 14 previous errors


The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref-ptr/ub-ref-ptr.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-ref-ptr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref-ptr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref-ptr/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const UNALIGNED: &u16 = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:17:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:17:1
   |
LL | const UNALIGNED_BOX: Box<u16> = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned box (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:21:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:21:1
   |
LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a null reference
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:24:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:24:1
   |
LL | const NULL_BOX: Box<u16> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a null box
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:31:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:31:1
   |
LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc16, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }
---
diff of 64bit.stderr:

13   --> $DIR/ub-nonnull.rs:19:30
14    |
15 LL |     let out_of_bounds_ptr = &ptr[255];
-    |                              ^^^^^^^^ dereferencing pointer failed: alloc11 has size 1, so pointer to 256 bytes starting at offset 0 is out-of-bounds
+    |                              ^^^^^^^^ dereferencing pointer failed: alloc12 has size 1, so pointer to 256 bytes starting at offset 0 is out-of-bounds
18 error[E0080]: it is undefined behavior to use this value
19   --> $DIR/ub-nonnull.rs:23:1



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/ub-nonnull.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-nonnull.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-nonnull.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const NULL_PTR: NonNull<u8> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:19:30
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:19:30
   |
LL |     let out_of_bounds_ptr = &ptr[255]; //~ ERROR evaluation of constant value failed
   |                              ^^^^^^^^ dereferencing pointer failed: alloc12 has size 1, so pointer to 256 bytes starting at offset 0 is out-of-bounds
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:23:1
   |
   |
LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:25:1
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:25:1
   |
LL | const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:33:1
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:33:1
   |
LL | const UNINIT: NonZeroU8 = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
               __                                              │ ░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:41:1
   |
   |
LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 42, but expected something in the range 10..=30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               2a 00 00 00                                     │ *...

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:47:1
   |
   |
LL | const BAD_RANGE2: RestrictedRange2 = unsafe { RestrictedRange2(20) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 20, but expected something less or equal to 10, or greater or equal to 30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: aborting due to 7 previous errors


For more information about this error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/union-const-eval-field.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/union-const-eval-field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-const-eval-field" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-const-eval-field/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: InterpErrorInfo(InterpErrorInfoInner { kind: using uninitialized data, but this operation requires initialized memory, backtrace: None })', compiler/rustc_const_eval/src/const_eval/valtrees.rs:79:39
stack backtrace:
   0:     0x7f872a35a9ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7a8d9e17c5c41d63
   1:     0x7f872a3c0ff8 - core::fmt::write::he96e6c1da7790e01
   2:     0x7f872a34a811 - std::io::Write::write_fmt::he213735f9fc023bf
   3:     0x7f872a35d9de - std::panicking::default_hook::{{closure}}::h1db1d872e7cc6dba
   4:     0x7f872a35d60c - std::panicking::default_hook::h117fb00cf01e902c
   5:     0x7f872aebeba1 - rustc_driver[f41c55b70dce3ba2]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f872a35e23e - std::panicking::rust_panic_with_hook::hfb6238818b193c9a
   7:     0x7f872a35e037 - std::panicking::begin_panic_handler::{{closure}}::h1c134276a38e45ce
   8:     0x7f872a35aec4 - std::sys_common::backtrace::__rust_end_short_backtrace::h3511975eefed2e40
   9:     0x7f872a35dd19 - rust_begin_unwind
  10:     0x7f872a312073 - core::panicking::panic_fmt::hb74e78879777d5d2
  11:     0x7f872a312273 - core::result::unwrap_failed::h943c676139e14f19
  12:     0x7f872c44019d - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::const_to_valtree_inner
  13:     0x7f872c3c2d58 - <core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::try_fold::<(), <core[f370780f52494a52]::iter::adapters::GenericShunt<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::try_fold<(), core[f370780f52494a52]::iter::traits::iterator::Iterator::try_for_each::call<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>::Break>::{closure#0}, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>::{closure#0}, core[f370780f52494a52]::ops::control_flow::ControlFlow<core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>>
  14:     0x7f872c42b02b - <core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::try_fold::<(), <core[f370780f52494a52]::iter::adapters::GenericShunt<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::try_fold<(), core[f370780f52494a52]::iter::traits::iterator::Iterator::try_for_each::call<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>::Break>::{closure#0}, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>::{closure#0}, core[f370780f52494a52]::ops::control_flow::ControlFlow<core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>>
  15:     0x7f872c3c745b - <core[f370780f52494a52]::iter::adapters::GenericShunt<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::next
  16:     0x7f872c4ae564 - <alloc[57f2ea052be60ee9]::vec::Vec<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree> as alloc[57f2ea052be60ee9]::vec::spec_from_iter::SpecFromIter<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree, core[f370780f52494a52]::iter::adapters::GenericShunt<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>>>>::from_iter
  17:     0x7f872c3bfb49 - core[f370780f52494a52]::iter::adapters::try_process::<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>, <core[f370780f52494a52]::option::Option<alloc[57f2ea052be60ee9]::vec::Vec<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>> as core[f370780f52494a52]::iter::traits::collect::FromIterator<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>>::from_iter<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>>::{closure#0}, alloc[57f2ea052be60ee9]::vec::Vec<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>
  18:     0x7f872c43e964 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches
  19:     0x7f872c43f9a2 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::const_to_valtree_inner
  20:     0x7f872c43e269 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::const_to_valtree
  21:     0x7f872c435630 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_const_value_raw_provider
  22:     0x7f872c6c38e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_const_value_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  23:     0x7f872ca3ad45 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_const_value_raw
  24:     0x7f872c434dc0 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_const_value_raw_provider
  25:     0x7f872c6c38e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_const_value_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  26:     0x7f872ca3ad45 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_const_value_raw
  27:     0x7f872d6c8e2d - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::const_eval_global_id
  28:     0x7f872d6f098a - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::const_eval_resolve
  29:     0x7f872d33651e - <rustc_trait_selection[561e35c8266e314f]::traits::query::normalize::QueryNormalizer as rustc_middle[95761bf3c41b10cf]::ty::fold::FallibleTypeFolder>::try_fold_const
  30:     0x7f872d336856 - <rustc_trait_selection[561e35c8266e314f]::traits::query::normalize::QueryNormalizer as rustc_middle[95761bf3c41b10cf]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  31:     0x7f872c1cba5f - <rustc_infer[6814aa6b2eed40e]::infer::at::At as rustc_trait_selection[561e35c8266e314f]::traits::query::normalize::AtExt>::normalize::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  32:     0x7f872c1d9a71 - <rustc_infer[6814aa6b2eed40e]::infer::InferCtxtBuilder>::enter::<core[f370780f52494a52]::result::Result<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind, rustc_middle[95761bf3c41b10cf]::traits::query::NoSolution>, rustc_traits[7167181ae6d12e6e]::normalize_erasing_regions::try_normalize_after_erasing_regions<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>::{closure#0}>
  33:     0x7f872c2da8fe - <rustc_traits[7167181ae6d12e6e]::normalize_erasing_regions::provide::{closure#1} as core[f370780f52494a52]::ops::function::FnOnce<(rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt, rustc_middle[95761bf3c41b10cf]::ty::ParamEnvAnd<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>)>>::call_once
  34:     0x7f872c6edec0 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::try_normalize_mir_const_after_erasing_regions, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  35:     0x7f872ca3bcee - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::try_normalize_mir_const_after_erasing_regions
  36:     0x7f872d869314 - <rustc_middle[95761bf3c41b10cf]::ty::normalize_erasing_regions::TryNormalizeAfterErasingRegionsFolder as rustc_middle[95761bf3c41b10cf]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  37:     0x7f872c4ba703 - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::try_normalize_erasing_regions::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  38:     0x7f872c4bbb51 - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::try_subst_and_normalize_erasing_regions::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  39:     0x7f872c35f273 - <rustc_const_eval[3bdf8a288a416b24]::interpret::eval_context::InterpCx<rustc_const_eval[3bdf8a288a416b24]::const_eval::machine::CompileTimeInterpreter>>::subst_from_current_frame_and_normalize_erasing_regions::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  40:     0x7f872c38eeca - <rustc_const_eval[3bdf8a288a416b24]::interpret::eval_context::InterpCx<rustc_const_eval[3bdf8a288a416b24]::const_eval::machine::CompileTimeInterpreter>>::push_stack_frame
  41:     0x7f872c4374da - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_allocation_raw_provider
  42:     0x7f872c6b98be - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_allocation_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  43:     0x7f872ca3ad05 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_allocation_raw
  44:     0x7f872c435151 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_const_value_raw_provider
  45:     0x7f872c6c38e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_const_value_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  46:     0x7f872ca3ad45 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_const_value_raw
  47:     0x7f872c434dc0 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_const_value_raw_provider
  48:     0x7f872c6c38e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_const_value_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  49:     0x7f872ca3ad45 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_const_value_raw
  50:     0x7f872d6c8e2d - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::const_eval_global_id
  51:     0x7f872d6f098a - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::const_eval_resolve
  52:     0x7f872d33651e - <rustc_trait_selection[561e35c8266e314f]::traits::query::normalize::QueryNormalizer as rustc_middle[95761bf3c41b10cf]::ty::fold::FallibleTypeFolder>::try_fold_const
  53:     0x7f872d336856 - <rustc_trait_selection[561e35c8266e314f]::traits::query::normalize::QueryNormalizer as rustc_middle[95761bf3c41b10cf]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  54:     0x7f872c1cba5f - <rustc_infer[6814aa6b2eed40e]::infer::at::At as rustc_trait_selection[561e35c8266e314f]::traits::query::normalize::AtExt>::normalize::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  55:     0x7f872c1d9a71 - <rustc_infer[6814aa6b2eed40e]::infer::InferCtxtBuilder>::enter::<core[f370780f52494a52]::result::Result<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind, rustc_middle[95761bf3c41b10cf]::traits::query::NoSolution>, rustc_traits[7167181ae6d12e6e]::normalize_erasing_regions::try_normalize_after_erasing_regions<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>::{closure#0}>
  56:     0x7f872c2da8fe - <rustc_traits[7167181ae6d12e6e]::normalize_erasing_regions::provide::{closure#1} as core[f370780f52494a52]::ops::function::FnOnce<(rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt, rustc_middle[95761bf3c41b10cf]::ty::ParamEnvAnd<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>)>>::call_once
  57:     0x7f872c6edec0 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::try_normalize_mir_const_after_erasing_regions, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  58:     0x7f872ca3bcee - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::try_normalize_mir_const_after_erasing_regions
  59:     0x7f872d869314 - <rustc_middle[95761bf3c41b10cf]::ty::normalize_erasing_regions::TryNormalizeAfterErasingRegionsFolder as rustc_middle[95761bf3c41b10cf]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  60:     0x7f872b5d11b3 - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::try_normalize_erasing_regions::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  61:     0x7f872b5d2bc1 - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::try_subst_and_normalize_erasing_regions::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  62:     0x7f872b382c03 - <rustc_const_eval[3bdf8a288a416b24]::interpret::eval_context::InterpCx<rustc_mir_transform[596f0f775876c0b2]::const_prop_lint::ConstPropMachine>>::subst_from_current_frame_and_normalize_erasing_regions::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  63:     0x7f872b3cab32 - <rustc_const_eval[3bdf8a288a416b24]::interpret::eval_context::InterpCx<rustc_mir_transform[596f0f775876c0b2]::const_prop_lint::ConstPropMachine>>::eval_operand
  64:     0x7f872b3a16b4 - <rustc_const_eval[3bdf8a288a416b24]::interpret::eval_context::InterpCx<rustc_mir_transform[596f0f775876c0b2]::const_prop_lint::ConstPropMachine>>::eval_rvalue_into_place
  65:     0x7f872b509e63 - <rustc_mir_transform[596f0f775876c0b2]::const_prop_lint::ConstPropagator as rustc_middle[95761bf3c41b10cf]::mir::visit::Visitor>::visit_statement
  66:     0x7f872b5098ff - <rustc_mir_transform[596f0f775876c0b2]::const_prop_lint::ConstPropagator as rustc_middle[95761bf3c41b10cf]::mir::visit::Visitor>::visit_body
  67:     0x7f872b505ebd - <rustc_mir_transform[596f0f775876c0b2]::const_prop_lint::ConstProp as rustc_mir_transform[596f0f775876c0b2]::pass_manager::MirLint>::run_lint
  68:     0x7f872b45b9f2 - rustc_mir_transform[596f0f775876c0b2]::pass_manager::run_passes
  69:     0x7f872b54ce09 - rustc_mir_transform[596f0f775876c0b2]::run_post_borrowck_cleanup_passes
  70:     0x7f872b54c6d9 - rustc_mir_transform[596f0f775876c0b2]::mir_drops_elaborated_and_const_checked
  71:     0x7f872c5a8878 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::try_execute_query::<rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt, rustc_query_system[839b2ca9f9a29e23]::query::caches::DefaultCache<rustc_middle[95761bf3c41b10cf]::ty::WithOptConstParam<rustc_span[3e9cd59107943c18]::def_id::LocalDefId>, &rustc_data_structures[3cf013d82b52e206]::steal::Steal<rustc_middle[95761bf3c41b10cf]::mir::Body>>>
  72:     0x7f872c6eb9ea - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  73:     0x7f872ca3a556 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  74:     0x7f872afd41ae - <rustc_session[9ce44d69e2e9f546]::session::Session>::time::<(), rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#3}>
  75:     0x7f872afba0f2 - rustc_interface[cf714bd2a8aae84d]::passes::analysis
  76:     0x7f872c5f473e - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::try_execute_query::<rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt, rustc_query_system[839b2ca9f9a29e23]::query::caches::DefaultCache<(), core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>>
  77:     0x7f872c6f2c78 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::analysis, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  78:     0x7f872aea12d4 - <rustc_interface[cf714bd2a8aae84d]::passes::QueryContext>::enter::<rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  79:     0x7f872ae4f06e - <rustc_interface[cf714bd2a8aae84d]::interface::Compiler>::enter::<rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}::{closure#2}, core[f370780f52494a52]::result::Result<core[f370780f52494a52]::option::Option<rustc_interface[cf714bd2a8aae84d]::queries::Linker>, rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  80:     0x7f872ae31b9b - rustc_span[3e9cd59107943c18]::with_source_map::<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_interface[cf714bd2a8aae84d]::interface::create_compiler_and_run<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#1}>
  81:     0x7f872ae50209 - <scoped_tls[763d6a33bb6869db]::ScopedKey<rustc_span[3e9cd59107943c18]::SessionGlobals>>::set::<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  82:     0x7f872aea9ec9 - std[8b1df1c4c90d5389]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf714bd2a8aae84d]::util::run_in_thread_pool_with_globals<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  83:     0x7f872ae515c1 - std[8b1df1c4c90d5389]::panicking::try::<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, core[f370780f52494a52]::panic::unwind_safe::AssertUnwindSafe<<std[8b1df1c4c90d5389]::thread::Builder>::spawn_unchecked_<rustc_interface[cf714bd2a8aae84d]::util::run_in_thread_pool_with_globals<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  84:     0x7f872aea4e02 - <<std[8b1df1c4c90d5389]::thread::Builder>::spawn_unchecked_<rustc_interface[cf714bd2a8aae84d]::util::run_in_thread_pool_with_globals<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#1} as core[f370780f52494a52]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  85:     0x7f872a369593 - std::sys::unix::thread::Thread::new::thread_start::h38917ce570c7f83a
  86:     0x7f87248bc609 - start_thread
  87:     0x7f872a1cf163 - clone
  88:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (4d1d3cedd 2022-04-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [eval_to_const_value_raw] simplifying constant for the type system `UNION`
#1 [eval_to_const_value_raw] simplifying constant for the type system `UNION`
#2 [try_normalize_mir_const_after_erasing_regions] normalizing `UNION`
#3 [eval_to_allocation_raw] const-evaluating + checking `read_field1::FIELD1`
#4 [eval_to_const_value_raw] simplifying constant for the type system `read_field1::FIELD1`
#5 [eval_to_const_value_raw] simplifying constant for the type system `read_field1::FIELD1`
#6 [try_normalize_mir_const_after_erasing_regions] normalizing `read_field1::FIELD1`
#7 [mir_drops_elaborated_and_const_checked] elaborating drops for `read_field1`
#8 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/union-ice.rs stdout ----
---- [ui] src/test/ui/consts/const-eval/union-ice.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/union-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-ice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-ice/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: InterpErrorInfo(InterpErrorInfoInner { kind: using uninitialized data, but this operation requires initialized memory, backtrace: None })', compiler/rustc_const_eval/src/const_eval/valtrees.rs:79:39
stack backtrace:
   0:     0x7f43fb4989ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7a8d9e17c5c41d63
   1:     0x7f43fb4feff8 - core::fmt::write::he96e6c1da7790e01
   2:     0x7f43fb488811 - std::io::Write::write_fmt::he213735f9fc023bf
   3:     0x7f43fb49b9de - std::panicking::default_hook::{{closure}}::h1db1d872e7cc6dba
   4:     0x7f43fb49b60c - std::panicking::default_hook::h117fb00cf01e902c
   5:     0x7f43fbffcba1 - rustc_driver[f41c55b70dce3ba2]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f43fb49c23e - std::panicking::rust_panic_with_hook::hfb6238818b193c9a
   7:     0x7f43fb49c037 - std::panicking::begin_panic_handler::{{closure}}::h1c134276a38e45ce
   8:     0x7f43fb498ec4 - std::sys_common::backtrace::__rust_end_short_backtrace::h3511975eefed2e40
   9:     0x7f43fb49bd19 - rust_begin_unwind
  10:     0x7f43fb450073 - core::panicking::panic_fmt::hb74e78879777d5d2
  11:     0x7f43fb450273 - core::result::unwrap_failed::h943c676139e14f19
  12:     0x7f43fd57e19d - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::const_to_valtree_inner
  13:     0x7f43fd500d58 - <core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::try_fold::<(), <core[f370780f52494a52]::iter::adapters::GenericShunt<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::try_fold<(), core[f370780f52494a52]::iter::traits::iterator::Iterator::try_for_each::call<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>::Break>::{closure#0}, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>::{closure#0}, core[f370780f52494a52]::ops::control_flow::ControlFlow<core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>>
  14:     0x7f43fd56902b - <core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::try_fold::<(), <core[f370780f52494a52]::iter::adapters::GenericShunt<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::try_fold<(), core[f370780f52494a52]::iter::traits::iterator::Iterator::try_for_each::call<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>::Break>::{closure#0}, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>::{closure#0}, core[f370780f52494a52]::ops::control_flow::ControlFlow<core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>>
  15:     0x7f43fd50545b - <core[f370780f52494a52]::iter::adapters::GenericShunt<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::next
  16:     0x7f43fd5ec4ae - <alloc[57f2ea052be60ee9]::vec::Vec<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree> as alloc[57f2ea052be60ee9]::vec::spec_from_iter::SpecFromIter<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree, core[f370780f52494a52]::iter::adapters::GenericShunt<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>>>>::from_iter
  17:     0x7f43fd4fdb49 - core[f370780f52494a52]::iter::adapters::try_process::<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>, <core[f370780f52494a52]::option::Option<alloc[57f2ea052be60ee9]::vec::Vec<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>> as core[f370780f52494a52]::iter::traits::collect::FromIterator<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>>::from_iter<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>>::{closure#0}, alloc[57f2ea052be60ee9]::vec::Vec<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>
  18:     0x7f43fd57c964 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches
  19:     0x7f43fd57d9a2 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::const_to_valtree_inner
  20:     0x7f43fd57c269 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::const_to_valtree
  21:     0x7f43fd573630 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_const_value_raw_provider
  22:     0x7f43fd8018e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_const_value_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  23:     0x7f43fdb78d45 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_const_value_raw
  24:     0x7f43fd572dc0 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_const_value_raw_provider
  25:     0x7f43fd8018e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_const_value_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  26:     0x7f43fdb78d45 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_const_value_raw
  27:     0x7f43fe806e2d - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::const_eval_global_id
  28:     0x7f43fe806793 - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::const_eval_poly
  29:     0x7f43fe384f97 - <rustc_lint[33f320154ce7d8a2]::builtin::UnusedBrokenConst as rustc_lint[33f320154ce7d8a2]::passes::LateLintPass>::check_item
  30:     0x7f43fe347144 - <rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass as rustc_lint[33f320154ce7d8a2]::passes::LateLintPass>::check_item
  31:     0x7f43fc171d8d - <rustc_lint[33f320154ce7d8a2]::late::LateContextAndPass<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass> as rustc_hir[deb440e854090368]::intravisit::Visitor>::visit_nested_item
  32:     0x7f43fc17a4cc - rustc_hir[deb440e854090368]::intravisit::walk_mod::<rustc_lint[33f320154ce7d8a2]::late::LateContextAndPass<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass>>
  33:     0x7f43fc16e7bc - <rustc_lint[33f320154ce7d8a2]::late::LateContextAndPass<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass> as rustc_hir[deb440e854090368]::intravisit::Visitor>::visit_mod
  34:     0x7f43fc164998 - rustc_lint[33f320154ce7d8a2]::late::late_lint_crate::<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass>
  35:     0x7f43fc11099b - <rustc_session[9ce44d69e2e9f546]::session::Session>::time::<(), rustc_lint[33f320154ce7d8a2]::late::check_crate<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass, rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}::{closure#0}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}::{closure#0}>
  36:     0x7f43fc0f9982 - rustc_data_structures[3cf013d82b52e206]::sync::join::<rustc_lint[33f320154ce7d8a2]::late::check_crate<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass, rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}::{closure#0}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}, rustc_lint[33f320154ce7d8a2]::late::check_crate<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass, rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}::{closure#0}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}, (), ()>
  37:     0x7f43fc110af9 - <rustc_session[9ce44d69e2e9f546]::session::Session>::time::<(), rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}::{closure#0}::{closure#2}::{closure#0}>
  38:     0x7f43fc115305 - std[8b1df1c4c90d5389]::panicking::try::<(), core[f370780f52494a52]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}::{closure#0}::{closure#2}>>
  39:     0x7f43fc0ca1bb - <core[f370780f52494a52]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}::{closure#0}> as core[f370780f52494a52]::ops::function::FnOnce<()>>::call_once
  40:     0x7f43fc112731 - <rustc_session[9ce44d69e2e9f546]::session::Session>::time::<(), rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}>
  41:     0x7f43fc0f813c - rustc_interface[cf714bd2a8aae84d]::passes::analysis
  42:     0x7f43fd73273e - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::try_execute_query::<rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt, rustc_query_system[839b2ca9f9a29e23]::query::caches::DefaultCache<(), core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>>
  43:     0x7f43fd830c78 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::analysis, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  44:     0x7f43fbfdf2d4 - <rustc_interface[cf714bd2a8aae84d]::passes::QueryContext>::enter::<rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  45:     0x7f43fbf8d06e - <rustc_interface[cf714bd2a8aae84d]::interface::Compiler>::enter::<rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}::{closure#2}, core[f370780f52494a52]::result::Result<core[f370780f52494a52]::option::Option<rustc_interface[cf714bd2a8aae84d]::queries::Linker>, rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  46:     0x7f43fbf6fb9b - rustc_span[3e9cd59107943c18]::with_source_map::<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_interface[cf714bd2a8aae84d]::interface::create_compiler_and_run<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#1}>
  47:     0x7f43fbf8e209 - <scoped_tls[763d6a33bb6869db]::ScopedKey<rustc_span[3e9cd59107943c18]::SessionGlobals>>::set::<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  48:     0x7f43fbfe7ec9 - std[8b1df1c4c90d5389]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf714bd2a8aae84d]::util::run_in_thread_pool_with_globals<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  49:     0x7f43fbf8f5c1 - std[8b1df1c4c90d5389]::panicking::try::<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, core[f370780f52494a52]::panic::unwind_safe::AssertUnwindSafe<<std[8b1df1c4c90d5389]::thread::Builder>::spawn_unchecked_<rustc_interface[cf714bd2a8aae84d]::util::run_in_thread_pool_with_globals<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  50:     0x7f43fbfe2e02 - <<std[8b1df1c4c90d5389]::thread::Builder>::spawn_unchecked_<rustc_interface[cf714bd2a8aae84d]::util::run_in_thread_pool_with_globals<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#1} as core[f370780f52494a52]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  51:     0x7f43fb4a7593 - std::sys::unix::thread::Thread::new::thread_start::h38917ce570c7f83a
  52:     0x7f43f59fa609 - start_thread
  53:     0x7f43fb30d163 - clone
  54:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (4d1d3cedd 2022-04-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [eval_to_const_value_raw] simplifying constant for the type system `UNION`
#1 [eval_to_const_value_raw] simplifying constant for the type system `UNION`
#2 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/ub-wide-ptr.rs stdout ----
---- [ui] src/test/ui/consts/const-eval/ub-wide-ptr.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const STR_TOO_LONG: &str = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc10───────╼ e7 03 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:40:1
   |
   |
LL | const NESTED_STR_MUCH_TOO_LONG: (&str,) = (unsafe { mem::transmute((&42, usize::MAX)) },);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc16───────╼ ff ff ff ff ff ff ff ff │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:43:1
   |
   |
LL | const STR_LENGTH_PTR: &str = unsafe { mem::transmute((&42u8, &3)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc23───────╼ ╾───────alloc25───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:46:1
   |
   |
LL | const MY_STR_LENGTH_PTR: &MyStr = unsafe { mem::transmute((&42u8, &3)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc31───────╼ ╾───────alloc33───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:48:1
   |
   |
LL | const MY_STR_MUCH_TOO_LONG: &MyStr = unsafe { mem::transmute((&42u8, usize::MAX)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc38───────╼ ff ff ff ff ff ff ff ff │ ╾──────╼........


thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: InterpErrorInfo(InterpErrorInfoInner { kind: using uninitialized data, but this operation requires initialized memory, backtrace: None })', compiler/rustc_const_eval/src/const_eval/valtrees.rs:79:39
   0:     0x7fe2d35519ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7a8d9e17c5c41d63
   1:     0x7fe2d35b7ff8 - core::fmt::write::he96e6c1da7790e01
   2:     0x7fe2d3541811 - std::io::Write::write_fmt::he213735f9fc023bf
   2:     0x7fe2d3541811 - std::io::Write::write_fmt::he213735f9fc023bf
   3:     0x7fe2d35549de - std::panicking::default_hook::{{closure}}::h1db1d872e7cc6dba
   4:     0x7fe2d355460c - std::panicking::default_hook::h117fb00cf01e902c
   5:     0x7fe2d40b5ba1 - rustc_driver[f41c55b70dce3ba2]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fe2d355523e - std::panicking::rust_panic_with_hook::hfb6238818b193c9a
   7:     0x7fe2d3555037 - std::panicking::begin_panic_handler::{{closure}}::h1c134276a38e45ce
   8:     0x7fe2d3551ec4 - std::sys_common::backtrace::__rust_end_short_backtrace::h3511975eefed2e40
  10:     0x7fe2d3509073 - core::panicking::panic_fmt::hb74e78879777d5d2
  11:     0x7fe2d3509273 - core::result::unwrap_failed::h943c676139e14f19
  11:     0x7fe2d3509273 - core::result::unwrap_failed::h943c676139e14f19
  12:     0x7fe2d563719d - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::const_to_valtree_inner
  13:     0x7fe2d55b9d58 - <core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::try_fold::<(), <core[f370780f52494a52]::iter::adapters::GenericShunt<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::try_fold<(), core[f370780f52494a52]::iter::traits::iterator::Iterator::try_for_each::call<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>::Break>::{closure#0}, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>::{closure#0}, core[f370780f52494a52]::ops::control_flow::ControlFlow<core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>>
  14:     0x7fe2d562202b - <core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::try_fold::<(), <core[f370780f52494a52]::iter::adapters::GenericShunt<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::try_fold<(), core[f370780f52494a52]::iter::traits::iterator::Iterator::try_for_each::call<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>::Break>::{closure#0}, core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>::{closure#0}, core[f370780f52494a52]::ops::control_flow::ControlFlow<core[f370780f52494a52]::ops::control_flow::ControlFlow<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>>
  15:     0x7fe2d55be45b - <core[f370780f52494a52]::iter::adapters::GenericShunt<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::next
  16:     0x7fe2d56a54ae - <alloc[57f2ea052be60ee9]::vec::Vec<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree> as alloc[57f2ea052be60ee9]::vec::spec_from_iter::SpecFromIter<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree, core[f370780f52494a52]::iter::adapters::GenericShunt<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>>>>::from_iter
  17:     0x7fe2d55b6b49 - core[f370780f52494a52]::iter::adapters::try_process::<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>, rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>, <core[f370780f52494a52]::option::Option<alloc[57f2ea052be60ee9]::vec::Vec<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>> as core[f370780f52494a52]::iter::traits::collect::FromIterator<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>>::from_iter<core[f370780f52494a52]::iter::adapters::chain::Chain<core[f370780f52494a52]::option::IntoIter<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>, core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<usize>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches::{closure#1}>>>::{closure#0}, alloc[57f2ea052be60ee9]::vec::Vec<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>
  18:     0x7fe2d5635964 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::branches
  19:     0x7fe2d56369a2 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::const_to_valtree_inner
  20:     0x7fe2d55bd8c5 - <core[f370780f52494a52]::iter::adapters::GenericShunt<core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<u64>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::slice_branches::{closure#0}>, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>> as core[f370780f52494a52]::iter::traits::iterator::Iterator>::next
  21:     0x7fe2d56a5119 - <alloc[57f2ea052be60ee9]::vec::Vec<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree> as alloc[57f2ea052be60ee9]::vec::spec_from_iter::SpecFromIter<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree, core[f370780f52494a52]::iter::adapters::GenericShunt<core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<u64>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::slice_branches::{closure#0}>, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>>>>::from_iter
  22:     0x7fe2d55b6947 - core[f370780f52494a52]::iter::adapters::try_process::<core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<u64>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::slice_branches::{closure#0}>, rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree, core[f370780f52494a52]::option::Option<core[f370780f52494a52]::convert::Infallible>, <core[f370780f52494a52]::option::Option<alloc[57f2ea052be60ee9]::vec::Vec<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>> as core[f370780f52494a52]::iter::traits::collect::FromIterator<core[f370780f52494a52]::option::Option<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>>::from_iter<core[f370780f52494a52]::iter::adapters::map::Map<core[f370780f52494a52]::ops::range::Range<u64>, rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::slice_branches::{closure#0}>>::{closure#0}, alloc[57f2ea052be60ee9]::vec::Vec<rustc_middle[95761bf3c41b10cf]::ty::consts::valtree::ValTree>>
  23:     0x7fe2d5636662 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::const_to_valtree_inner
  24:     0x7fe2d56368d9 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::const_to_valtree_inner
  25:     0x7fe2d5635269 - rustc_const_eval[3bdf8a288a416b24]::const_eval::valtrees::const_to_valtree
  26:     0x7fe2d562c630 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_const_value_raw_provider
  27:     0x7fe2d58ba8e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_const_value_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  28:     0x7fe2d5c31d45 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_const_value_raw
  29:     0x7fe2d562bdc0 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_const_value_raw_provider
  30:     0x7fe2d58ba8e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_const_value_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  31:     0x7fe2d5c31d45 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_const_value_raw
  32:     0x7fe2d68bfe2d - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::const_eval_global_id
  33:     0x7fe2d68e798a - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::const_eval_resolve
  34:     0x7fe2d652d51e - <rustc_trait_selection[561e35c8266e314f]::traits::query::normalize::QueryNormalizer as rustc_middle[95761bf3c41b10cf]::ty::fold::FallibleTypeFolder>::try_fold_const
  35:     0x7fe2d652d856 - <rustc_trait_selection[561e35c8266e314f]::traits::query::normalize::QueryNormalizer as rustc_middle[95761bf3c41b10cf]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  36:     0x7fe2d53c2a5f - <rustc_infer[6814aa6b2eed40e]::infer::at::At as rustc_trait_selection[561e35c8266e314f]::traits::query::normalize::AtExt>::normalize::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  37:     0x7fe2d53d0a71 - <rustc_infer[6814aa6b2eed40e]::infer::InferCtxtBuilder>::enter::<core[f370780f52494a52]::result::Result<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind, rustc_middle[95761bf3c41b10cf]::traits::query::NoSolution>, rustc_traits[7167181ae6d12e6e]::normalize_erasing_regions::try_normalize_after_erasing_regions<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>::{closure#0}>
  38:     0x7fe2d54d18fe - <rustc_traits[7167181ae6d12e6e]::normalize_erasing_regions::provide::{closure#1} as core[f370780f52494a52]::ops::function::FnOnce<(rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt, rustc_middle[95761bf3c41b10cf]::ty::ParamEnvAnd<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>)>>::call_once
  39:     0x7fe2d58e4ec0 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::try_normalize_mir_const_after_erasing_regions, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  40:     0x7fe2d5c32cee - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::try_normalize_mir_const_after_erasing_regions
  41:     0x7fe2d6a60314 - <rustc_middle[95761bf3c41b10cf]::ty::normalize_erasing_regions::TryNormalizeAfterErasingRegionsFolder as rustc_middle[95761bf3c41b10cf]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  42:     0x7fe2d56b1703 - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::try_normalize_erasing_regions::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  43:     0x7fe2d56b2b51 - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::try_subst_and_normalize_erasing_regions::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  44:     0x7fe2d5556273 - <rustc_const_eval[3bdf8a288a416b24]::interpret::eval_context::InterpCx<rustc_const_eval[3bdf8a288a416b24]::const_eval::machine::CompileTimeInterpreter>>::subst_from_current_frame_and_normalize_erasing_regions::<rustc_middle[95761bf3c41b10cf]::mir::ConstantKind>
  45:     0x7fe2d556b70f - <rustc_const_eval[3bdf8a288a416b24]::interpret::eval_context::InterpCx<rustc_const_eval[3bdf8a288a416b24]::const_eval::machine::CompileTimeInterpreter>>::eval_operand
  46:     0x7fe2d557816d - <rustc_const_eval[3bdf8a288a416b24]::interpret::eval_context::InterpCx<rustc_const_eval[3bdf8a288a416b24]::const_eval::machine::CompileTimeInterpreter>>::eval_rvalue_into_place
  47:     0x7fe2d5574672 - <rustc_const_eval[3bdf8a288a416b24]::interpret::eval_context::InterpCx<rustc_const_eval[3bdf8a288a416b24]::const_eval::machine::CompileTimeInterpreter>>::run
  48:     0x7fe2d562e4f3 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_allocation_raw_provider
  49:     0x7fe2d58b08be - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_allocation_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  50:     0x7fe2d5c31d05 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_allocation_raw
  51:     0x7fe2d562c151 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_const_value_raw_provider
  52:     0x7fe2d58ba8e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_const_value_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  53:     0x7fe2d5c31d45 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_const_value_raw
  54:     0x7fe2d562bdc0 - rustc_const_eval[3bdf8a288a416b24]::const_eval::eval_queries::eval_to_const_value_raw_provider
  55:     0x7fe2d58ba8e5 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::eval_to_const_value_raw, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  56:     0x7fe2d5c31d45 - <rustc_query_impl[191c58d5d1bdd6d0]::Queries as rustc_middle[95761bf3c41b10cf]::ty::query::QueryEngine>::eval_to_const_value_raw
  57:     0x7fe2d68bfe2d - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::const_eval_global_id
  58:     0x7fe2d68bf793 - <rustc_middle[95761bf3c41b10cf]::ty::context::TyCtxt>::const_eval_poly
  59:     0x7fe2d643df97 - <rustc_lint[33f320154ce7d8a2]::builtin::UnusedBrokenConst as rustc_lint[33f320154ce7d8a2]::passes::LateLintPass>::check_item
  60:     0x7fe2d6400144 - <rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass as rustc_lint[33f320154ce7d8a2]::passes::LateLintPass>::check_item
  61:     0x7fe2d422ad8d - <rustc_lint[33f320154ce7d8a2]::late::LateContextAndPass<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass> as rustc_hir[deb440e854090368]::intravisit::Visitor>::visit_nested_item
  62:     0x7fe2d42334cc - rustc_hir[deb440e854090368]::intravisit::walk_mod::<rustc_lint[33f320154ce7d8a2]::late::LateContextAndPass<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass>>
  63:     0x7fe2d42277bc - <rustc_lint[33f320154ce7d8a2]::late::LateContextAndPass<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass> as rustc_hir[deb440e854090368]::intravisit::Visitor>::visit_mod
  64:     0x7fe2d421d998 - rustc_lint[33f320154ce7d8a2]::late::late_lint_crate::<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass>
  65:     0x7fe2d41c999b - <rustc_session[9ce44d69e2e9f546]::session::Session>::time::<(), rustc_lint[33f320154ce7d8a2]::late::check_crate<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass, rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}::{closure#0}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}::{closure#0}>
  66:     0x7fe2d41b2982 - rustc_data_structures[3cf013d82b52e206]::sync::join::<rustc_lint[33f320154ce7d8a2]::late::check_crate<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass, rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}::{closure#0}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}, rustc_lint[33f320154ce7d8a2]::late::check_crate<rustc_lint[33f320154ce7d8a2]::BuiltinCombinedLateLintPass, rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}::{closure#0}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}, (), ()>
  67:     0x7fe2d41c9af9 - <rustc_session[9ce44d69e2e9f546]::session::Session>::time::<(), rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}::{closure#0}::{closure#2}::{closure#0}>
  68:     0x7fe2d41ce305 - std[8b1df1c4c90d5389]::panicking::try::<(), core[f370780f52494a52]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}::{closure#0}::{closure#2}>>
  69:     0x7fe2d41831bb - <core[f370780f52494a52]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}::{closure#0}> as core[f370780f52494a52]::ops::function::FnOnce<()>>::call_once
  70:     0x7fe2d41cb731 - <rustc_session[9ce44d69e2e9f546]::session::Session>::time::<(), rustc_interface[cf714bd2a8aae84d]::passes::analysis::{closure#5}>
  71:     0x7fe2d41b113c - rustc_interface[cf714bd2a8aae84d]::passes::analysis
  72:     0x7fe2d57eb73e - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::try_execute_query::<rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt, rustc_query_system[839b2ca9f9a29e23]::query::caches::DefaultCache<(), core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>>
  73:     0x7fe2d58e9c78 - rustc_query_system[839b2ca9f9a29e23]::query::plumbing::get_query::<rustc_query_impl[191c58d5d1bdd6d0]::queries::analysis, rustc_query_impl[191c58d5d1bdd6d0]::plumbing::QueryCtxt>
  74:     0x7fe2d40982d4 - <rustc_interface[cf714bd2a8aae84d]::passes::QueryContext>::enter::<rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  75:     0x7fe2d404606e - <rustc_interface[cf714bd2a8aae84d]::interface::Compiler>::enter::<rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}::{closure#2}, core[f370780f52494a52]::result::Result<core[f370780f52494a52]::option::Option<rustc_interface[cf714bd2a8aae84d]::queries::Linker>, rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  76:     0x7fe2d4028b9b - rustc_span[3e9cd59107943c18]::with_source_map::<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_interface[cf714bd2a8aae84d]::interface::create_compiler_and_run<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#1}>
  77:     0x7fe2d4047209 - <scoped_tls[763d6a33bb6869db]::ScopedKey<rustc_span[3e9cd59107943c18]::SessionGlobals>>::set::<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  78:     0x7fe2d40a0ec9 - std[8b1df1c4c90d5389]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf714bd2a8aae84d]::util::run_in_thread_pool_with_globals<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>
  79:     0x7fe2d40485c1 - std[8b1df1c4c90d5389]::panicking::try::<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, core[f370780f52494a52]::panic::unwind_safe::AssertUnwindSafe<<std[8b1df1c4c90d5389]::thread::Builder>::spawn_unchecked_<rustc_interface[cf714bd2a8aae84d]::util::run_in_thread_pool_with_globals<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  80:     0x7fe2d409be02 - <<std[8b1df1c4c90d5389]::thread::Builder>::spawn_unchecked_<rustc_interface[cf714bd2a8aae84d]::util::run_in_thread_pool_with_globals<rustc_interface[cf714bd2a8aae84d]::interface::run_compiler<core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>, rustc_driver[f41c55b70dce3ba2]::run_compiler::{closure#1}>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#0}, core[f370780f52494a52]::result::Result<(), rustc_errors[b2d4e9a5f061dc95]::ErrorGuaranteed>>::{closure#1} as core[f370780f52494a52]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  81:     0x7fe2d3560593 - std::sys::unix::thread::Thread::new::thread_start::h38917ce570c7f83a
  82:     0x7fe2cdab3609 - start_thread
  83:     0x7fe2d33c6163 - clone
  84:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (4d1d3cedd 2022-04-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [eval_to_const_value_raw] simplifying constant for the type system `STR_NO_INIT::promoted[0]`
#1 [eval_to_const_value_raw] simplifying constant for the type system `STR_NO_INIT::promoted[0]`
#2 [try_normalize_mir_const_after_erasing_regions] normalizing `STR_NO_INIT::promoted[0]`
#3 [eval_to_allocation_raw] const-evaluating + checking `STR_NO_INIT`
#4 [eval_to_const_value_raw] simplifying constant for the type system `STR_NO_INIT`
#5 [eval_to_const_value_raw] simplifying constant for the type system `STR_NO_INIT`
#6 [analysis] running analysis passes on this crate
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/union-ub.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/union-ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-ub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-ub/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const BAD_BOOL: bool = unsafe { DummyUnion { u8: 42 }.bool};
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x2a, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
               2a                                              │ *

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/union-ub.rs:35:1
