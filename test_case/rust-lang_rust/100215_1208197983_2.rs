text
error: internal compiler error: compiler/rustc_middle/src/ty/subst.rs:554:13: Region parameter out of range when substituting in region 'b (index=5)

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/affe0d3a00e92fa7885e3f5d2c5073fde432d154/compiler/rustc_errors/src/lib.rs:1392:9
stack backtrace:
   0:     0x7fdf48e529f0 - std::backtrace_rs::backtrace::libunwind::trace::h74f2a0cc6b6b0bc5
                               at /rustc/affe0d3a00e92fa7885e3f5d2c5073fde432d154/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7fdf48e529f0 - std::backtrace_rs::backtrace::trace_unsynchronized::h06f3d1adbfc08eec
                               at /rustc/affe0d3a00e92fa7885e3f5d2c5073fde432d154/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fdf48e529f0 - std::sys_common::backtrace::_print_fmt::he5f15b7310953827
                               at /rustc/affe0d3a00e92fa7885e3f5d2c5073fde432d154/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7fdf48e529f0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0481eccec82d418c
                               at /rustc/affe0d3a00e92fa7885e3f5d2c5073fde432d154/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7fdf48eae3bc - core::fmt::write::h609cae14de3da602
                               at /rustc/affe0d3a00e92fa7885e3f5d2c5073fde432d154/library/core/src/fmt/mod.rs:1198:17
   5:     0x7fdf48e43a35 - std::io::Write::write_fmt::h01e5a38c3656642d
                               at /rustc/affe0d3a00e92fa7885e3f5d2c5073fde432d154/library/std/src/io/mod.rs:1672:15
   6:     0x7fdf48e556d1 - std::sys_common::backtrace::_print::h7b5df4d8c3756e42
                               at /rustc/affe0d3a00e92fa7885e3f5d2c5073fde432d154/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7fdf48e556d1 - std::sys_common::backtrace::print::h9d73a5d917f22346
                               at /rustc/affe0d3a00e92fa7885e3f5d2c5073fde432d154/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7fdf48e556d1 - std::panicking::default_hook::{{closure}}::hcf9a8bcb350b3a14
                               at /rustc/affe0d3a00e92fa7885e3f5d2c5073fde432d154/library/std/src/panicking.rs:295:22
   9:     0x7fdf48e5539e - std::panicking::default_hook::hfcc6873a008cb403
                               at /rustc/affe0d3a00e92fa7885e3f5d2c5073fde432d154/library/std/src/panicking.rs:314:9
  10:     0x7fdf4b6cdbe4 - rustc_driver[ca7fdc4c44e2051e]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fdf48e55f06 - std::panicking::rust_panic_with_hook::h4a279a71d839a82c
                               at /rustc/affe0d3a00e92fa7885e3f5d2c5073fde432d154/library/std/src/panicking.rs:702:17
  12:     0x7fdf4c66c851 - std[42edab32bc6d989e]::panicking::begin_panic::<rustc_errors[7749712bdedc646d]::ExplicitBug>::{closure#0}
  13:     0x7fdf4c66bd46 - std[42edab32bc6d989e]::sys_common::backtrace::__rust_end_short_backtrace::<std[42edab32bc6d989e]::panicking::begin_panic<rustc_errors[7749712bdedc646d]::ExplicitBug>::{closure#0}, !>
  14:     0x7fdf4c64ddb6 - std[42edab32bc6d989e]::panicking::begin_panic::<rustc_errors[7749712bdedc646d]::ExplicitBug>
  15:     0x7fdf4c6917a6 - std[42edab32bc6d989e]::panic::panic_any::<rustc_errors[7749712bdedc646d]::ExplicitBug>
  16:     0x7fdf4c6915f5 - <rustc_errors[7749712bdedc646d]::HandlerInner>::bug::<&alloc[67c8b44c78f2df8e]::string::String>
  17:     0x7fdf4c691350 - <rustc_errors[7749712bdedc646d]::Handler>::bug::<&alloc[67c8b44c78f2df8e]::string::String>
  18:     0x7fdf4c6b42cd - rustc_middle[f14eabcfdb3f5619]::ty::context::tls::with_context_opt::<rustc_middle[f14eabcfdb3f5619]::ty::context::tls::with_opt<rustc_middle[f14eabcfdb3f5619]::util::bug::opt_span_bug_fmt<rustc_span[b823e81b2ca00566]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  19:     0x7fdf4c6b7386 - rustc_middle[f14eabcfdb3f5619]::util::bug::opt_span_bug_fmt::<rustc_span[b823e81b2ca00566]::span_encoding::Span>
  20:     0x7fdf4a121bd3 - rustc_middle[f14eabcfdb3f5619]::util::bug::bug_fmt
  21:     0x7fdf4c6de886 - <rustc_middle[f14eabcfdb3f5619]::ty::subst::SubstFolder as rustc_middle[f14eabcfdb3f5619]::ty::fold::TypeFolder>::fold_region::region_param_out_of_range
  22:     0x7fdf4a0e2793 - <rustc_middle[f14eabcfdb3f5619]::ty::subst::SubstFolder as rustc_middle[f14eabcfdb3f5619]::ty::fold::TypeFolder>::fold_region
  23:     0x7fdf4b1c8c99 - <rustc_middle[f14eabcfdb3f5619]::ty::PredicateKind as rustc_middle[f14eabcfdb3f5619]::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle[f14eabcfdb3f5619]::ty::subst::SubstFolder>
  24:     0x7fdf4b295a81 - <rustc_middle[f14eabcfdb3f5619]::ty::subst::SubstFolder as rustc_middle[f14eabcfdb3f5619]::ty::fold::FallibleTypeFolder>::try_fold_binder::<rustc_middle[f14eabcfdb3f5619]::ty::PredicateKind>
  25:     0x7fdf4b295436 - <rustc_infer[7cebeff41e17d02b]::infer::InferCtxt>::register_hidden_type
  26:     0x7fdf4b294ae7 - <rustc_infer[7cebeff41e17d02b]::infer::InferCtxt>::handle_opaque_type::{closure#0}
  27:     0x7fdf4b294405 - <rustc_infer[7cebeff41e17d02b]::infer::InferCtxt>::handle_opaque_type
  28:     0x7fdf4b294039 - <rustc_middle[f14eabcfdb3f5619]::ty::fold::BottomUpFolder<<rustc_infer[7cebeff41e17d02b]::infer::InferCtxt>::replace_opaque_types_with_inference_vars::{closure#3}, <rustc_infer[7cebeff41e17d02b]::infer::InferCtxt>::replace_opaque_types_with_inference_vars::{closure#1}, <rustc_infer[7cebeff41e17d02b]::infer::InferCtxt>::replace_opaque_types_with_inference_vars::{closure#2}> as rustc_middle[f14eabcfdb3f5619]::ty::fold::FallibleTypeFolder>::try_fold_ty
  29:     0x7fdf4b29487f - <&rustc_middle[f14eabcfdb3f5619]::ty::list::List<rustc_middle[f14eabcfdb3f5619]::ty::subst::GenericArg> as rustc_middle[f14eabcfdb3f5619]::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle[f14eabcfdb3f5619]::ty::fold::BottomUpFolder<<rustc_infer[7cebeff41e17d02b]::infer::InferCtxt>::replace_opaque_types_with_inference_vars::{closure#3}, <rustc_infer[7cebeff41e17d02b]::infer::InferCtxt>::replace_opaque_types_with_inference_vars::{closure#1}, <rustc_infer[7cebeff41e17d02b]::infer::InferCtxt>::replace_opaque_types_with_inference_vars::{closure#2}>>
  30:     0x7fdf4b294634 - <rustc_middle[f14eabcfdb3f5619]::ty::Ty as rustc_middle[f14eabcfdb3f5619]::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_middle[f14eabcfdb3f5619]::ty::fold::BottomUpFolder<<rustc_infer[7cebeff41e17d02b]::infer::InferCtxt>::replace_opaque_types_with_inference_vars::{closure#3}, <rustc_infer[7cebeff41e17d02b]::infer::InferCtxt>::replace_opaque_types_with_inference_vars::{closure#1}, <rustc_infer[7cebeff41e17d02b]::infer::InferCtxt>::replace_opaque_types_with_inference_vars::{closure#2}>>
  31:     0x7fdf4b293e1f - <rustc_middle[f14eabcfdb3f5619]::ty::fold::BottomUpFolder<<rustc_infer[7cebeff41e17d02b]::infer::InferCtxt>::replace_opaque_types_with_inference_vars::{closure#3}, <rustc_infer[7cebeff41e17d02b]::infer::InferCtxt>::replace_opaque_types_with_inference_vars::{closure#1}, <rustc_infer[7cebeff41e17d02b]::infer::InferCtxt>::replace_opaque_types_with_inference_vars::{closure#2}> as rustc_middle[f14eabcfdb3f5619]::ty::fold::FallibleTypeFolder>::try_fold_ty
  32:     0x7fdf4a6c76f1 - <rustc_infer[7cebeff41e17d02b]::infer::InferCtxt>::replace_opaque_types_with_inference_vars
  33:     0x7fdf4a1bbc35 - <rustc_typeck[ef3c384faf947dad]::check::fn_ctxt::FnCtxt>::supplied_sig_of_closure
  34:     0x7fdf4a1b887e - <rustc_typeck[ef3c384faf947dad]::check::fn_ctxt::FnCtxt>::check_expr_closure
  35:     0x7fdf4a138191 - <rustc_typeck[ef3c384faf947dad]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  36:     0x7fdf4a12f788 - <rustc_typeck[ef3c384faf947dad]::check::fn_ctxt::FnCtxt>::check_argument_types
  37:     0x7fdf4a0e7527 - <rustc_typeck[ef3c384faf947dad]::check::fn_ctxt::FnCtxt>::check_call
  38:     0x7fdf4a135257 - <rustc_typeck[ef3c384faf947dad]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  39:     0x7fdf4a6c462d - rustc_typeck[ef3c384faf947dad]::check::check::check_fn
  40:     0x7fdf4a6b67de - <rustc_infer[7cebeff41e17d02b]::infer::InferCtxtBuilder>::enter::<&rustc_middle[f14eabcfdb3f5619]::ty::context::TypeckResults, <rustc_typeck[ef3c384faf947dad]::check::inherited::InheritedBuilder>::enter<rustc_typeck[ef3c384faf947dad]::check::typeck_with_fallback<rustc_typeck[ef3c384faf947dad]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[f14eabcfdb3f5619]::ty::context::TypeckResults>::{closure#0}>
  41:     0x7fdf4a6b40b4 - rustc_typeck[ef3c384faf947dad]::check::typeck
  42:     0x7fdf4ad528ef - rustc_query_system[5413e4a7279a8813]::query::plumbing::try_execute_query::<rustc_query_impl[32ca9fca6ee6b15c]::plumbing::QueryCtxt, rustc_query_system[5413e4a7279a8813]::query::caches::DefaultCache<rustc_span[b823e81b2ca00566]::def_id::LocalDefId, &rustc_middle[f14eabcfdb3f5619]::ty::context::TypeckResults>>
  43:     0x7fdf4b5ccb38 - <rustc_query_impl[32ca9fca6ee6b15c]::Queries as rustc_middle[f14eabcfdb3f5619]::ty::query::QueryEngine>::typeck
  44:     0x7fdf4acd2045 - <rustc_middle[f14eabcfdb3f5619]::ty::context::TyCtxt>::typeck_opt_const_arg
  45:     0x7fdf4acd0af5 - rustc_mir_build[72d255824fbbcb84]::build::mir_built
  46:     0x7fdf4ab0062d - rustc_query_system[5413e4a7279a8813]::query::plumbing::try_execute_query::<rustc_query_impl[32ca9fca6ee6b15c]::plumbing::QueryCtxt, rustc_query_system[5413e4a7279a8813]::query::caches::DefaultCache<rustc_middle[f14eabcfdb3f5619]::ty::WithOptConstParam<rustc_span[b823e81b2ca00566]::def_id::LocalDefId>, &rustc_data_structures[4f5439bcc61025e2]::steal::Steal<rustc_middle[f14eabcfdb3f5619]::mir::Body>>>
  47:     0x7fdf4b5cb3c5 - <rustc_query_impl[32ca9fca6ee6b15c]::Queries as rustc_middle[f14eabcfdb3f5619]::ty::query::QueryEngine>::mir_built
  48:     0x7fdf4a6fc027 - rustc_mir_transform[1cb8e5ec8b62e36a]::check_unsafety::unsafety_check_result
  49:     0x7fdf4a6fbabe - <rustc_mir_transform[1cb8e5ec8b62e36a]::check_unsafety::provide::{closure#0} as core[ff7cb6048b1d9418]::ops::function::FnOnce<(rustc_middle[f14eabcfdb3f5619]::ty::context::TyCtxt, rustc_span[b823e81b2ca00566]::def_id::LocalDefId)>>::call_once
  50:     0x7fdf4ac0650f - rustc_query_system[5413e4a7279a8813]::query::plumbing::try_execute_query::<rustc_query_impl[32ca9fca6ee6b15c]::plumbing::QueryCtxt, rustc_query_system[5413e4a7279a8813]::query::caches::DefaultCache<rustc_span[b823e81b2ca00566]::def_id::LocalDefId, &rustc_middle[f14eabcfdb3f5619]::mir::query::UnsafetyCheckResult>>
  51:     0x7fdf4b5cc73e - <rustc_query_impl[32ca9fca6ee6b15c]::Queries as rustc_middle[f14eabcfdb3f5619]::ty::query::QueryEngine>::unsafety_check_result
  52:     0x7fdf4a03eb70 - rustc_mir_transform[1cb8e5ec8b62e36a]::mir_const
  53:     0x7fdf4ab0062d - rustc_query_system[5413e4a7279a8813]::query::plumbing::try_execute_query::<rustc_query_impl[32ca9fca6ee6b15c]::plumbing::QueryCtxt, rustc_query_system[5413e4a7279a8813]::query::caches::DefaultCache<rustc_middle[f14eabcfdb3f5619]::ty::WithOptConstParam<rustc_span[b823e81b2ca00566]::def_id::LocalDefId>, &rustc_data_structures[4f5439bcc61025e2]::steal::Steal<rustc_middle[f14eabcfdb3f5619]::mir::Body>>>
  54:     0x7fdf4b5cb4fc - <rustc_query_impl[32ca9fca6ee6b15c]::Queries as rustc_middle[f14eabcfdb3f5619]::ty::query::QueryEngine>::mir_const
  55:     0x7fdf4a6b10ca - rustc_mir_transform[1cb8e5ec8b62e36a]::mir_promoted
  56:     0x7fdf4b088b5e - rustc_query_system[5413e4a7279a8813]::query::plumbing::get_query::<rustc_query_impl[32ca9fca6ee6b15c]::queries::mir_promoted, rustc_query_impl[32ca9fca6ee6b15c]::plumbing::QueryCtxt>
  57:     0x7fdf4b088797 - <rustc_query_impl[32ca9fca6ee6b15c]::Queries as rustc_middle[f14eabcfdb3f5619]::ty::query::QueryEngine>::mir_promoted
  58:     0x7fdf4b02c24f - rustc_borrowck[c43740bb2a2d5909]::mir_borrowck
  59:     0x7fdf4b02bd6e - <rustc_borrowck[c43740bb2a2d5909]::provide::{closure#0} as core[ff7cb6048b1d9418]::ops::function::FnOnce<(rustc_middle[f14eabcfdb3f5619]::ty::context::TyCtxt, rustc_span[b823e81b2ca00566]::def_id::LocalDefId)>>::call_once
  60:     0x7fdf4aa55d70 - rustc_query_system[5413e4a7279a8813]::query::plumbing::try_execute_query::<rustc_query_impl[32ca9fca6ee6b15c]::plumbing::QueryCtxt, rustc_query_system[5413e4a7279a8813]::query::caches::DefaultCache<rustc_span[b823e81b2ca00566]::def_id::LocalDefId, &rustc_middle[f14eabcfdb3f5619]::mir::query::BorrowCheckResult>>
  61:     0x7fdf4b5ccdaa - <rustc_query_impl[32ca9fca6ee6b15c]::Queries as rustc_middle[f14eabcfdb3f5619]::ty::query::QueryEngine>::mir_borrowck
  62:     0x7fdf4b339c5c - rustc_typeck[ef3c384faf947dad]::collect::type_of::type_of
  63:     0x7fdf4ab0ba5d - rustc_query_system[5413e4a7279a8813]::query::plumbing::get_query::<rustc_query_impl[32ca9fca6ee6b15c]::queries::type_of, rustc_query_impl[32ca9fca6ee6b15c]::plumbing::QueryCtxt>
  64:     0x7fdf4adf2935 - rustc_typeck[ef3c384faf947dad]::check::check::check_item_type
  65:     0x7fdf4adeee1e - rustc_typeck[ef3c384faf947dad]::check::check::check_mod_item_types
  66:     0x7fdf4a90880b - rustc_query_system[5413e4a7279a8813]::query::plumbing::try_execute_query::<rustc_query_impl[32ca9fca6ee6b15c]::plumbing::QueryCtxt, rustc_query_system[5413e4a7279a8813]::query::caches::DefaultCache<rustc_span[b823e81b2ca00566]::def_id::LocalDefId, ()>>
  67:     0x7fdf4b1164c3 - rustc_query_system[5413e4a7279a8813]::query::plumbing::get_query::<rustc_query_impl[32ca9fca6ee6b15c]::queries::check_mod_item_types, rustc_query_impl[32ca9fca6ee6b15c]::plumbing::QueryCtxt>
  68:     0x7fdf4b3ba24f - <rustc_middle[f14eabcfdb3f5619]::hir::map::Map>::for_each_module::<rustc_typeck[ef3c384faf947dad]::check_crate::{closure#6}::{closure#0}>
  69:     0x7fdf4b0ad235 - rustc_typeck[ef3c384faf947dad]::check_crate
  70:     0x7fdf4b0ac877 - rustc_interface[3d3dd0fc21522ad9]::passes::analysis
  71:     0x7fdf4b45fa05 - rustc_query_system[5413e4a7279a8813]::query::plumbing::try_execute_query::<rustc_query_impl[32ca9fca6ee6b15c]::plumbing::QueryCtxt, rustc_query_system[5413e4a7279a8813]::query::caches::DefaultCache<(), core[ff7cb6048b1d9418]::result::Result<(), rustc_errors[7749712bdedc646d]::ErrorGuaranteed>>>
  72:     0x7fdf4b45f78e - rustc_query_system[5413e4a7279a8813]::query::plumbing::get_query::<rustc_query_impl[32ca9fca6ee6b15c]::queries::analysis, rustc_query_impl[32ca9fca6ee6b15c]::plumbing::QueryCtxt>
  73:     0x7fdf4aea5baa - <rustc_interface[3d3dd0fc21522ad9]::interface::Compiler>::enter::<rustc_driver[ca7fdc4c44e2051e]::run_compiler::{closure#1}::{closure#2}, core[ff7cb6048b1d9418]::result::Result<core[ff7cb6048b1d9418]::option::Option<rustc_interface[3d3dd0fc21522ad9]::queries::Linker>, rustc_errors[7749712bdedc646d]::ErrorGuaranteed>>
  74:     0x7fdf4ae995ca - rustc_span[b823e81b2ca00566]::with_source_map::<core[ff7cb6048b1d9418]::result::Result<(), rustc_errors[7749712bdedc646d]::ErrorGuaranteed>, rustc_interface[3d3dd0fc21522ad9]::interface::create_compiler_and_run<core[ff7cb6048b1d9418]::result::Result<(), rustc_errors[7749712bdedc646d]::ErrorGuaranteed>, rustc_driver[ca7fdc4c44e2051e]::run_compiler::{closure#1}>::{closure#1}>
  75:     0x7fdf4ae98fa0 - rustc_interface[3d3dd0fc21522ad9]::interface::create_compiler_and_run::<core[ff7cb6048b1d9418]::result::Result<(), rustc_errors[7749712bdedc646d]::ErrorGuaranteed>, rustc_driver[ca7fdc4c44e2051e]::run_compiler::{closure#1}>
  76:     0x7fdf4ae97311 - <scoped_tls[1695127579464aca]::ScopedKey<rustc_span[b823e81b2ca00566]::SessionGlobals>>::set::<rustc_interface[3d3dd0fc21522ad9]::interface::run_compiler<core[ff7cb6048b1d9418]::result::Result<(), rustc_errors[7749712bdedc646d]::ErrorGuaranteed>, rustc_driver[ca7fdc4c44e2051e]::run_compiler::{closure#1}>::{closure#0}, core[ff7cb6048b1d9418]::result::Result<(), rustc_errors[7749712bdedc646d]::ErrorGuaranteed>>
  77:     0x7fdf4ae96fff - std[42edab32bc6d989e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3d3dd0fc21522ad9]::util::run_in_thread_pool_with_globals<rustc_interface[3d3dd0fc21522ad9]::interface::run_compiler<core[ff7cb6048b1d9418]::result::Result<(), rustc_errors[7749712bdedc646d]::ErrorGuaranteed>, rustc_driver[ca7fdc4c44e2051e]::run_compiler::{closure#1}>::{closure#0}, core[ff7cb6048b1d9418]::result::Result<(), rustc_errors[7749712bdedc646d]::ErrorGuaranteed>>::{closure#0}, core[ff7cb6048b1d9418]::result::Result<(), rustc_errors[7749712bdedc646d]::ErrorGuaranteed>>
  78:     0x7fdf4b4f07d9 - <<std[42edab32bc6d989e]::thread::Builder>::spawn_unchecked_<rustc_interface[3d3dd0fc21522ad9]::util::run_in_thread_pool_with_globals<rustc_interface[3d3dd0fc21522ad9]::interface::run_compiler<core[ff7cb6048b1d9418]::result::Result<(), rustc_errors[7749712bdedc646d]::ErrorGuaranteed>, rustc_driver[ca7fdc4c44e2051e]::run_compiler::{closure#1}>::{closure#0}, core[ff7cb6048b1d9418]::result::Result<(), rustc_errors[7749712bdedc646d]::ErrorGuaranteed>>::{closure#0}, core[ff7cb6048b1d9418]::result::Result<(), rustc_errors[7749712bdedc646d]::ErrorGuaranteed>>::{closure#1} as core[ff7cb6048b1d9418]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  79:     0x7fdf48e5fed3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hf47b126657473903
                               at /rustc/affe0d3a00e92fa7885e3f5d2c5073fde432d154/library/alloc/src/boxed.rs:1935:9
  80:     0x7fdf48e5fed3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h674e0463c0783734
                               at /rustc/affe0d3a00e92fa7885e3f5d2c5073fde432d154/library/alloc/src/boxed.rs:1935:9
  81:     0x7fdf48e5fed3 - std::sys::unix::thread::Thread::new::thread_start::ha7dc26e0bc2fdd91
                               at /rustc/affe0d3a00e92fa7885e3f5d2c5073fde432d154/library/std/src/sys/unix/thread.rs:108:17
  82:     0x7fdf48ba554d - <unknown>
  83:     0x7fdf48c2a874 - clone
  84:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (affe0d3a0 2022-08-05) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `executor::select::<impl at /home/vtvz/.cargo/registry/src/github.com-1ecc6299db9ec823/sea-orm-0.9.0/src/executor/select.rs:113:1: 113:18>::stream`
#1 [mir_built] building MIR for `executor::select::<impl at /home/vtvz/.cargo/registry/src/github.com-1ecc6299db9ec823/sea-orm-0.9.0/src/executor/select.rs:113:1: 113:18>::stream`
#2 [unsafety_check_result] unsafety-checking `executor::select::<impl at /home/vtvz/.cargo/registry/src/github.com-1ecc6299db9ec823/sea-orm-0.9.0/src/executor/select.rs:113:1: 113:18>::stream`
#3 [mir_const] processing MIR for `executor::select::<impl at /home/vtvz/.cargo/registry/src/github.com-1ecc6299db9ec823/sea-orm-0.9.0/src/executor/select.rs:113:1: 113:18>::stream`
#4 [mir_promoted] processing `executor::select::<impl at /home/vtvz/.cargo/registry/src/github.com-1ecc6299db9ec823/sea-orm-0.9.0/src/executor/select.rs:113:1: 113:18>::stream`
#5 [mir_borrowck] borrow-checking `executor::select::<impl at /home/vtvz/.cargo/registry/src/github.com-1ecc6299db9ec823/sea-orm-0.9.0/src/executor/select.rs:113:1: 113:18>::stream`
#6 [type_of] computing type of `executor::select::<impl at /home/vtvz/.cargo/registry/src/github.com-1ecc6299db9ec823/sea-orm-0.9.0/src/executor/select.rs:113:1: 113:18>::stream::{opaque#0}`
#7 [check_mod_item_types] checking item types in module `executor::select`
#8 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `sea-orm`
