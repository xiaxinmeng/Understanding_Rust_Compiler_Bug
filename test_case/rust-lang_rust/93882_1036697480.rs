powershell
error: internal compiler error: Encountered errors `[FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<T as core::marker::Sized>, polarity:Positive), []), depth=1),Unimplemented), FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<T as core_simd::MaskElement>, polarity:Positive), []), depth=1),Unimplemented)]` resolving bounds outside of type inference

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1200:27
stack backtrace:
   0:     0x7f3a77b7cb6c - std::backtrace_rs::backtrace::libunwind::trace::h824347123a539ac3
                               at /rustc/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f3a77b7cb6c - std::backtrace_rs::backtrace::trace_unsynchronized::h30996243a6d4a816
                               at /rustc/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f3a77b7cb6c - std::sys_common::backtrace::_print_fmt::he0e84aa8853fdb2a
                               at /rustc/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f3a77b7cb6c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h699cf46bd548daad
                               at /rustc/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f3a77bde24c - core::fmt::write::hc21dca261faff02f
                               at /rustc/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/core/src/fmt/mod.rs:1190:17
   5:     0x7f3a77b6ce98 - std::io::Write::write_fmt::ha562dab75b1aee7a
                               at /rustc/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/std/src/io/mod.rs:1657:15
   6:     0x7f3a77b80ae7 - std::sys_common::backtrace::_print::h6995311cb4b18c83
                               at /rustc/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f3a77b80ae7 - std::sys_common::backtrace::print::h5c00a73bc75b17c4
                               at /rustc/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f3a77b80ae7 - std::panicking::default_hook::{{closure}}::he5c22156c1cfda73
                               at /rustc/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/std/src/panicking.rs:295:22
   9:     0x7f3a77b807af - std::panicking::default_hook::hd9340e2830e1e464
                               at /rustc/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/std/src/panicking.rs:314:9
  10:     0x7f3a7830dd91 - rustc_driver[f7ae5b3e036b262e]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f3a77b813c5 - std::panicking::rust_panic_with_hook::hb17e9d2f86437995
                               at /rustc/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/std/src/panicking.rs:702:17
  12:     0x7f3a77b81039 - std::panicking::begin_panic_handler::{{closure}}::h5ea470c57cd887ae
                               at /rustc/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/std/src/panicking.rs:586:13
  13:     0x7f3a77b7d014 - std::sys_common::backtrace::__rust_end_short_backtrace::hdf89f68b6e6aa66b
                               at /rustc/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7f3a77b80d79 - rust_begin_unwind
                               at /rustc/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/std/src/panicking.rs:584:5
  15:     0x7f3a77b48d03 - core::panicking::panic_fmt::h0b506cc131461e16
                               at /rustc/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/core/src/panicking.rs:143:14
  16:     0x7f3a793fe82e - <rustc_errors[59c9bb132555b663]::HandlerInner>::panic_if_treat_err_as_bug
  17:     0x7f3a7aa7f7e4 - <rustc_errors[59c9bb132555b663]::HandlerInner>::emit_diagnostic
  18:     0x7f3a7912a391 - <rustc_errors[59c9bb132555b663]::HandlerInner>::emit_diag_at_span::<rustc_span[eca3046f9c14e76]::span_encoding::Span>
  19:     0x7f3a7912a363 - <rustc_errors[59c9bb132555b663]::HandlerInner>::span_bug::<rustc_span[eca3046f9c14e76]::span_encoding::Span>
  20:     0x7f3a7912cef5 - <rustc_session[e825f980c6f8a293]::session::Session>::delay_span_bug::<rustc_span[eca3046f9c14e76]::span_encoding::Span>
  21:     0x7f3a79ee185a - <rustc_infer[8ea8801e962e8d71]::infer::InferCtxtBuilder>::enter::<core[b9efde14e06b09ed]::result::Result<rustc_middle[69fe71bd9bb50da9]::traits::ImplSource<()>, rustc_errors[59c9bb132555b663]::ErrorReported>, rustc_trait_selection[f7534e1344bd6007]::traits::codegen::codegen_fulfill_obligation::{closure#0}>
  22:     0x7f3a79f6d21a - rustc_trait_selection[f7534e1344bd6007]::traits::codegen::codegen_fulfill_obligation
  23:     0x7f3a79c064cc - rustc_query_system[fb683e47e947d40]::query::plumbing::try_execute_query::<rustc_query_impl[576a7c6b4634b180]::plumbing::QueryCtxt, rustc_query_system[fb683e47e947d40]::query::caches::DefaultCache<(rustc_middle[69fe71bd9bb50da9]::ty::ParamEnv, rustc_middle[69fe71bd9bb50da9]::ty::sty::Binder<rustc_middle[69fe71bd9bb50da9]::ty::sty::TraitRef>), core[b9efde14e06b09ed]::result::Result<rustc_middle[69fe71bd9bb50da9]::traits::ImplSource<()>, rustc_errors[59c9bb132555b663]::ErrorReported>>>
  24:     0x7f3a79c4ef2a - <rustc_query_impl[576a7c6b4634b180]::Queries as rustc_middle[69fe71bd9bb50da9]::ty::query::QueryEngine>::codegen_fulfill_obligation
  25:     0x7f3a79814517 - rustc_ty_utils[76f99f00e579ed4c]::instance::inner_resolve_instance
  26:     0x7f3a79813626 - rustc_ty_utils[76f99f00e579ed4c]::instance::resolve_instance
  27:     0x7f3a79c19f3b - rustc_query_system[fb683e47e947d40]::query::plumbing::get_query::<rustc_query_impl[576a7c6b4634b180]::queries::resolve_instance, rustc_query_impl[576a7c6b4634b180]::plumbing::QueryCtxt>
  28:     0x7f3a79c51dfb - <rustc_query_impl[576a7c6b4634b180]::Queries as rustc_middle[69fe71bd9bb50da9]::ty::query::QueryEngine>::resolve_instance
  29:     0x7f3a7a0afd75 - <rustc_middle[69fe71bd9bb50da9]::ty::instance::Instance>::resolve_opt_const_arg
  30:     0x7f3a7a9da5cb - <rustc_middle[69fe71bd9bb50da9]::ty::context::TyCtxt>::const_eval_resolve
  31:     0x7f3a79f65514 - <rustc_trait_selection[f7534e1344bd6007]::traits::query::normalize::QueryNormalizer as rustc_middle[69fe71bd9bb50da9]::ty::fold::FallibleTypeFolder>::try_fold_const
  32:     0x7f3a79f65801 - <rustc_trait_selection[f7534e1344bd6007]::traits::query::normalize::QueryNormalizer as rustc_middle[69fe71bd9bb50da9]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  33:     0x7f3a79bb2b15 - <rustc_infer[8ea8801e962e8d71]::infer::InferCtxtBuilder>::enter::<core[b9efde14e06b09ed]::result::Result<rustc_middle[69fe71bd9bb50da9]::mir::ConstantKind, rustc_middle[69fe71bd9bb50da9]::traits::query::NoSolution>, rustc_traits[401f2dcba686126]::normalize_erasing_regions::try_normalize_after_erasing_regions<rustc_middle[69fe71bd9bb50da9]::mir::ConstantKind>::{closure#0}>
  34:     0x7f3a79bbdc98 - <rustc_traits[401f2dcba686126]::normalize_erasing_regions::provide::{closure#1} as core[b9efde14e06b09ed]::ops::function::FnOnce<(rustc_middle[69fe71bd9bb50da9]::ty::context::TyCtxt, rustc_middle[69fe71bd9bb50da9]::ty::ParamEnvAnd<rustc_middle[69fe71bd9bb50da9]::mir::ConstantKind>)>>::call_once
  35:     0x7f3a79bd84b8 - rustc_query_system[fb683e47e947d40]::query::plumbing::try_execute_query::<rustc_query_impl[576a7c6b4634b180]::plumbing::QueryCtxt, rustc_query_system[fb683e47e947d40]::query::caches::DefaultCache<rustc_middle[69fe71bd9bb50da9]::ty::ParamEnvAnd<rustc_middle[69fe71bd9bb50da9]::mir::ConstantKind>, core[b9efde14e06b09ed]::result::Result<rustc_middle[69fe71bd9bb50da9]::mir::ConstantKind, rustc_middle[69fe71bd9bb50da9]::traits::query::NoSolution>>>
  36:     0x7f3a79c519b4 - <rustc_query_impl[576a7c6b4634b180]::Queries as rustc_middle[69fe71bd9bb50da9]::ty::query::QueryEngine>::try_normalize_mir_const_after_erasing_regions
  37:     0x7f3a7a121c7f - <rustc_middle[69fe71bd9bb50da9]::ty::normalize_erasing_regions::TryNormalizeAfterErasingRegionsFolder as rustc_middle[69fe71bd9bb50da9]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  38:     0x7f3a7a5be733 - <rustc_const_eval[7cfdc43ee7de405e]::interpret::eval_context::InterpCx<rustc_const_eval[7cfdc43ee7de405e]::const_eval::machine::CompileTimeInterpreter>>::subst_from_current_frame_and_normalize_erasing_regions::<rustc_middle[69fe71bd9bb50da9]::mir::ConstantKind>
  39:     0x7f3a79ad058c - <rustc_const_eval[7cfdc43ee7de405e]::interpret::eval_context::InterpCx<rustc_const_eval[7cfdc43ee7de405e]::const_eval::machine::CompileTimeInterpreter>>::push_stack_frame
  40:     0x7f3a79b5e849 - rustc_const_eval[7cfdc43ee7de405e]::const_eval::eval_queries::eval_to_allocation_raw_provider
  41:     0x7f3a79bd9ac2 - rustc_query_system[fb683e47e947d40]::query::plumbing::try_execute_query::<rustc_query_impl[576a7c6b4634b180]::plumbing::QueryCtxt, rustc_query_system[fb683e47e947d40]::query::caches::DefaultCache<rustc_middle[69fe71bd9bb50da9]::ty::ParamEnvAnd<rustc_middle[69fe71bd9bb50da9]::mir::interpret::GlobalId>, core[b9efde14e06b09ed]::result::Result<rustc_middle[69fe71bd9bb50da9]::mir::interpret::value::ConstAlloc, rustc_middle[69fe71bd9bb50da9]::mir::interpret::error::ErrorHandled>>>
  42:     0x7f3a79c4a73f - <rustc_query_impl[576a7c6b4634b180]::Queries as rustc_middle[69fe71bd9bb50da9]::ty::query::QueryEngine>::eval_to_allocation_raw
  43:     0x7f3a79b5d493 - rustc_const_eval[7cfdc43ee7de405e]::const_eval::eval_queries::eval_to_const_value_raw_provider
  44:     0x7f3a79bda9b7 - rustc_query_system[fb683e47e947d40]::query::plumbing::try_execute_query::<rustc_query_impl[576a7c6b4634b180]::plumbing::QueryCtxt, rustc_query_system[fb683e47e947d40]::query::caches::DefaultCache<rustc_middle[69fe71bd9bb50da9]::ty::ParamEnvAnd<rustc_middle[69fe71bd9bb50da9]::mir::interpret::GlobalId>, core[b9efde14e06b09ed]::result::Result<rustc_middle[69fe71bd9bb50da9]::mir::interpret::value::ConstValue, rustc_middle[69fe71bd9bb50da9]::mir::interpret::error::ErrorHandled>>>
  45:     0x7f3a79c4a8fb - <rustc_query_impl[576a7c6b4634b180]::Queries as rustc_middle[69fe71bd9bb50da9]::ty::query::QueryEngine>::eval_to_const_value_raw
  46:     0x7f3a79b5d1d6 - rustc_const_eval[7cfdc43ee7de405e]::const_eval::eval_queries::eval_to_const_value_raw_provider
  47:     0x7f3a79bda9b7 - rustc_query_system[fb683e47e947d40]::query::plumbing::try_execute_query::<rustc_query_impl[576a7c6b4634b180]::plumbing::QueryCtxt, rustc_query_system[fb683e47e947d40]::query::caches::DefaultCache<rustc_middle[69fe71bd9bb50da9]::ty::ParamEnvAnd<rustc_middle[69fe71bd9bb50da9]::mir::interpret::GlobalId>, core[b9efde14e06b09ed]::result::Result<rustc_middle[69fe71bd9bb50da9]::mir::interpret::value::ConstValue, rustc_middle[69fe71bd9bb50da9]::mir::interpret::error::ErrorHandled>>>
  48:     0x7f3a79c4a8fb - <rustc_query_impl[576a7c6b4634b180]::Queries as rustc_middle[69fe71bd9bb50da9]::ty::query::QueryEngine>::eval_to_const_value_raw
  49:     0x7f3a7a063225 - <rustc_middle[69fe71bd9bb50da9]::ty::context::TyCtxt>::const_eval_global_id
  50:     0x7f3a7a9da609 - <rustc_middle[69fe71bd9bb50da9]::ty::context::TyCtxt>::const_eval_resolve
  51:     0x560e221a9b5a - <&rustc_middle[69fe71bd9bb50da9]::ty::TyS as rustdoc[e5ecb062f4162e39]::clean::Clean<rustdoc[e5ecb062f4162e39]::clean::types::Type>>::clean
  52:     0x560e22026bbf - rustdoc[e5ecb062f4162e39]::clean::clean_fn_decl_from_did_and_sig
  53:     0x560e21fdda29 - <rustc_middle[69fe71bd9bb50da9]::ty::assoc::AssocItem as rustdoc[e5ecb062f4162e39]::clean::Clean<rustdoc[e5ecb062f4162e39]::clean::types::Item>>::clean
  54:     0x560e21f9385f - <alloc[14a0d6d4553afc6a]::vec::Vec<rustdoc[e5ecb062f4162e39]::clean::types::Item> as alloc[14a0d6d4553afc6a]::vec::spec_from_iter::SpecFromIter<rustdoc[e5ecb062f4162e39]::clean::types::Item, core[b9efde14e06b09ed]::iter::adapters::filter_map::FilterMap<core[b9efde14e06b09ed]::iter::adapters::map::Map<core[b9efde14e06b09ed]::iter::adapters::map::Map<core[b9efde14e06b09ed]::slice::iter::Iter<(rustc_span[eca3046f9c14e76]::symbol::Symbol, &rustc_middle[69fe71bd9bb50da9]::ty::assoc::AssocItem)>, <rustc_data_structures[291ebb7c081c24f5]::sorted_map::index_map::SortedIndexMultiMap<u32, rustc_span[eca3046f9c14e76]::symbol::Symbol, &rustc_middle[69fe71bd9bb50da9]::ty::assoc::AssocItem>>::iter::{closure#0}>, <rustc_middle[69fe71bd9bb50da9]::ty::assoc::AssocItems>::in_definition_order::{closure#0}>, rustdoc[e5ecb062f4162e39]::clean::inline::build_impl<core[b9efde14e06b09ed]::option::Option<rustc_span[eca3046f9c14e76]::def_id::DefId>>::{closure#3}>>>::from_iter
  55:     0x560e2209ab1a - rustdoc[e5ecb062f4162e39]::clean::inline::build_impl::<core[b9efde14e06b09ed]::option::Option<rustc_span[eca3046f9c14e76]::def_id::DefId>>
  56:     0x560e220faac1 - <rustdoc[e5ecb062f4162e39]::core::DocContext>::with_all_trait_impls::<rustdoc[e5ecb062f4162e39]::passes::collect_trait_impls::collect_trait_impls::{closure#3}>
  57:     0x560e2215ae37 - rustdoc[e5ecb062f4162e39]::passes::collect_trait_impls::collect_trait_impls
  58:     0x560e21ef19c4 - <rustc_session[e825f980c6f8a293]::session::Session>::time::<rustdoc[e5ecb062f4162e39]::clean::types::Crate, rustdoc[e5ecb062f4162e39]::core::run_global_ctxt::{closure#8}>
  59:     0x560e220ff413 - rustdoc[e5ecb062f4162e39]::core::run_global_ctxt
  60:     0x560e21ef1e42 - <rustc_session[e825f980c6f8a293]::session::Session>::time::<(rustdoc[e5ecb062f4162e39]::clean::types::Crate, rustdoc[e5ecb062f4162e39]::config::RenderOptions, rustdoc[e5ecb062f4162e39]::formats::cache::Cache), rustdoc[e5ecb062f4162e39]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#0}>
  61:     0x560e2212b890 - <rustc_interface[f21518ebe01ac716]::passes::QueryContext>::enter::<rustdoc[e5ecb062f4162e39]::main_options::{closure#0}::{closure#0}::{closure#1}, core[b9efde14e06b09ed]::result::Result<(), rustc_errors[59c9bb132555b663]::ErrorReported>>
  62:     0x560e2209f8be - <rustc_interface[f21518ebe01ac716]::interface::Compiler>::enter::<rustdoc[e5ecb062f4162e39]::main_options::{closure#0}::{closure#0}, core[b9efde14e06b09ed]::result::Result<(), rustc_errors[59c9bb132555b663]::ErrorReported>>
  63:     0x560e2212d9bc - rustc_span[eca3046f9c14e76]::with_source_map::<core[b9efde14e06b09ed]::result::Result<(), rustc_errors[59c9bb132555b663]::ErrorReported>, rustc_interface[f21518ebe01ac716]::interface::create_compiler_and_run<core[b9efde14e06b09ed]::result::Result<(), rustc_errors[59c9bb132555b663]::ErrorReported>, rustdoc[e5ecb062f4162e39]::main_options::{closure#0}>::{closure#1}>
  64:     0x560e2209062d - rustc_interface[f21518ebe01ac716]::interface::create_compiler_and_run::<core[b9efde14e06b09ed]::result::Result<(), rustc_errors[59c9bb132555b663]::ErrorReported>, rustdoc[e5ecb062f4162e39]::main_options::{closure#0}>
  65:     0x560e21edd6cd - rustdoc[e5ecb062f4162e39]::main_options
  66:     0x560e21fc8a8d - <scoped_tls[e9ba5f945198f548]::ScopedKey<rustc_span[eca3046f9c14e76]::SessionGlobals>>::set::<rustc_interface[f21518ebe01ac716]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustdoc[e5ecb062f4162e39]::main_args::{closure#0}, core[b9efde14e06b09ed]::result::Result<(), rustc_errors[59c9bb132555b663]::ErrorReported>>::{closure#0}::{closure#0}, core[b9efde14e06b09ed]::result::Result<(), rustc_errors[59c9bb132555b663]::ErrorReported>>
  67:     0x560e22068d39 - std[1e60cef5fcfeb639]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f21518ebe01ac716]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustdoc[e5ecb062f4162e39]::main_args::{closure#0}, core[b9efde14e06b09ed]::result::Result<(), rustc_errors[59c9bb132555b663]::ErrorReported>>::{closure#0}, core[b9efde14e06b09ed]::result::Result<(), rustc_errors[59c9bb132555b663]::ErrorReported>>
  68:     0x560e21ef0ae9 - <<std[1e60cef5fcfeb639]::thread::Builder>::spawn_unchecked_<rustc_interface[f21518ebe01ac716]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustdoc[e5ecb062f4162e39]::main_args::{closure#0}, core[b9efde14e06b09ed]::result::Result<(), rustc_errors[59c9bb132555b663]::ErrorReported>>::{closure#0}, core[b9efde14e06b09ed]::result::Result<(), rustc_errors[59c9bb132555b663]::ErrorReported>>::{closure#1} as core[b9efde14e06b09ed]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  69:     0x7f3a77b8ce13 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h6b24c31e6604f724
                               at /rustc/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/alloc/src/boxed.rs:1854:9
  70:     0x7f3a77b8ce13 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h357a4d585864792d
                               at /rustc/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/alloc/src/boxed.rs:1854:9
  71:     0x7f3a77b8ce13 - std::sys::unix::thread::Thread::new::thread_start::h571ee483c9a2c616
                               at /rustc/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/std/src/sys/unix/thread.rs:108:17
  72:     0x7f3a77aa4259 - start_thread
  73:     0x7f3a7785a5e3 - __GI___clone
  74:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (e7aca8959 2022-02-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -Z treat-err-as-bug

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [codegen_fulfill_obligation] checking if `core_simd::core_simd::masks::to_bitmask::ToBitMaskArray` fulfills its obligations
#1 [resolve_instance] resolving instance `<core_simd::core_simd::masks::Mask<T, 32_usize> as core_simd::core_simd::masks::to_bitmask::ToBitMaskArray>::BYTES`
#2 [try_normalize_mir_const_after_erasing_regions] normalizing `<core_simd::core_simd::masks::Mask<T, 32_usize> as core_simd::core_simd::masks::to_bitmask::ToBitMaskArray>::BYTES`
#3 [eval_to_allocation_raw] const-evaluating + checking `<core_simd::core_simd::masks::Mask<T, 32_usize> as core_simd::core_simd::masks::to_bitmask::ToBitMaskArray>::to_bitmask_array::{constant#0}`
#4 [eval_to_const_value_raw] simplifying constant for the type system `<core_simd::core_simd::masks::Mask<T, 32_usize> as core_simd::core_simd::masks::to_bitmask::ToBitMaskArray>::to_bitmask_array::{constant#0}`
#5 [eval_to_const_value_raw] simplifying constant for the type system `<core_simd::core_simd::masks::Mask<T, 32_usize> as core_simd::core_simd::masks::to_bitmask::ToBitMaskArray>::to_bitmask_array::{constant#0}`
end of query stack
