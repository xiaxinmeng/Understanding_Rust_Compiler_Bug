plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: InterpErrorInfo(InterpErrorInfoInner { kind: memory access failed: alloc1410 has size 0, so pointer to 1 byte starting at offset 0 is out-of-bounds, backtrace: None })', compiler/rustc_const_eval/src/const_eval/valtrees.rs:266:14
   0:     0x7f2290b0ec32 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha35fb4208844cb61
   0:     0x7f2290b0ec32 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha35fb4208844cb61
   1:     0x7f2290b75ff8 - core::fmt::write::h42234c3e51154f4c
   2:     0x7f2290aff171 - std::io::Write::write_fmt::hf3faa85fa7d28190
   3:     0x7f2290b11f76 - std::panicking::default_hook::{{closure}}::h243e0a014f6b15da
   4:     0x7f2290b11b6d - std::panicking::default_hook::hdf681f01978f1e20
   5:     0x7f229162f5a1 - rustc_driver[610de2f075f3812e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2290b12910 - std::panicking::rust_panic_with_hook::h1c127668bc0f49d8
   7:     0x7f2290b12727 - std::panicking::begin_panic_handler::{{closure}}::hdc297c549f81c3b7
   8:     0x7f2290b0f1d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h7b90b067d1e7c19a
   9:     0x7f2290b12419 - rust_begin_unwind
  10:     0x7f2290ac60b3 - core::panicking::panic_fmt::h0eedb34d228802aa
  11:     0x7f2290ac6263 - core::result::unwrap_failed::hf38d50bcb33515da
  12:     0x7f2292bd6dfa - rustc_const_eval[da0e58d07d0e471]::const_eval::valtrees::fill_place_recursively
  13:     0x7f2292bd4ccb - rustc_const_eval[da0e58d07d0e471]::const_eval::valtrees::fill_place_recursively
  14:     0x7f2292bd51b9 - rustc_const_eval[da0e58d07d0e471]::const_eval::valtrees::fill_place_recursively
  15:     0x7f2292bd4ccb - rustc_const_eval[da0e58d07d0e471]::const_eval::valtrees::fill_place_recursively
  16:     0x7f2292bd264f - rustc_const_eval[da0e58d07d0e471]::const_eval::valtrees::valtree_to_const_value
  17:     0x7f2292b0deab - <rustc_const_eval[da0e58d07d0e471]::provide::{closure#2} as core[10878fb91fc84a80]::ops::function::FnOnce<(rustc_middle[87143a0b6987fc74]::ty::context::TyCtxt, (rustc_middle[87143a0b6987fc74]::ty::Ty, rustc_middle[87143a0b6987fc74]::ty::consts::valtree::ValTree))>>::call_once
  18:     0x7f2292de0718 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::valtree_to_const_val, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  19:     0x7f229314d3db - <rustc_query_impl[62d43cdc40496630]::Queries as rustc_middle[87143a0b6987fc74]::ty::query::QueryEngine>::valtree_to_const_val
  20:     0x7f2292b80ff7 - rustc_const_eval[da0e58d07d0e471]::const_eval::eval_queries::eval_to_const_value_raw_provider
  21:     0x7f2292df4f86 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::eval_to_const_value_raw, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  22:     0x7f229314d345 - <rustc_query_impl[62d43cdc40496630]::Queries as rustc_middle[87143a0b6987fc74]::ty::query::QueryEngine>::eval_to_const_value_raw
  23:     0x7f2292b80272 - rustc_const_eval[da0e58d07d0e471]::const_eval::eval_queries::eval_to_const_value_raw_provider
  24:     0x7f2292df4f86 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::eval_to_const_value_raw, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  25:     0x7f229314d345 - <rustc_query_impl[62d43cdc40496630]::Queries as rustc_middle[87143a0b6987fc74]::ty::query::QueryEngine>::eval_to_const_value_raw
  26:     0x7f2293d91c8e - <rustc_middle[87143a0b6987fc74]::ty::context::TyCtxt>::const_eval_global_id
  27:     0x7f2293db8f0a - <rustc_middle[87143a0b6987fc74]::ty::context::TyCtxt>::const_eval_resolve
  28:     0x7f2293a1f309 - <rustc_trait_selection[ee9dd18565913941]::traits::query::normalize::QueryNormalizer as rustc_middle[87143a0b6987fc74]::ty::fold::FallibleTypeFolder>::try_fold_const
  29:     0x7f2293a1f4d7 - <rustc_trait_selection[ee9dd18565913941]::traits::query::normalize::QueryNormalizer as rustc_middle[87143a0b6987fc74]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  30:     0x7f2292921fd2 - <rustc_infer[e9909202d8948eff]::infer::at::At as rustc_trait_selection[ee9dd18565913941]::traits::query::normalize::AtExt>::normalize::<rustc_middle[87143a0b6987fc74]::mir::ConstantKind>
  31:     0x7f229292fc4c - <rustc_infer[e9909202d8948eff]::infer::InferCtxtBuilder>::enter::<core[10878fb91fc84a80]::result::Result<rustc_middle[87143a0b6987fc74]::mir::ConstantKind, rustc_middle[87143a0b6987fc74]::traits::query::NoSolution>, rustc_traits[cff765eab9023830]::normalize_erasing_regions::try_normalize_after_erasing_regions<rustc_middle[87143a0b6987fc74]::mir::ConstantKind>::{closure#0}>
  32:     0x7f2292a1430e - <rustc_traits[cff765eab9023830]::normalize_erasing_regions::provide::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<(rustc_middle[87143a0b6987fc74]::ty::context::TyCtxt, rustc_middle[87143a0b6987fc74]::ty::ParamEnvAnd<rustc_middle[87143a0b6987fc74]::mir::ConstantKind>)>>::call_once
  33:     0x7f2292e1d071 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::try_normalize_mir_const_after_erasing_regions, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  34:     0x7f229314e266 - <rustc_query_impl[62d43cdc40496630]::Queries as rustc_middle[87143a0b6987fc74]::ty::query::QueryEngine>::try_normalize_mir_const_after_erasing_regions
  35:     0x7f2293f202cf - <rustc_middle[87143a0b6987fc74]::ty::normalize_erasing_regions::TryNormalizeAfterErasingRegionsFolder as rustc_middle[87143a0b6987fc74]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  36:     0x7f2291d235b7 - <rustc_middle[87143a0b6987fc74]::ty::context::TyCtxt>::try_normalize_erasing_regions::<rustc_middle[87143a0b6987fc74]::mir::ConstantKind>
  37:     0x7f2291d25031 - <rustc_middle[87143a0b6987fc74]::ty::context::TyCtxt>::try_subst_and_normalize_erasing_regions::<rustc_middle[87143a0b6987fc74]::mir::ConstantKind>
  38:     0x7f2291af9ad7 - <rustc_const_eval[da0e58d07d0e471]::interpret::eval_context::InterpCx<rustc_mir_transform[4aaf9bfc3db13ff7]::const_prop::ConstPropMachine>>::subst_from_current_frame_and_normalize_erasing_regions::<rustc_middle[87143a0b6987fc74]::mir::ConstantKind>
  39:     0x7f2291b44d93 - <rustc_const_eval[da0e58d07d0e471]::interpret::eval_context::InterpCx<rustc_mir_transform[4aaf9bfc3db13ff7]::const_prop_lint::ConstPropMachine>>::eval_operand
  40:     0x7f2291b1976e - <rustc_const_eval[da0e58d07d0e471]::interpret::eval_context::InterpCx<rustc_mir_transform[4aaf9bfc3db13ff7]::const_prop_lint::ConstPropMachine>>::eval_rvalue_into_place
  41:     0x7f2291c778d3 - <rustc_mir_transform[4aaf9bfc3db13ff7]::const_prop_lint::ConstPropagator as rustc_middle[87143a0b6987fc74]::mir::visit::Visitor>::visit_statement
  42:     0x7f2291c7736f - <rustc_mir_transform[4aaf9bfc3db13ff7]::const_prop_lint::ConstPropagator as rustc_middle[87143a0b6987fc74]::mir::visit::Visitor>::visit_body
  43:     0x7f2291c73cbd - <rustc_mir_transform[4aaf9bfc3db13ff7]::const_prop_lint::ConstProp as rustc_mir_transform[4aaf9bfc3db13ff7]::pass_manager::MirLint>::run_lint
  44:     0x7f2291d18495 - rustc_mir_transform[4aaf9bfc3db13ff7]::pass_manager::run_passes
  45:     0x7f2291cba2d5 - rustc_mir_transform[4aaf9bfc3db13ff7]::run_post_borrowck_cleanup_passes
  46:     0x7f2291cb9e6d - rustc_mir_transform[4aaf9bfc3db13ff7]::mir_drops_elaborated_and_const_checked
  47:     0x7f2292ce353c - rustc_query_system[6655655853c83e28]::query::plumbing::try_execute_query::<rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt, rustc_query_system[6655655853c83e28]::query::caches::DefaultCache<rustc_middle[87143a0b6987fc74]::ty::WithOptConstParam<rustc_span[e033c2886c1ea87]::def_id::LocalDefId>, &rustc_data_structures[e873bb7798a1662c]::steal::Steal<rustc_middle[87143a0b6987fc74]::mir::Body>>>
  48:     0x7f2292e1adab - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  49:     0x7f229314cb56 - <rustc_query_impl[62d43cdc40496630]::Queries as rustc_middle[87143a0b6987fc74]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  50:     0x7f229173f942 - <rustc_session[490bd8b11d3080dc]::session::Session>::time::<(), rustc_interface[8c85a1b7802599b3]::passes::analysis::{closure#3}>
  51:     0x7f229173a852 - rustc_interface[8c85a1b7802599b3]::passes::analysis
  52:     0x7f2292d2d75c - rustc_query_system[6655655853c83e28]::query::plumbing::try_execute_query::<rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt, rustc_query_system[6655655853c83e28]::query::caches::DefaultCache<(), core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>>
  53:     0x7f2292e21b89 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::analysis, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  54:     0x7f2291620fea - <rustc_interface[8c85a1b7802599b3]::passes::QueryContext>::enter::<rustc_driver[610de2f075f3812e]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  55:     0x7f22915c47a6 - <rustc_interface[8c85a1b7802599b3]::interface::Compiler>::enter::<rustc_driver[610de2f075f3812e]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[8c85a1b7802599b3]::queries::Linker>, rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  56:     0x7f22915a62b6 - rustc_span[e033c2886c1ea87]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_interface[8c85a1b7802599b3]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[610de2f075f3812e]::run_compiler::{closure#1}>::{closure#1}>
  57:     0x7f22915d7897 - rustc_interface[8c85a1b7802599b3]::interface::create_compiler_and_run::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[610de2f075f3812e]::run_compiler::{closure#1}>
  58:     0x7f22915daff2 - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[e033c2886c1ea87]::SessionGlobals>>::set::<rustc_interface[8c85a1b7802599b3]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[610de2f075f3812e]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  59:     0x7f2291622f69 - std[38ff3720b7fd637]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8c85a1b7802599b3]::util::run_in_thread_pool_with_globals<rustc_interface[8c85a1b7802599b3]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[610de2f075f3812e]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  60:     0x7f22915dd39e - std[38ff3720b7fd637]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[38ff3720b7fd637]::thread::Builder>::spawn_unchecked_<rustc_interface[8c85a1b7802599b3]::util::run_in_thread_pool_with_globals<rustc_interface[8c85a1b7802599b3]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[610de2f075f3812e]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  61:     0x7f229161e0c0 - <<std[38ff3720b7fd637]::thread::Builder>::spawn_unchecked_<rustc_interface[8c85a1b7802599b3]::util::run_in_thread_pool_with_globals<rustc_interface[8c85a1b7802599b3]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[610de2f075f3812e]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  62:     0x7f2290b1f313 - std::sys::unix::thread::Thread::new::thread_start::h38902d511e7013ce
  63:     0x7f228b070609 - start_thread
  64:     0x7f2290983163 - clone
  65:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (1845c607b 2022-04-21) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [valtree_to_const_val] convert type-level constant value to mir constant value
#1 [eval_to_const_value_raw] simplifying constant for the type system `char::convert::from_digit::promoted[1]`
#2 [eval_to_const_value_raw] simplifying constant for the type system `char::convert::from_digit::promoted[1]`
#3 [try_normalize_mir_const_after_erasing_regions] normalizing `char::convert::from_digit::promoted[1]`
#4 [mir_drops_elaborated_and_const_checked] elaborating drops for `char::convert::from_digit`
#5 [analysis] running analysis passes on this crate
error: could not compile `core`
Build completed unsuccessfully in 0:03:22
