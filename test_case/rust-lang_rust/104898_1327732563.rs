plain
........................................................................................ 352/13908
..................................F..................................................... 440/13908
........................................................................................ 528/13908
........................................................................................ 616/13908
...............................................F........................................ 704/13908
...........................................F....................F....................... 792/13908
...................................i.................................................... 968/13908
........................................................................................ 1056/13908
........................................................................................ 1144/13908
........................................................................................ 1232/13908
---
........................................................................................ 2904/13908
........................................................................................ 2992/13908
........................................................................................ 3080/13908
.......................................................i................................ 3168/13908
......................i........................................FF............F...F.....F 3256/13908
...F.................................................................................... 3344/13908
.....iiiii.............................................................................. 3520/13908
........................................................................................ 3608/13908
........................................................................................ 3696/13908
........................................................................................ 3784/13908
---
failures:

---- [ui] src/test/ui/associated-type-bounds/traits-assoc-type-macros.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/traits-assoc-type-macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/traits-assoc-type-macros/traits-assoc-type-macros.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/traits-assoc-type-macros" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/traits-assoc-type-macros/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'could not lift for printing', compiler/rustc_middle/src/ty/print/pretty.rs:2542:1
   0:     0x7fb220d03ee5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1a7649ac77790747
   0:     0x7fb220d03ee5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1a7649ac77790747
   1:     0x7fb220d73b28 - core::fmt::write::h2987f1da831254ba
   2:     0x7fb220cf5c91 - std::io::Write::write_fmt::h9134dac7fedc552d
   3:     0x7fb220d03cf1 - std::sys_common::backtrace::print::h9bbeb4ce4ce0398b
   4:     0x7fb220d07034 - std::panicking::default_hook::{{closure}}::ha08f9e0148f6988f
   5:     0x7fb220d06cfa - std::panicking::default_hook::h1611fbcdf9dd28c0
   6:     0x7fb22174c764 - rustc_driver[466f9bbdde2d424f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fb220d077a4 - std::panicking::rust_panic_with_hook::hb4658a518d8f7f94
   8:     0x7fb220d07509 - std::panicking::begin_panic_handler::{{closure}}::hd014742727ccfc65
   9:     0x7fb220d04404 - std::sys_common::backtrace::__rust_end_short_backtrace::hf7cef729504cc08a
  10:     0x7fb220d071b2 - rust_begin_unwind
  11:     0x7fb220cb7ff3 - core::panicking::panic_fmt::hc017567c8322f888
  12:     0x7fb220d70311 - core::panicking::panic_display::hb0348ae40115c924
  13:     0x7fb220d702bb - core::panicking::panic_str::h9deed821c280dc1c
  14:     0x7fb220cb7fb6 - core::option::expect_failed::ha18499e3b0d5a4f5
  15:     0x7fb2243f658c - <rustc_middle[cba042a01728d163]::ty::Ty as core[2421c901ece542d5]::fmt::Display>::fmt
  16:     0x7fb220d73b28 - core::fmt::write::h2987f1da831254ba
  17:     0x7fb22456f25a - <rustc_middle[cba042a01728d163]::ty::print::pretty::FmtPrinter as core[2421c901ece542d5]::fmt::Write>::write_fmt
  18:     0x7fb2245371a5 - <rustc_middle[cba042a01728d163]::ty::sty::TraitRef as rustc_middle[cba042a01728d163]::ty::print::Print<rustc_middle[cba042a01728d163]::ty::print::pretty::FmtPrinter>>::print
  19:     0x7fb224537b2e - <rustc_middle[cba042a01728d163]::ty::sty::TraitRef as core[2421c901ece542d5]::fmt::Display>::fmt
  20:     0x7fb22452b566 - <rustc_middle[cba042a01728d163]::ty::sty::TraitRef as core[2421c901ece542d5]::fmt::Debug>::fmt
  21:     0x7fb220d73b28 - core::fmt::write::h2987f1da831254ba
  22:     0x7fb220d74ac3 - core::fmt::Formatter::write_fmt::hb1a8980ba87e6e48
  23:     0x7fb2243ef9fd - <rustc_middle[cba042a01728d163]::ty::PredicateKind as core[2421c901ece542d5]::fmt::Debug>::fmt
  24:     0x7fb220d70b30 - core::fmt::builders::DebugTuple::field::hdb9654b4a89e4676
  25:     0x7fb220d752b5 - core::fmt::Formatter::debug_tuple_field2_finish::hec12b9e69f1117f9
  26:     0x7fb224539d06 - <rustc_middle[cba042a01728d163]::ty::sty::Binder<rustc_middle[cba042a01728d163]::ty::PredicateKind> as core[2421c901ece542d5]::fmt::Debug>::fmt
  27:     0x7fb220d73b28 - core::fmt::write::h2987f1da831254ba
  28:     0x7fb220d74ac3 - core::fmt::Formatter::write_fmt::hb1a8980ba87e6e48
  29:     0x7fb2243e8029 - <&rustc_middle[cba042a01728d163]::ty::Predicate as core[2421c901ece542d5]::fmt::Debug>::fmt
  30:     0x7fb220d70ce8 - core::fmt::builders::DebugInner::entry::h07815c3ffe0fcc3c
  31:     0x7fb220d70da9 - core::fmt::builders::DebugSet::entry::h7ad70c19ae56adfa
  32:     0x7fb2245417e3 - <core[2421c901ece542d5]::fmt::builders::DebugList>::entries::<&rustc_middle[cba042a01728d163]::ty::Predicate, core[2421c901ece542d5]::slice::iter::Iter<rustc_middle[cba042a01728d163]::ty::Predicate>>
  33:     0x7fb2243e8acd - <[rustc_middle[cba042a01728d163]::ty::Predicate] as core[2421c901ece542d5]::fmt::Debug>::fmt
  34:     0x7fb220d708ac - core::fmt::builders::DebugStruct::field::ha752fa0c7987e9bc
  35:     0x7fb2243f45e5 - <rustc_middle[cba042a01728d163]::ty::ParamEnv as core[2421c901ece542d5]::fmt::Debug>::fmt
  36:     0x7fb220d708ac - core::fmt::builders::DebugStruct::field::ha752fa0c7987e9bc
  37:     0x7fb220d74ca4 - core::fmt::Formatter::debug_struct_field2_finish::h9d4571940b1c39d3
  38:     0x7fb2233ff829 - <&rustc_middle[cba042a01728d163]::ty::ParamEnvAnd<rustc_middle[cba042a01728d163]::ty::Ty> as core[2421c901ece542d5]::fmt::Debug>::fmt
  39:     0x7fb220d708ac - core::fmt::builders::DebugStruct::field::ha752fa0c7987e9bc
  40:     0x7fb220d74dac - core::fmt::Formatter::debug_struct_field3_finish::hb172eace3c23e018
  41:     0x7fb2231b6c90 - <rustc_middle[cba042a01728d163]::infer::canonical::Canonical<rustc_middle[cba042a01728d163]::ty::ParamEnvAnd<rustc_middle[cba042a01728d163]::ty::Ty>> as core[2421c901ece542d5]::fmt::Debug>::fmt
  42:     0x7fb220d73b28 - core::fmt::write::h2987f1da831254ba
  43:     0x7fb220d07389 - <std::panicking::begin_panic_handler::PanicPayload as core::panic::BoxMeUp>::get::h924f4b1c45865160
  44:     0x7fb220d0778a - std::panicking::rust_panic_with_hook::hb4658a518d8f7f94
  45:     0x7fb220d07509 - std::panicking::begin_panic_handler::{{closure}}::hd014742727ccfc65
  46:     0x7fb220d04404 - std::sys_common::backtrace::__rust_end_short_backtrace::hf7cef729504cc08a
  47:     0x7fb220d071b2 - rust_begin_unwind
  48:     0x7fb220cb7ff3 - core::panicking::panic_fmt::hc017567c8322f888
  49:     0x7fb2232b00fa - <rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_middle[cba042a01728d163]::infer::canonical::Canonical<rustc_middle[cba042a01728d163]::ty::ParamEnvAnd<rustc_middle[cba042a01728d163]::ty::Ty>>, core[2421c901ece542d5]::result::Result<&rustc_middle[cba042a01728d163]::infer::canonical::Canonical<rustc_middle[cba042a01728d163]::infer::canonical::QueryResponse<alloc[a565b7b4b1676a09]::vec::Vec<rustc_middle[cba042a01728d163]::traits::query::OutlivesBound>>>, rustc_middle[cba042a01728d163]::traits::query::NoSolution>>
  50:     0x7fb223618275 - rustc_query_system[8b69943028d6b97f]::query::plumbing::get_query::<rustc_query_impl[6b04a642019f58b7]::queries::implied_outlives_bounds, rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt>
  51:     0x7fb22324b1a6 - <rustc_query_impl[6b04a642019f58b7]::Queries as rustc_middle[cba042a01728d163]::ty::query::QueryEngine>::implied_outlives_bounds
  52:     0x7fb2240855a4 - <rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::implied_outlives_bounds::ImpliedOutlivesBounds as rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::QueryTypeOp>::perform_query
  53:     0x7fb22408579f - <rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::implied_outlives_bounds::ImpliedOutlivesBounds as rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::QueryTypeOp>::fully_perform_into
  54:     0x7fb22415c528 - <rustc_middle[cba042a01728d163]::ty::ParamEnvAnd<rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::implied_outlives_bounds::ImpliedOutlivesBounds> as rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::TypeOp>::fully_perform
  55:     0x7fb224069228 - <rustc_infer[4839981d34d86096]::infer::InferCtxt as rustc_trait_selection[972a6de06ac5a492]::traits::outlives_bounds::InferCtxtExt>::implied_outlives_bounds
  56:     0x7fb222050663 - <&mut <rustc_infer[4839981d34d86096]::infer::InferCtxt as rustc_trait_selection[972a6de06ac5a492]::traits::outlives_bounds::InferCtxtExt>::implied_bounds_tys::{closure#0} as core[2421c901ece542d5]::ops::function::FnOnce<(rustc_middle[cba042a01728d163]::ty::Ty,)>>::call_once
  57:     0x7fb221ecf920 - <rustc_infer[4839981d34d86096]::infer::outlives::env::OutlivesEnvironment>::with_bounds::<core[2421c901ece542d5]::iter::adapters::flatten::Flatten<core[2421c901ece542d5]::iter::adapters::map::Map<indexmap[ee6033d77fa8f813]::set::IntoIter<rustc_middle[cba042a01728d163]::ty::Ty>, <rustc_infer[4839981d34d86096]::infer::InferCtxt as rustc_trait_selection[972a6de06ac5a492]::traits::outlives_bounds::InferCtxtExt>::implied_bounds_tys::{closure#0}>>>
  58:     0x7fb2220da04c - rustc_hir_analysis[dc2d927b109b9899]::check::compare_method::compare_predicate_entailment
  59:     0x7fb2220c6b86 - rustc_hir_analysis[dc2d927b109b9899]::check::compare_method::compare_impl_method
  60:     0x7fb221f3d3ff - rustc_hir_analysis[dc2d927b109b9899]::check::check::check_impl_items_against_trait
  61:     0x7fb221f39479 - rustc_hir_analysis[dc2d927b109b9899]::check::check::check_item_type
  62:     0x7fb221f4207a - rustc_hir_analysis[dc2d927b109b9899]::check::check::check_mod_item_types
  63:     0x7fb22311be8e - <rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind as rustc_query_system[8b69943028d6b97f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_span[4dd1b2649365341f]::def_id::LocalDefId, ()>::{closure#0}, ()>
  64:     0x7fb2232c726f - <rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_span[4dd1b2649365341f]::def_id::LocalDefId, ()>
  65:     0x7fb22354fadf - rustc_query_system[8b69943028d6b97f]::query::plumbing::try_execute_query::<rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt, rustc_query_system[8b69943028d6b97f]::query::caches::VecCache<rustc_span[4dd1b2649365341f]::def_id::LocalDefId, ()>>
  66:     0x7fb223605aa3 - rustc_query_system[8b69943028d6b97f]::query::plumbing::get_query::<rustc_query_impl[6b04a642019f58b7]::queries::check_mod_item_types, rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt>
  67:     0x7fb2231ff710 - <rustc_query_impl[6b04a642019f58b7]::Queries as rustc_middle[cba042a01728d163]::ty::query::QueryEngine>::check_mod_item_types
  68:     0x7fb221fa8bda - <rustc_middle[cba042a01728d163]::hir::map::Map>::for_each_module::<rustc_hir_analysis[dc2d927b109b9899]::check_crate::{closure#6}::{closure#0}>
  69:     0x7fb221f63552 - <rustc_session[ead57d6d327013e2]::session::Session>::time::<(), rustc_hir_analysis[dc2d927b109b9899]::check_crate::{closure#6}>
  70:     0x7fb2220d6f01 - rustc_hir_analysis[dc2d927b109b9899]::check_crate
  71:     0x7fb221899491 - rustc_interface[9b825bed216d41b4]::passes::analysis
  72:     0x7fb223123145 - <rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind as rustc_query_system[8b69943028d6b97f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, (), core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  73:     0x7fb22330393e - <rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, (), core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  74:     0x7fb223521315 - rustc_query_system[8b69943028d6b97f]::query::plumbing::try_execute_query::<rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt, rustc_query_system[8b69943028d6b97f]::query::caches::DefaultCache<(), core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>>
  75:     0x7fb22363913f - rustc_query_system[8b69943028d6b97f]::query::plumbing::get_query::<rustc_query_impl[6b04a642019f58b7]::queries::analysis, rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt>
  76:     0x7fb2231da84a - <rustc_query_impl[6b04a642019f58b7]::Queries as rustc_middle[cba042a01728d163]::ty::query::QueryEngine>::analysis
  77:     0x7fb2217a7b2c - <rustc_interface[9b825bed216d41b4]::passes::QueryContext>::enter::<rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  78:     0x7fb2217b1d9f - <rustc_interface[9b825bed216d41b4]::interface::Compiler>::enter::<rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}::{closure#2}, core[2421c901ece542d5]::result::Result<core[2421c901ece542d5]::option::Option<rustc_interface[9b825bed216d41b4]::queries::Linker>, rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  79:     0x7fb22174dec6 - rustc_span[4dd1b2649365341f]::with_source_map::<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  80:     0x7fb2217b2b2a - <scoped_tls[a17af59ae263c11]::ScopedKey<rustc_span[4dd1b2649365341f]::SessionGlobals>>::set::<rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  81:     0x7fb22176c30f - std[2f547644045151aa]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9b825bed216d41b4]::util::run_in_thread_pool_with_globals<rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  82:     0x7fb2217cbd36 - std[2f547644045151aa]::panicking::try::<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, core[2421c901ece542d5]::panic::unwind_safe::AssertUnwindSafe<<std[2f547644045151aa]::thread::Builder>::spawn_unchecked_<rustc_interface[9b825bed216d41b4]::util::run_in_thread_pool_with_globals<rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  83:     0x7fb22175cc65 - <<std[2f547644045151aa]::thread::Builder>::spawn_unchecked_<rustc_interface[9b825bed216d41b4]::util::run_in_thread_pool_with_globals<rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#1} as core[2421c901ece542d5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  84:     0x7fb220d13dbe - std::sys::unix::thread::Thread::new::thread_start::h205ff25f6c397b29
  85:     0x7fb220aa9b43 - <unknown>
  86:     0x7fb220b3ba00 - <unknown>
  87:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (b13a9a36e 2022-11-25) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
thread panicked while processing panic. aborting.
------------------------------------------



---- [ui] src/test/ui/async-await/drop-tracking-unresolved-typeck-results.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-tracking-unresolved-typeck-results.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-tracking-unresolved-typeck-results/drop-tracking-unresolved-typeck-results.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-tracking-unresolved-typeck-results" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-tracking-unresolved-typeck-results/auxiliary" "-Zdrop-tracking" "--edition=2021"
stdout: none
--- stderr -------------------------------
error[E0283]: type annotations needed: cannot satisfy `Self: FnOnce1<A>`
   |
   |
LL | pub trait FnMut1<A>: FnOnce1<A> {
   |
   |
   = note: cannot satisfy `Self: FnOnce1<A>`

thread 'rustc' panicked at 'forcing query with already existing `DepNode`
- query-key: Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [Binder(TraitPredicate(<T as std::marker::Sized>, polarity:Positive), []), Binder(TraitPredicate(<T as std::ops::FnOnce<(A,)>>, polarity:Positive), []), Binder(TraitPredicate(<R as std::marker::Sized>, polarity:Positive), []), Binder(TraitPredicate(<A as std::marker::Sized>, polarity:Positive), []), Binder(ProjectionPredicate(ProjectionTy { substs: [T, (A,)], item_def_id: DefId(2:3695 ~ core[2421]::ops::function::FnOnce::Output) }, Term::Ty(R)), [])], reveal: UserFacing, constness: NotConst }, value: A } }
- dep-node: implied_outlives_bounds(36bf3b78676f1e97-bc5faf4cbc7476e3)', /checkout/compiler/rustc_query_system/src/dep_graph/graph.rs:316:9
   0:     0x7f5f8c319ee5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1a7649ac77790747
   0:     0x7f5f8c319ee5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1a7649ac77790747
   1:     0x7f5f8c389b28 - core::fmt::write::h2987f1da831254ba
   2:     0x7f5f8c30bc91 - std::io::Write::write_fmt::h9134dac7fedc552d
   3:     0x7f5f8c319cf1 - std::sys_common::backtrace::print::h9bbeb4ce4ce0398b
   4:     0x7f5f8c31d034 - std::panicking::default_hook::{{closure}}::ha08f9e0148f6988f
   5:     0x7f5f8c31ccfa - std::panicking::default_hook::h1611fbcdf9dd28c0
   6:     0x7f5f8cd62764 - rustc_driver[466f9bbdde2d424f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f5f8c31d7a4 - std::panicking::rust_panic_with_hook::hb4658a518d8f7f94
   8:     0x7f5f8c31d509 - std::panicking::begin_panic_handler::{{closure}}::hd014742727ccfc65
   9:     0x7f5f8c31a404 - std::sys_common::backtrace::__rust_end_short_backtrace::hf7cef729504cc08a
  10:     0x7f5f8c31d1b2 - rust_begin_unwind
  11:     0x7f5f8c2cdff3 - core::panicking::panic_fmt::hc017567c8322f888
  12:     0x7f5f8e8c60fa - <rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_middle[cba042a01728d163]::infer::canonical::Canonical<rustc_middle[cba042a01728d163]::ty::ParamEnvAnd<rustc_middle[cba042a01728d163]::ty::Ty>>, core[2421c901ece542d5]::result::Result<&rustc_middle[cba042a01728d163]::infer::canonical::Canonical<rustc_middle[cba042a01728d163]::infer::canonical::QueryResponse<alloc[a565b7b4b1676a09]::vec::Vec<rustc_middle[cba042a01728d163]::traits::query::OutlivesBound>>>, rustc_middle[cba042a01728d163]::traits::query::NoSolution>>
  13:     0x7f5f8ec2e275 - rustc_query_system[8b69943028d6b97f]::query::plumbing::get_query::<rustc_query_impl[6b04a642019f58b7]::queries::implied_outlives_bounds, rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt>
  14:     0x7f5f8e8611a6 - <rustc_query_impl[6b04a642019f58b7]::Queries as rustc_middle[cba042a01728d163]::ty::query::QueryEngine>::implied_outlives_bounds
  15:     0x7f5f8f69b5a4 - <rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::implied_outlives_bounds::ImpliedOutlivesBounds as rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::QueryTypeOp>::perform_query
  16:     0x7f5f8f69b79f - <rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::implied_outlives_bounds::ImpliedOutlivesBounds as rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::QueryTypeOp>::fully_perform_into
  17:     0x7f5f8f772528 - <rustc_middle[cba042a01728d163]::ty::ParamEnvAnd<rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::implied_outlives_bounds::ImpliedOutlivesBounds> as rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::TypeOp>::fully_perform
  18:     0x7f5f8f67f228 - <rustc_infer[4839981d34d86096]::infer::InferCtxt as rustc_trait_selection[972a6de06ac5a492]::traits::outlives_bounds::InferCtxtExt>::implied_outlives_bounds
  19:     0x7f5f8d666663 - <&mut <rustc_infer[4839981d34d86096]::infer::InferCtxt as rustc_trait_selection[972a6de06ac5a492]::traits::outlives_bounds::InferCtxtExt>::implied_bounds_tys::{closure#0} as core[2421c901ece542d5]::ops::function::FnOnce<(rustc_middle[cba042a01728d163]::ty::Ty,)>>::call_once
  20:     0x7f5f8d4e5920 - <rustc_infer[4839981d34d86096]::infer::outlives::env::OutlivesEnvironment>::with_bounds::<core[2421c901ece542d5]::iter::adapters::flatten::Flatten<core[2421c901ece542d5]::iter::adapters::map::Map<indexmap[ee6033d77fa8f813]::set::IntoIter<rustc_middle[cba042a01728d163]::ty::Ty>, <rustc_infer[4839981d34d86096]::infer::InferCtxt as rustc_trait_selection[972a6de06ac5a492]::traits::outlives_bounds::InferCtxtExt>::implied_bounds_tys::{closure#0}>>>
  21:     0x7f5f8d643849 - rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_associated_item
  22:     0x7f5f8d631525 - rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_well_formed
  23:     0x7f5f8e730eae - <rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind as rustc_query_system[8b69943028d6b97f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_hir[18e52f59420a28ef]::hir_id::OwnerId, ()>::{closure#0}, ()>
  24:     0x7f5f8e8d486f - <rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_hir[18e52f59420a28ef]::hir_id::OwnerId, ()>
  25:     0x7f5f8eb57adf - rustc_query_system[8b69943028d6b97f]::query::plumbing::try_execute_query::<rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt, rustc_query_system[8b69943028d6b97f]::query::caches::VecCache<rustc_hir[18e52f59420a28ef]::hir_id::OwnerId, ()>>
  26:     0x7f5f8ec10163 - rustc_query_system[8b69943028d6b97f]::query::plumbing::get_query::<rustc_query_impl[6b04a642019f58b7]::queries::check_well_formed, rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt>
  27:     0x7f5f8e843630 - <rustc_query_impl[6b04a642019f58b7]::Queries as rustc_middle[cba042a01728d163]::ty::query::QueryEngine>::check_well_formed
  28:     0x7f5f8d6672e8 - <core[2421c901ece542d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f80276d6338a5d6a]::sync::par_for_each_in<&[rustc_hir[18e52f59420a28ef]::hir::ImplItemId], <rustc_middle[cba042a01728d163]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>::{closure#0}::{closure#0}> as core[2421c901ece542d5]::ops::function::FnOnce<()>>::call_once
  29:     0x7f5f8d58f496 - std[2f547644045151aa]::panic::catch_unwind::<core[2421c901ece542d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f80276d6338a5d6a]::sync::par_for_each_in<&[rustc_hir[18e52f59420a28ef]::hir::ImplItemId], <rustc_middle[cba042a01728d163]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  30:     0x7f5f8d6d356d - rustc_data_structures[f80276d6338a5d6a]::sync::par_for_each_in::<&[rustc_hir[18e52f59420a28ef]::hir::ImplItemId], <rustc_middle[cba042a01728d163]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>
  31:     0x7f5f8d640f51 - rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_mod_type_wf
  32:     0x7f5f8e731e8e - <rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind as rustc_query_system[8b69943028d6b97f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_span[4dd1b2649365341f]::def_id::LocalDefId, ()>::{closure#0}, ()>
  33:     0x7f5f8e8dd26f - <rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_span[4dd1b2649365341f]::def_id::LocalDefId, ()>
  34:     0x7f5f8eb65adf - rustc_query_system[8b69943028d6b97f]::query::plumbing::try_execute_query::<rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt, rustc_query_system[8b69943028d6b97f]::query::caches::VecCache<rustc_span[4dd1b2649365341f]::def_id::LocalDefId, ()>>
  35:     0x7f5f8ec10053 - rustc_query_system[8b69943028d6b97f]::query::plumbing::get_query::<rustc_query_impl[6b04a642019f58b7]::queries::check_mod_type_wf, rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt>
  36:     0x7f5f8e818360 - <rustc_query_impl[6b04a642019f58b7]::Queries as rustc_middle[cba042a01728d163]::ty::query::QueryEngine>::check_mod_type_wf
  37:     0x7f5f8d6679a8 - <core[2421c901ece542d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f80276d6338a5d6a]::sync::par_for_each_in<&[rustc_hir[18e52f59420a28ef]::hir_id::OwnerId], <rustc_middle[cba042a01728d163]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[dc2d927b109b9899]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[2421c901ece542d5]::ops::function::FnOnce<()>>::call_once
  38:     0x7f5f8d58f516 - std[2f547644045151aa]::panic::catch_unwind::<core[2421c901ece542d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f80276d6338a5d6a]::sync::par_for_each_in<&[rustc_hir[18e52f59420a28ef]::hir_id::OwnerId], <rustc_middle[cba042a01728d163]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[dc2d927b109b9899]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  39:     0x7f5f8d6d3b2d - rustc_data_structures[f80276d6338a5d6a]::sync::par_for_each_in::<&[rustc_hir[18e52f59420a28ef]::hir_id::OwnerId], <rustc_middle[cba042a01728d163]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[dc2d927b109b9899]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  40:     0x7f5f8d582a8d - <rustc_session[ead57d6d327013e2]::session::Session>::track_errors::<rustc_hir_analysis[dc2d927b109b9899]::check_crate::{closure#5}, ()>
  41:     0x7f5f8d6ece93 - rustc_hir_analysis[dc2d927b109b9899]::check_crate
  42:     0x7f5f8ceaf491 - rustc_interface[9b825bed216d41b4]::passes::analysis
  43:     0x7f5f8e739145 - <rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind as rustc_query_system[8b69943028d6b97f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, (), core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  44:     0x7f5f8e91993e - <rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, (), core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  45:     0x7f5f8eb37315 - rustc_query_system[8b69943028d6b97f]::query::plumbing::try_execute_query::<rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt, rustc_query_system[8b69943028d6b97f]::query::caches::DefaultCache<(), core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>>
  46:     0x7f5f8ec4f13f - rustc_query_system[8b69943028d6b97f]::query::plumbing::get_query::<rustc_query_impl[6b04a642019f58b7]::queries::analysis, rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt>
  47:     0x7f5f8e7f084a - <rustc_query_impl[6b04a642019f58b7]::Queries as rustc_middle[cba042a01728d163]::ty::query::QueryEngine>::analysis
  48:     0x7f5f8cdbdb2c - <rustc_interface[9b825bed216d41b4]::passes::QueryContext>::enter::<rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  49:     0x7f5f8cdc7d9f - <rustc_interface[9b825bed216d41b4]::interface::Compiler>::enter::<rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}::{closure#2}, core[2421c901ece542d5]::result::Result<core[2421c901ece542d5]::option::Option<rustc_interface[9b825bed216d41b4]::queries::Linker>, rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  50:     0x7f5f8cd63ec6 - rustc_span[4dd1b2649365341f]::with_source_map::<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  51:     0x7f5f8cdc8b2a - <scoped_tls[a17af59ae263c11]::ScopedKey<rustc_span[4dd1b2649365341f]::SessionGlobals>>::set::<rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  52:     0x7f5f8cd8230f - std[2f547644045151aa]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9b825bed216d41b4]::util::run_in_thread_pool_with_globals<rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  53:     0x7f5f8cde1d36 - std[2f547644045151aa]::panicking::try::<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, core[2421c901ece542d5]::panic::unwind_safe::AssertUnwindSafe<<std[2f547644045151aa]::thread::Builder>::spawn_unchecked_<rustc_interface[9b825bed216d41b4]::util::run_in_thread_pool_with_globals<rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  54:     0x7f5f8cd72c65 - <<std[2f547644045151aa]::thread::Builder>::spawn_unchecked_<rustc_interface[9b825bed216d41b4]::util::run_in_thread_pool_with_globals<rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#1} as core[2421c901ece542d5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  55:     0x7f5f8c329dbe - std::sys::unix::thread::Thread::new::thread_start::h205ff25f6c397b29
  56:     0x7f5f8c0bfb43 - <unknown>
  57:     0x7f5f8c151a00 - <unknown>
  58:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (b13a9a36e 2022-11-25) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z drop-tracking
query stack during panic:
query stack during panic:
#0 [implied_outlives_bounds] computing implied outlives bounds for `A`
#1 [check_well_formed] checking that `<impl at /checkout/src/test/ui/async-await/drop-tracking-unresolved-typeck-results.rs:33:1: 33:31>::call_once` is well-formed
#2 [check_mod_type_wf] checking that types are well-formed in top-level module
#3 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'forcing query with already existing `DepNode`
- query-key: Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [Binder(TraitPredicate(<T as std::marker::Sized>, polarity:Positive), []), Binder(TraitPredicate(<T as std::ops::FnMut<(A,)>>, polarity:Positive), []), Binder(TraitPredicate(<T as std::ops::FnOnce<(A,)>>, polarity:Positive), []), Binder(TraitPredicate(<R as std::marker::Sized>, polarity:Positive), []), Binder(TraitPredicate(<A as std::marker::Sized>, polarity:Positive), []), Binder(ProjectionPredicate(ProjectionTy { substs: [T, (A,)], item_def_id: DefId(2:3695 ~ core[2421]::ops::function::FnOnce::Output) }, Term::Ty(R)), [])], reveal: UserFacing, constness: NotConst }, value: A } }
- dep-node: implied_outlives_bounds(2e8efe8f7cd83f96-8924b8d4c7c84b4)', /checkout/compiler/rustc_query_system/src/dep_graph/graph.rs:316:9
   0:     0x7f5f8c319ee5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1a7649ac77790747
   0:     0x7f5f8c319ee5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1a7649ac77790747
   1:     0x7f5f8c389b28 - core::fmt::write::h2987f1da831254ba
   2:     0x7f5f8c30bc91 - std::io::Write::write_fmt::h9134dac7fedc552d
   3:     0x7f5f8c319cf1 - std::sys_common::backtrace::print::h9bbeb4ce4ce0398b
   4:     0x7f5f8c31d034 - std::panicking::default_hook::{{closure}}::ha08f9e0148f6988f
   5:     0x7f5f8c31ccfa - std::panicking::default_hook::h1611fbcdf9dd28c0
   6:     0x7f5f8cd62764 - rustc_driver[466f9bbdde2d424f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f5f8c31d7a4 - std::panicking::rust_panic_with_hook::hb4658a518d8f7f94
   8:     0x7f5f8c31d509 - std::panicking::begin_panic_handler::{{closure}}::hd014742727ccfc65
   9:     0x7f5f8c31a404 - std::sys_common::backtrace::__rust_end_short_backtrace::hf7cef729504cc08a
  10:     0x7f5f8c31d1b2 - rust_begin_unwind
  11:     0x7f5f8c2cdff3 - core::panicking::panic_fmt::hc017567c8322f888
  12:     0x7f5f8e8c60fa - <rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_middle[cba042a01728d163]::infer::canonical::Canonical<rustc_middle[cba042a01728d163]::ty::ParamEnvAnd<rustc_middle[cba042a01728d163]::ty::Ty>>, core[2421c901ece542d5]::result::Result<&rustc_middle[cba042a01728d163]::infer::canonical::Canonical<rustc_middle[cba042a01728d163]::infer::canonical::QueryResponse<alloc[a565b7b4b1676a09]::vec::Vec<rustc_middle[cba042a01728d163]::traits::query::OutlivesBound>>>, rustc_middle[cba042a01728d163]::traits::query::NoSolution>>
  13:     0x7f5f8ec2e275 - rustc_query_system[8b69943028d6b97f]::query::plumbing::get_query::<rustc_query_impl[6b04a642019f58b7]::queries::implied_outlives_bounds, rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt>
  14:     0x7f5f8e8611a6 - <rustc_query_impl[6b04a642019f58b7]::Queries as rustc_middle[cba042a01728d163]::ty::query::QueryEngine>::implied_outlives_bounds
  15:     0x7f5f8f69b5a4 - <rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::implied_outlives_bounds::ImpliedOutlivesBounds as rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::QueryTypeOp>::perform_query
  16:     0x7f5f8f69b79f - <rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::implied_outlives_bounds::ImpliedOutlivesBounds as rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::QueryTypeOp>::fully_perform_into
  17:     0x7f5f8f772528 - <rustc_middle[cba042a01728d163]::ty::ParamEnvAnd<rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::implied_outlives_bounds::ImpliedOutlivesBounds> as rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::TypeOp>::fully_perform
  18:     0x7f5f8f67f228 - <rustc_infer[4839981d34d86096]::infer::InferCtxt as rustc_trait_selection[972a6de06ac5a492]::traits::outlives_bounds::InferCtxtExt>::implied_outlives_bounds
  19:     0x7f5f8d666663 - <&mut <rustc_infer[4839981d34d86096]::infer::InferCtxt as rustc_trait_selection[972a6de06ac5a492]::traits::outlives_bounds::InferCtxtExt>::implied_bounds_tys::{closure#0} as core[2421c901ece542d5]::ops::function::FnOnce<(rustc_middle[cba042a01728d163]::ty::Ty,)>>::call_once
  20:     0x7f5f8d4e5920 - <rustc_infer[4839981d34d86096]::infer::outlives::env::OutlivesEnvironment>::with_bounds::<core[2421c901ece542d5]::iter::adapters::flatten::Flatten<core[2421c901ece542d5]::iter::adapters::map::Map<indexmap[ee6033d77fa8f813]::set::IntoIter<rustc_middle[cba042a01728d163]::ty::Ty>, <rustc_infer[4839981d34d86096]::infer::InferCtxt as rustc_trait_selection[972a6de06ac5a492]::traits::outlives_bounds::InferCtxtExt>::implied_bounds_tys::{closure#0}>>>
  21:     0x7f5f8d643849 - rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_associated_item
  22:     0x7f5f8d631525 - rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_well_formed
  23:     0x7f5f8e730eae - <rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind as rustc_query_system[8b69943028d6b97f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_hir[18e52f59420a28ef]::hir_id::OwnerId, ()>::{closure#0}, ()>
  24:     0x7f5f8e8d486f - <rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_hir[18e52f59420a28ef]::hir_id::OwnerId, ()>
  25:     0x7f5f8eb57adf - rustc_query_system[8b69943028d6b97f]::query::plumbing::try_execute_query::<rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt, rustc_query_system[8b69943028d6b97f]::query::caches::VecCache<rustc_hir[18e52f59420a28ef]::hir_id::OwnerId, ()>>
  26:     0x7f5f8ec10163 - rustc_query_system[8b69943028d6b97f]::query::plumbing::get_query::<rustc_query_impl[6b04a642019f58b7]::queries::check_well_formed, rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt>
  27:     0x7f5f8e843630 - <rustc_query_impl[6b04a642019f58b7]::Queries as rustc_middle[cba042a01728d163]::ty::query::QueryEngine>::check_well_formed
  28:     0x7f5f8d6672e8 - <core[2421c901ece542d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f80276d6338a5d6a]::sync::par_for_each_in<&[rustc_hir[18e52f59420a28ef]::hir::ImplItemId], <rustc_middle[cba042a01728d163]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>::{closure#0}::{closure#0}> as core[2421c901ece542d5]::ops::function::FnOnce<()>>::call_once
  29:     0x7f5f8d58f496 - std[2f547644045151aa]::panic::catch_unwind::<core[2421c901ece542d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f80276d6338a5d6a]::sync::par_for_each_in<&[rustc_hir[18e52f59420a28ef]::hir::ImplItemId], <rustc_middle[cba042a01728d163]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  30:     0x7f5f8d6d356d - rustc_data_structures[f80276d6338a5d6a]::sync::par_for_each_in::<&[rustc_hir[18e52f59420a28ef]::hir::ImplItemId], <rustc_middle[cba042a01728d163]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>
  31:     0x7f5f8d640f51 - rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_mod_type_wf
  32:     0x7f5f8e731e8e - <rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind as rustc_query_system[8b69943028d6b97f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_span[4dd1b2649365341f]::def_id::LocalDefId, ()>::{closure#0}, ()>
  33:     0x7f5f8e8dd26f - <rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_span[4dd1b2649365341f]::def_id::LocalDefId, ()>
  34:     0x7f5f8eb65adf - rustc_query_system[8b69943028d6b97f]::query::plumbing::try_execute_query::<rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt, rustc_query_system[8b69943028d6b97f]::query::caches::VecCache<rustc_span[4dd1b2649365341f]::def_id::LocalDefId, ()>>
  35:     0x7f5f8ec10053 - rustc_query_system[8b69943028d6b97f]::query::plumbing::get_query::<rustc_query_impl[6b04a642019f58b7]::queries::check_mod_type_wf, rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt>
  36:     0x7f5f8e818360 - <rustc_query_impl[6b04a642019f58b7]::Queries as rustc_middle[cba042a01728d163]::ty::query::QueryEngine>::check_mod_type_wf
  37:     0x7f5f8d6679a8 - <core[2421c901ece542d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f80276d6338a5d6a]::sync::par_for_each_in<&[rustc_hir[18e52f59420a28ef]::hir_id::OwnerId], <rustc_middle[cba042a01728d163]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[dc2d927b109b9899]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[2421c901ece542d5]::ops::function::FnOnce<()>>::call_once
  38:     0x7f5f8d58f516 - std[2f547644045151aa]::panic::catch_unwind::<core[2421c901ece542d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f80276d6338a5d6a]::sync::par_for_each_in<&[rustc_hir[18e52f59420a28ef]::hir_id::OwnerId], <rustc_middle[cba042a01728d163]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[dc2d927b109b9899]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  39:     0x7f5f8d6d3b2d - rustc_data_structures[f80276d6338a5d6a]::sync::par_for_each_in::<&[rustc_hir[18e52f59420a28ef]::hir_id::OwnerId], <rustc_middle[cba042a01728d163]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[dc2d927b109b9899]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  40:     0x7f5f8d582a8d - <rustc_session[ead57d6d327013e2]::session::Session>::track_errors::<rustc_hir_analysis[dc2d927b109b9899]::check_crate::{closure#5}, ()>
  41:     0x7f5f8d6ece93 - rustc_hir_analysis[dc2d927b109b9899]::check_crate
  42:     0x7f5f8ceaf491 - rustc_interface[9b825bed216d41b4]::passes::analysis
  43:     0x7f5f8e739145 - <rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind as rustc_query_system[8b69943028d6b97f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, (), core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  44:     0x7f5f8e91993e - <rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, (), core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  45:     0x7f5f8eb37315 - rustc_query_system[8b69943028d6b97f]::query::plumbing::try_execute_query::<rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt, rustc_query_system[8b69943028d6b97f]::query::caches::DefaultCache<(), core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>>
  46:     0x7f5f8ec4f13f - rustc_query_system[8b69943028d6b97f]::query::plumbing::get_query::<rustc_query_impl[6b04a642019f58b7]::queries::analysis, rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt>
  47:     0x7f5f8e7f084a - <rustc_query_impl[6b04a642019f58b7]::Queries as rustc_middle[cba042a01728d163]::ty::query::QueryEngine>::analysis
  48:     0x7f5f8cdbdb2c - <rustc_interface[9b825bed216d41b4]::passes::QueryContext>::enter::<rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  49:     0x7f5f8cdc7d9f - <rustc_interface[9b825bed216d41b4]::interface::Compiler>::enter::<rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}::{closure#2}, core[2421c901ece542d5]::result::Result<core[2421c901ece542d5]::option::Option<rustc_interface[9b825bed216d41b4]::queries::Linker>, rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  50:     0x7f5f8cd63ec6 - rustc_span[4dd1b2649365341f]::with_source_map::<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  51:     0x7f5f8cdc8b2a - <scoped_tls[a17af59ae263c11]::ScopedKey<rustc_span[4dd1b2649365341f]::SessionGlobals>>::set::<rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  52:     0x7f5f8cd8230f - std[2f547644045151aa]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9b825bed216d41b4]::util::run_in_thread_pool_with_globals<rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  53:     0x7f5f8cde1d36 - std[2f547644045151aa]::panicking::try::<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, core[2421c901ece542d5]::panic::unwind_safe::AssertUnwindSafe<<std[2f547644045151aa]::thread::Builder>::spawn_unchecked_<rustc_interface[9b825bed216d41b4]::util::run_in_thread_pool_with_globals<rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  54:     0x7f5f8cd72c65 - <<std[2f547644045151aa]::thread::Builder>::spawn_unchecked_<rustc_interface[9b825bed216d41b4]::util::run_in_thread_pool_with_globals<rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#1} as core[2421c901ece542d5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  55:     0x7f5f8c329dbe - std::sys::unix::thread::Thread::new::thread_start::h205ff25f6c397b29
  56:     0x7f5f8c0bfb43 - <unknown>
  57:     0x7f5f8c151a00 - <unknown>
  58:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (b13a9a36e 2022-11-25) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z drop-tracking
query stack during panic:
query stack during panic:
#0 [implied_outlives_bounds] computing implied outlives bounds for `A`
#1 [check_well_formed] checking that `<impl at /checkout/src/test/ui/async-await/drop-tracking-unresolved-typeck-results.rs:47:1: 47:30>::call_mut` is well-formed
#2 [check_mod_type_wf] checking that types are well-formed in top-level module
#3 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'could not lift for printing', compiler/rustc_middle/src/ty/print/pretty.rs:2542:1
   0:     0x7f5f8c319ee5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1a7649ac77790747
   0:     0x7f5f8c319ee5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1a7649ac77790747
   1:     0x7f5f8c389b28 - core::fmt::write::h2987f1da831254ba
   2:     0x7f5f8c30bc91 - std::io::Write::write_fmt::h9134dac7fedc552d
   3:     0x7f5f8c319cf1 - std::sys_common::backtrace::print::h9bbeb4ce4ce0398b
   4:     0x7f5f8c31d034 - std::panicking::default_hook::{{closure}}::ha08f9e0148f6988f
   5:     0x7f5f8c31ccfa - std::panicking::default_hook::h1611fbcdf9dd28c0
   6:     0x7f5f8cd62764 - rustc_driver[466f9bbdde2d424f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f5f8c31d7a4 - std::panicking::rust_panic_with_hook::hb4658a518d8f7f94
   8:     0x7f5f8c31d509 - std::panicking::begin_panic_handler::{{closure}}::hd014742727ccfc65
   9:     0x7f5f8c31a404 - std::sys_common::backtrace::__rust_end_short_backtrace::hf7cef729504cc08a
  10:     0x7f5f8c31d1b2 - rust_begin_unwind
  11:     0x7f5f8c2cdff3 - core::panicking::panic_fmt::hc017567c8322f888
  12:     0x7f5f8c386311 - core::panicking::panic_display::hb0348ae40115c924
  13:     0x7f5f8c3862bb - core::panicking::panic_str::h9deed821c280dc1c
  14:     0x7f5f8c2cdfb6 - core::option::expect_failed::ha18499e3b0d5a4f5
  15:     0x7f5f8fa0c58c - <rustc_middle[cba042a01728d163]::ty::Ty as core[2421c901ece542d5]::fmt::Display>::fmt
  16:     0x7f5f8c389b28 - core::fmt::write::h2987f1da831254ba
  17:     0x7f5f8fb8525a - <rustc_middle[cba042a01728d163]::ty::print::pretty::FmtPrinter as core[2421c901ece542d5]::fmt::Write>::write_fmt
  18:     0x7f5f8fb4d1a5 - <rustc_middle[cba042a01728d163]::ty::sty::TraitRef as rustc_middle[cba042a01728d163]::ty::print::Print<rustc_middle[cba042a01728d163]::ty::print::pretty::FmtPrinter>>::print
  19:     0x7f5f8fb4db2e - <rustc_middle[cba042a01728d163]::ty::sty::TraitRef as core[2421c901ece542d5]::fmt::Display>::fmt
  20:     0x7f5f8fb41566 - <rustc_middle[cba042a01728d163]::ty::sty::TraitRef as core[2421c901ece542d5]::fmt::Debug>::fmt
  21:     0x7f5f8c389b28 - core::fmt::write::h2987f1da831254ba
  22:     0x7f5f8c38aac3 - core::fmt::Formatter::write_fmt::hb1a8980ba87e6e48
  23:     0x7f5f8fa059fd - <rustc_middle[cba042a01728d163]::ty::PredicateKind as core[2421c901ece542d5]::fmt::Debug>::fmt
  24:     0x7f5f8c386b30 - core::fmt::builders::DebugTuple::field::hdb9654b4a89e4676
  25:     0x7f5f8c38b2b5 - core::fmt::Formatter::debug_tuple_field2_finish::hec12b9e69f1117f9
  26:     0x7f5f8fb4fd06 - <rustc_middle[cba042a01728d163]::ty::sty::Binder<rustc_middle[cba042a01728d163]::ty::PredicateKind> as core[2421c901ece542d5]::fmt::Debug>::fmt
  27:     0x7f5f8c389b28 - core::fmt::write::h2987f1da831254ba
  28:     0x7f5f8c38aac3 - core::fmt::Formatter::write_fmt::hb1a8980ba87e6e48
  29:     0x7f5f8f9fe029 - <&rustc_middle[cba042a01728d163]::ty::Predicate as core[2421c901ece542d5]::fmt::Debug>::fmt
  30:     0x7f5f8c386ce8 - core::fmt::builders::DebugInner::entry::h07815c3ffe0fcc3c
  31:     0x7f5f8c386da9 - core::fmt::builders::DebugSet::entry::h7ad70c19ae56adfa
  32:     0x7f5f8fb577e3 - <core[2421c901ece542d5]::fmt::builders::DebugList>::entries::<&rustc_middle[cba042a01728d163]::ty::Predicate, core[2421c901ece542d5]::slice::iter::Iter<rustc_middle[cba042a01728d163]::ty::Predicate>>
  33:     0x7f5f8f9feacd - <[rustc_middle[cba042a01728d163]::ty::Predicate] as core[2421c901ece542d5]::fmt::Debug>::fmt
  34:     0x7f5f8c3868ac - core::fmt::builders::DebugStruct::field::ha752fa0c7987e9bc
  35:     0x7f5f8fa0a5e5 - <rustc_middle[cba042a01728d163]::ty::ParamEnv as core[2421c901ece542d5]::fmt::Debug>::fmt
  36:     0x7f5f8c3868ac - core::fmt::builders::DebugStruct::field::ha752fa0c7987e9bc
  37:     0x7f5f8c38aca4 - core::fmt::Formatter::debug_struct_field2_finish::h9d4571940b1c39d3
  38:     0x7f5f8ea15829 - <&rustc_middle[cba042a01728d163]::ty::ParamEnvAnd<rustc_middle[cba042a01728d163]::ty::Ty> as core[2421c901ece542d5]::fmt::Debug>::fmt
  39:     0x7f5f8c3868ac - core::fmt::builders::DebugStruct::field::ha752fa0c7987e9bc
  40:     0x7f5f8c38adac - core::fmt::Formatter::debug_struct_field3_finish::hb172eace3c23e018
  41:     0x7f5f8e7ccc90 - <rustc_middle[cba042a01728d163]::infer::canonical::Canonical<rustc_middle[cba042a01728d163]::ty::ParamEnvAnd<rustc_middle[cba042a01728d163]::ty::Ty>> as core[2421c901ece542d5]::fmt::Debug>::fmt
  42:     0x7f5f8c389b28 - core::fmt::write::h2987f1da831254ba
  43:     0x7f5f8c31d389 - <std::panicking::begin_panic_handler::PanicPayload as core::panic::BoxMeUp>::get::h924f4b1c45865160
  44:     0x7f5f8c31d78a - std::panicking::rust_panic_with_hook::hb4658a518d8f7f94
  45:     0x7f5f8c31d509 - std::panicking::begin_panic_handler::{{closure}}::hd014742727ccfc65
  46:     0x7f5f8c31a404 - std::sys_common::backtrace::__rust_end_short_backtrace::hf7cef729504cc08a
  47:     0x7f5f8c31d1b2 - rust_begin_unwind
  48:     0x7f5f8c2cdff3 - core::panicking::panic_fmt::hc017567c8322f888
  49:     0x7f5f8e8c60fa - <rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_middle[cba042a01728d163]::infer::canonical::Canonical<rustc_middle[cba042a01728d163]::ty::ParamEnvAnd<rustc_middle[cba042a01728d163]::ty::Ty>>, core[2421c901ece542d5]::result::Result<&rustc_middle[cba042a01728d163]::infer::canonical::Canonical<rustc_middle[cba042a01728d163]::infer::canonical::QueryResponse<alloc[a565b7b4b1676a09]::vec::Vec<rustc_middle[cba042a01728d163]::traits::query::OutlivesBound>>>, rustc_middle[cba042a01728d163]::traits::query::NoSolution>>
  50:     0x7f5f8ec2e275 - rustc_query_system[8b69943028d6b97f]::query::plumbing::get_query::<rustc_query_impl[6b04a642019f58b7]::queries::implied_outlives_bounds, rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt>
  51:     0x7f5f8e8611a6 - <rustc_query_impl[6b04a642019f58b7]::Queries as rustc_middle[cba042a01728d163]::ty::query::QueryEngine>::implied_outlives_bounds
  52:     0x7f5f8f69b5a4 - <rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::implied_outlives_bounds::ImpliedOutlivesBounds as rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::QueryTypeOp>::perform_query
  53:     0x7f5f8f69b79f - <rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::implied_outlives_bounds::ImpliedOutlivesBounds as rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::QueryTypeOp>::fully_perform_into
  54:     0x7f5f8f772528 - <rustc_middle[cba042a01728d163]::ty::ParamEnvAnd<rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::implied_outlives_bounds::ImpliedOutlivesBounds> as rustc_trait_selection[972a6de06ac5a492]::traits::query::type_op::TypeOp>::fully_perform
  55:     0x7f5f8f67f228 - <rustc_infer[4839981d34d86096]::infer::InferCtxt as rustc_trait_selection[972a6de06ac5a492]::traits::outlives_bounds::InferCtxtExt>::implied_outlives_bounds
  56:     0x7f5f8d666663 - <&mut <rustc_infer[4839981d34d86096]::infer::InferCtxt as rustc_trait_selection[972a6de06ac5a492]::traits::outlives_bounds::InferCtxtExt>::implied_bounds_tys::{closure#0} as core[2421c901ece542d5]::ops::function::FnOnce<(rustc_middle[cba042a01728d163]::ty::Ty,)>>::call_once
  57:     0x7f5f8d4e5920 - <rustc_infer[4839981d34d86096]::infer::outlives::env::OutlivesEnvironment>::with_bounds::<core[2421c901ece542d5]::iter::adapters::flatten::Flatten<core[2421c901ece542d5]::iter::adapters::map::Map<indexmap[ee6033d77fa8f813]::set::IntoIter<rustc_middle[cba042a01728d163]::ty::Ty>, <rustc_infer[4839981d34d86096]::infer::InferCtxt as rustc_trait_selection[972a6de06ac5a492]::traits::outlives_bounds::InferCtxtExt>::implied_bounds_tys::{closure#0}>>>
  58:     0x7f5f8d643849 - rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_associated_item
  59:     0x7f5f8d631525 - rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_well_formed
  60:     0x7f5f8e730eae - <rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind as rustc_query_system[8b69943028d6b97f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_hir[18e52f59420a28ef]::hir_id::OwnerId, ()>::{closure#0}, ()>
  61:     0x7f5f8e8d486f - <rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_hir[18e52f59420a28ef]::hir_id::OwnerId, ()>
  62:     0x7f5f8eb57adf - rustc_query_system[8b69943028d6b97f]::query::plumbing::try_execute_query::<rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt, rustc_query_system[8b69943028d6b97f]::query::caches::VecCache<rustc_hir[18e52f59420a28ef]::hir_id::OwnerId, ()>>
  63:     0x7f5f8ec10163 - rustc_query_system[8b69943028d6b97f]::query::plumbing::get_query::<rustc_query_impl[6b04a642019f58b7]::queries::check_well_formed, rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt>
  64:     0x7f5f8e843630 - <rustc_query_impl[6b04a642019f58b7]::Queries as rustc_middle[cba042a01728d163]::ty::query::QueryEngine>::check_well_formed
  65:     0x7f5f8d6672e8 - <core[2421c901ece542d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f80276d6338a5d6a]::sync::par_for_each_in<&[rustc_hir[18e52f59420a28ef]::hir::ImplItemId], <rustc_middle[cba042a01728d163]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>::{closure#0}::{closure#0}> as core[2421c901ece542d5]::ops::function::FnOnce<()>>::call_once
  66:     0x7f5f8d58f496 - std[2f547644045151aa]::panic::catch_unwind::<core[2421c901ece542d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f80276d6338a5d6a]::sync::par_for_each_in<&[rustc_hir[18e52f59420a28ef]::hir::ImplItemId], <rustc_middle[cba042a01728d163]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  67:     0x7f5f8d6d356d - rustc_data_structures[f80276d6338a5d6a]::sync::par_for_each_in::<&[rustc_hir[18e52f59420a28ef]::hir::ImplItemId], <rustc_middle[cba042a01728d163]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>
  68:     0x7f5f8d640f51 - rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_mod_type_wf
  69:     0x7f5f8e731e8e - <rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind as rustc_query_system[8b69943028d6b97f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_span[4dd1b2649365341f]::def_id::LocalDefId, ()>::{closure#0}, ()>
  70:     0x7f5f8e8dd26f - <rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_span[4dd1b2649365341f]::def_id::LocalDefId, ()>
  71:     0x7f5f8eb65adf - rustc_query_system[8b69943028d6b97f]::query::plumbing::try_execute_query::<rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt, rustc_query_system[8b69943028d6b97f]::query::caches::VecCache<rustc_span[4dd1b2649365341f]::def_id::LocalDefId, ()>>
  72:     0x7f5f8ec10053 - rustc_query_system[8b69943028d6b97f]::query::plumbing::get_query::<rustc_query_impl[6b04a642019f58b7]::queries::check_mod_type_wf, rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt>
  73:     0x7f5f8e818360 - <rustc_query_impl[6b04a642019f58b7]::Queries as rustc_middle[cba042a01728d163]::ty::query::QueryEngine>::check_mod_type_wf
  74:     0x7f5f8d6679a8 - <core[2421c901ece542d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f80276d6338a5d6a]::sync::par_for_each_in<&[rustc_hir[18e52f59420a28ef]::hir_id::OwnerId], <rustc_middle[cba042a01728d163]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[dc2d927b109b9899]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[2421c901ece542d5]::ops::function::FnOnce<()>>::call_once
  75:     0x7f5f8d58f516 - std[2f547644045151aa]::panic::catch_unwind::<core[2421c901ece542d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f80276d6338a5d6a]::sync::par_for_each_in<&[rustc_hir[18e52f59420a28ef]::hir_id::OwnerId], <rustc_middle[cba042a01728d163]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[dc2d927b109b9899]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  76:     0x7f5f8d6d3b2d - rustc_data_structures[f80276d6338a5d6a]::sync::par_for_each_in::<&[rustc_hir[18e52f59420a28ef]::hir_id::OwnerId], <rustc_middle[cba042a01728d163]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[dc2d927b109b9899]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  77:     0x7f5f8d582a8d - <rustc_session[ead57d6d327013e2]::session::Session>::track_errors::<rustc_hir_analysis[dc2d927b109b9899]::check_crate::{closure#5}, ()>
  78:     0x7f5f8d6ece93 - rustc_hir_analysis[dc2d927b109b9899]::check_crate
  79:     0x7f5f8ceaf491 - rustc_interface[9b825bed216d41b4]::passes::analysis
  80:     0x7f5f8e739145 - <rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind as rustc_query_system[8b69943028d6b97f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, (), core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  81:     0x7f5f8e91993e - <rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, (), core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  82:     0x7f5f8eb37315 - rustc_query_system[8b69943028d6b97f]::query::plumbing::try_execute_query::<rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt, rustc_query_system[8b69943028d6b97f]::query::caches::DefaultCache<(), core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>>
  83:     0x7f5f8ec4f13f - rustc_query_system[8b69943028d6b97f]::query::plumbing::get_query::<rustc_query_impl[6b04a642019f58b7]::queries::analysis, rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt>
  84:     0x7f5f8e7f084a - <rustc_query_impl[6b04a642019f58b7]::Queries as rustc_middle[cba042a01728d163]::ty::query::QueryEngine>::analysis
  85:     0x7f5f8cdbdb2c - <rustc_interface[9b825bed216d41b4]::passes::QueryContext>::enter::<rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  86:     0x7f5f8cdc7d9f - <rustc_interface[9b825bed216d41b4]::interface::Compiler>::enter::<rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}::{closure#2}, core[2421c901ece542d5]::result::Result<core[2421c901ece542d5]::option::Option<rustc_interface[9b825bed216d41b4]::queries::Linker>, rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  87:     0x7f5f8cd63ec6 - rustc_span[4dd1b2649365341f]::with_source_map::<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  88:     0x7f5f8cdc8b2a - <scoped_tls[a17af59ae263c11]::ScopedKey<rustc_span[4dd1b2649365341f]::SessionGlobals>>::set::<rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  89:     0x7f5f8cd8230f - std[2f547644045151aa]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9b825bed216d41b4]::util::run_in_thread_pool_with_globals<rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>
  90:     0x7f5f8cde1d36 - std[2f547644045151aa]::panicking::try::<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, core[2421c901ece542d5]::panic::unwind_safe::AssertUnwindSafe<<std[2f547644045151aa]::thread::Builder>::spawn_unchecked_<rustc_interface[9b825bed216d41b4]::util::run_in_thread_pool_with_globals<rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  91:     0x7f5f8cd72c65 - <<std[2f547644045151aa]::thread::Builder>::spawn_unchecked_<rustc_interface[9b825bed216d41b4]::util::run_in_thread_pool_with_globals<rustc_interface[9b825bed216d41b4]::interface::run_compiler<core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>, rustc_driver[466f9bbdde2d424f]::run_compiler::{closure#1}>::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2421c901ece542d5]::result::Result<(), rustc_errors[77c6fcb1cd076939]::ErrorGuaranteed>>::{closure#1} as core[2421c901ece542d5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  92:     0x7f5f8c329dbe - std::sys::unix::thread::Thread::new::thread_start::h205ff25f6c397b29
  93:     0x7f5f8c0bfb43 - <unknown>
  94:     0x7f5f8c151a00 - <unknown>
  95:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (b13a9a36e 2022-11-25) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z drop-tracking
query stack during panic:
query stack during panic:
#0 [implied_outlives_bounds] computing implied outlives bounds for `Buffered<St>`
#1 [check_well_formed] checking that `<impl at /checkout/src/test/ui/async-await/drop-tracking-unresolved-typeck-results.rs:76:1: 76:33>::Item` is well-formed
#2 [check_mod_type_wf] checking that types are well-formed in top-level module
#3 [analysis] running analysis passes on this crate
thread panicked while panicking. aborting.
------------------------------------------



---- [ui] src/test/ui/async-await/issue-72442.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-72442.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-72442/issue-72442.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-72442" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-72442/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'could not lift for printing', compiler/rustc_middle/src/ty/print/pretty.rs:2542:1
   0:     0x7f2447c95ee5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1a7649ac77790747
   0:     0x7f2447c95ee5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1a7649ac77790747
   1:     0x7f2447d05b28 - core::fmt::write::h2987f1da831254ba
   2:     0x7f2447c87c91 - std::io::Write::write_fmt::h9134dac7fedc552d
   3:     0x7f2447c95cf1 - std::sys_common::backtrace::print::h9bbeb4ce4ce0398b
   4:     0x7f2447c99034 - std::panicking::default_hook::{{closure}}::ha08f9e0148f6988f
   5:     0x7f2447c98cfa - std::panicking::default_hook::h1611fbcdf9dd28c0
   6:     0x7f24486de764 - rustc_driver[466f9bbdde2d424f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f2447c997a4 - std::panicking::rust_panic_with_hook::hb4658a518d8f7f94
   8:     0x7f2447c99509 - std::panicking::begin_panic_handler::{{closure}}::hd014742727ccfc65
   9:     0x7f2447c96404 - std::sys_common::backtrace::__rust_end_short_backtrace::hf7cef729504cc08a
  10:     0x7f2447c991b2 - rust_begin_unwind
  11:     0x7f2447c49ff3 - core::panicking::panic_fmt::hc017567c8322f888
  12:     0x7f2447d02311 - core::panicking::panic_display::hb0348ae40115c924
  13:     0x7f2447d022bb - core::panicking::panic_str::h9deed821c280dc1c
  14:     0x7f2447c49fb6 - core::option::expect_failed::ha18499e3b0d5a4f5
  15:     0x7f244b38858c - <rustc_middle[cba042a01728d163]::ty::Ty as core[2421c901ece542d5]::fmt::Display>::fmt
  16:     0x7f2447d05b28 - core::fmt::write::h2987f1da831254ba
  17:     0x7f244b50125a - <rustc_middle[cba042a01728d163]::ty::print::pretty::FmtPrinter as core[2421c901ece542d5]::fmt::Write>::write_fmt
  18:     0x7f244b4c91a5 - <rustc_middle[cba042a01728d163]::ty::sty::TraitRef as rustc_middle[cba042a01728d163]::ty::print::Print<rustc_middle[cba042a01728d163]::ty::print::pretty::FmtPrinter>>::print
  19:     0x7f244b4c9b2e - <rustc_middle[cba042a01728d163]::ty::sty::TraitRef as core[2421c901ece542d5]::fmt::Display>::fmt
  20:     0x7f244b4bd566 - <rustc_middle[cba042a01728d163]::ty::sty::TraitRef as core[2421c901ece542d5]::fmt::Debug>::fmt
  21:     0x7f2447d05b28 - core::fmt::write::h2987f1da831254ba
  22:     0x7f2447d06ac3 - core::fmt::Formatter::write_fmt::hb1a8980ba87e6e48
  23:     0x7f244b3819fd - <rustc_middle[cba042a01728d163]::ty::PredicateKind as core[2421c901ece542d5]::fmt::Debug>::fmt
  24:     0x7f2447d02b30 - core::fmt::builders::DebugTuple::field::hdb9654b4a89e4676
  25:     0x7f2447d072b5 - core::fmt::Formatter::debug_tuple_field2_finish::hec12b9e69f1117f9
  26:     0x7f244b4cbd06 - <rustc_middle[cba042a01728d163]::ty::sty::Binder<rustc_middle[cba042a01728d163]::ty::PredicateKind> as core[2421c901ece542d5]::fmt::Debug>::fmt
  27:     0x7f2447d05b28 - core::fmt::write::h2987f1da831254ba
  28:     0x7f2447d06ac3 - core::fmt::Formatter::write_fmt::hb1a8980ba87e6e48
  29:     0x7f244b37a029 - <&rustc_middle[cba042a01728d163]::ty::Predicate as core[2421c901ece542d5]::fmt::Debug>::fmt
  30:     0x7f2447d02ce8 - core::fmt::builders::DebugInner::entry::h07815c3ffe0fcc3c
  31:     0x7f2447d02da9 - core::fmt::builders::DebugSet::entry::h7ad70c19ae56adfa
  32:     0x7f244b4d37e3 - <core[2421c901ece542d5]::fmt::builders::DebugList>::entries::<&rustc_middle[cba042a01728d163]::ty::Predicate, core[2421c901ece542d5]::slice::iter::Iter<rustc_middle[cba042a01728d163]::ty::Predicate>>
  33:     0x7f244b37aacd - <[rustc_middle[cba042a01728d163]::ty::Predicate] as core[2421c901ece542d5]::fmt::Debug>::fmt
  34:     0x7f2447d028ac - core::fmt::builders::DebugStruct::field::ha752fa0c7987e9bc
  35:     0x7f244b3865e5 - <rustc_middle[cba042a01728d163]::ty::ParamEnv as core[2421c901ece542d5]::fmt::Debug>::fmt
  36:     0x7f2447d028ac - core::fmt::builders::DebugStruct::field::ha752fa0c7987e9bc
  37:     0x7f2447d06ca4 - core::fmt::Formatter::debug_struct_field2_finish::h9d4571940b1c39d3
  38:     0x7f244a3918a9 - <&rustc_middle[cba042a01728d163]::ty::ParamEnvAnd<rustc_middle[cba042a01728d163]::ty::Predicate> as core[2421c901ece542d5]::fmt::Debug>::fmt
  39:     0x7f2447d028ac - core::fmt::builders::DebugStruct::field::ha752fa0c7987e9bc
  40:     0x7f2447d06dac - core::fmt::Formatter::debug_struct_field3_finish::hb172eace3c23e018
  41:     0x7f244a148d30 - <rustc_middle[cba042a01728d163]::infer::canonical::Canonical<rustc_middle[cba042a01728d163]::ty::ParamEnvAnd<rustc_middle[cba042a01728d163]::ty::Predicate>> as core[2421c901ece542d5]::fmt::Debug>::fmt
  42:     0x7f2447d05b28 - core::fmt::write::h2987f1da831254ba
  43:     0x7f2447c99389 - <std::panicking::begin_panic_handler::PanicPayload as core::panic::BoxMeUp>::get::h924f4b1c45865160
  44:     0x7f2447c9978a - std::panicking::rust_panic_with_hook::hb4658a518d8f7f94
  45:     0x7f2447c99509 - std::panicking::begin_panic_handler::{{closure}}::hd014742727ccfc65
  46:     0x7f2447c96404 - std::sys_common::backtrace::__rust_end_short_backtrace::hf7cef729504cc08a
  47:     0x7f2447c991b2 - rust_begin_unwind
  48:     0x7f2447c49ff3 - core::panicking::panic_fmt::hc017567c8322f888
  49:     0x7f244a2438da - <rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_middle[cba042a01728d163]::infer::canonical::Canonical<rustc_middle[cba042a01728d163]::ty::ParamEnvAnd<rustc_middle[cba042a01728d163]::ty::Predicate>>, core[2421c901ece542d5]::result::Result<rustc_middle[cba042a01728d163]::traits::select::EvaluationResult, rustc_middle[cba042a01728d163]::traits::select::OverflowError>>
  50:     0x7f244a59443c - rustc_query_system[8b69943028d6b97f]::query::plumbing::get_query::<rustc_query_impl[6b04a642019f58b7]::queries::evaluate_obligation, rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt>
  51:     0x7f244a1de0c8 - <rustc_query_impl[6b04a642019f58b7]::Queries as rustc_middle[cba042a01728d163]::ty::query::QueryEngine>::evaluate_obligation
  52:     0x7f244aff8648 - <rustc_infer[4839981d34d86096]::infer::InferCtxt as rustc_trait_selection[972a6de06ac5a492]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  53:     0x7f244aff87dc - <rustc_infer[4839981d34d86096]::infer::InferCtxt as rustc_trait_selection[972a6de06ac5a492]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  54:     0x7f244b05530c - rustc_trait_selection[972a6de06ac5a492]::traits::object_safety::receiver_is_dispatchable
  55:     0x7f244b05300e - rustc_trait_selection[972a6de06ac5a492]::traits::object_safety::virtual_call_violation_for_method
  56:     0x7f244b051e18 - rustc_trait_selection[972a6de06ac5a492]::traits::object_safety::object_safety_violation_for_method
  57:     0x7f244af313e0 - <core[2421c901ece542d5]::iter::adapters::map::Map<core[2421c901ece542d5]::iter::adapters::map::Map<core[2421c901ece542d5]::slice::iter::Iter<(rustc_span[4dd1b2649365341f]::symbol::Symbol, &rustc_middle[cba042a01728d163]::ty::assoc::AssocItem)>, <rustc_data_structures[f80276d6338a5d6a]::sorted_map::index_map::SortedIndexMultiMap<u32, rustc_span[4dd1b2649365341f]::symbol::Symbol, &rustc_middle[cba042a01728d163]::ty::assoc::AssocItem>>::iter::{closure#0}>, <rustc_middle[cba042a01728d163]::ty::assoc::AssocItems>::in_definition_order::{closure#0}> as core[2421c901ece542d5]::iter::traits::iterator::Iterator>::try_fold::<(), core[2421c901ece542d5]::iter::adapters::filter::filter_try_fold<&rustc_middle[cba042a01728d163]::ty::assoc::AssocItem, (), core[2421c901ece542d5]::ops::control_flow::ControlFlow<rustc_middle[cba042a01728d163]::traits::ObjectSafetyViolation>, rustc_trait_selection[972a6de06ac5a492]::traits::object_safety::object_safety_violations_for_trait::{closure#0}, core[2421c901ece542d5]::iter::adapters::filter_map::filter_map_try_fold<&rustc_middle[cba042a01728d163]::ty::assoc::AssocItem, rustc_middle[cba042a01728d163]::traits::ObjectSafetyViolation, (), core[2421c901ece542d5]::ops::control_flow::ControlFlow<rustc_middle[cba042a01728d163]::traits::ObjectSafetyViolation>, rustc_trait_selection[972a6de06ac5a492]::traits::object_safety::object_safety_violations_for_trait::{closure#1}, core[2421c901ece542d5]::iter::traits::iterator::Iterator::find::check<rustc_middle[cba042a01728d163]::traits::ObjectSafetyViolation, &mut rustc_trait_selection[972a6de06ac5a492]::traits::object_safety::object_safety_violations_for_trait::{closure#2}>::{closure#0}>::{closure#0}>::{closure#0}, core[2421c901ece542d5]::ops::control_flow::ControlFlow<rustc_middle[cba042a01728d163]::traits::ObjectSafetyViolation>>
  58:     0x7f244afb5cd3 - <alloc[a565b7b4b1676a09]::vec::Vec<rustc_middle[cba042a01728d163]::traits::ObjectSafetyViolation> as alloc[a565b7b4b1676a09]::vec::spec_from_iter::SpecFromIter<rustc_middle[cba042a01728d163]::traits::ObjectSafetyViolation, core[2421c901ece542d5]::iter::adapters::filter::Filter<core[2421c901ece542d5]::iter::adapters::filter_map::FilterMap<core[2421c901ece542d5]::iter::adapters::filter::Filter<core[2421c901ece542d5]::iter::adapters::map::Map<core[2421c901ece542d5]::iter::adapters::map::Map<core[2421c901ece542d5]::slice::iter::Iter<(rustc_span[4dd1b2649365341f]::symbol::Symbol, &rustc_middle[cba042a01728d163]::ty::assoc::AssocItem)>, <rustc_data_structures[f80276d6338a5d6a]::sorted_map::index_map::SortedIndexMultiMap<u32, rustc_span[4dd1b2649365341f]::symbol::Symbol, &rustc_middle[cba042a01728d163]::ty::assoc::AssocItem>>::iter::{closure#0}>, <rustc_middle[cba042a01728d163]::ty::assoc::AssocItems>::in_definition_order::{closure#0}>, rustc_trait_selection[972a6de06ac5a492]::traits::object_safety::object_safety_violations_for_trait::{closure#0}>, rustc_trait_selection[972a6de06ac5a492]::traits::object_safety::object_safety_violations_for_trait::{closure#1}>, rustc_trait_selection[972a6de06ac5a492]::traits::object_safety::object_safety_violations_for_trait::{closure#2}>>>::from_iter
  59:     0x7f244b0449d4 - <&mut rustc_trait_selection[972a6de06ac5a492]::traits::object_safety::object_safety_violations::{closure#0} as core[2421c901ece542d5]::ops::function::FnOnce<(rustc_span[4dd1b2649365341f]::def_id::DefId,)>>::call_once
  60:     0x7f244afdaaf0 - <core[2421c901ece542d5]::iter::adapters::flatten::FlatMap<rustc_trait_selection[972a6de06ac5a492]::traits::util::SupertraitDefIds, alloc[a565b7b4b1676a09]::vec::Vec<rustc_middle[cba042a01728d163]::traits::ObjectSafetyViolation>, rustc_trait_selection[972a6de06ac5a492]::traits::object_safety::object_safety_violations::{closure#0}> as core[2421c901ece542d5]::iter::traits::iterator::Iterator>::next
  61:     0x7f244afd1e52 - <smallvec[2e0c523ab931d2d0]::SmallVec<[rustc_middle[cba042a01728d163]::traits::ObjectSafetyViolation; 8usize]> as core[2421c901ece542d5]::iter::traits::collect::Extend<rustc_middle[cba042a01728d163]::traits::ObjectSafetyViolation>>::extend::<core[2421c901ece542d5]::iter::adapters::flatten::FlatMap<rustc_trait_selection[972a6de06ac5a492]::traits::util::SupertraitDefIds, alloc[a565b7b4b1676a09]::vec::Vec<rustc_middle[cba042a01728d163]::traits::ObjectSafetyViolation>, rustc_trait_selection[972a6de06ac5a492]::traits::object_safety::object_safety_violations::{closure#0}>>
  62:     0x7f244af75d90 - <rustc_middle[cba042a01728d163]::arena::Arena>::alloc_from_iter::<rustc_middle[cba042a01728d163]::traits::ObjectSafetyViolation, rustc_arena[8a477e222018055c]::IsNotCopy, core[2421c901ece542d5]::iter::adapters::flatten::FlatMap<rustc_trait_selection[972a6de06ac5a492]::traits::util::SupertraitDefIds, alloc[a565b7b4b1676a09]::vec::Vec<rustc_middle[cba042a01728d163]::traits::ObjectSafetyViolation>, rustc_trait_selection[972a6de06ac5a492]::traits::object_safety::object_safety_violations::{closure#0}>>
  63:     0x7f244b050c30 - rustc_trait_selection[972a6de06ac5a492]::traits::object_safety::object_safety_violations
  64:     0x7f244a0b172d - <rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind as rustc_query_system[8b69943028d6b97f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_span[4dd1b2649365341f]::def_id::DefId, &[rustc_middle[cba042a01728d163]::traits::ObjectSafetyViolation]>::{closure#0}, &[rustc_middle[cba042a01728d163]::traits::ObjectSafetyViolation]>
  65:     0x7f244a276897 - <rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_span[4dd1b2649365341f]::def_id::DefId, &[rustc_middle[cba042a01728d163]::traits::ObjectSafetyViolation]>
  66:     0x7f244a4aa8e5 - rustc_query_system[8b69943028d6b97f]::query::plumbing::try_execute_query::<rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt, rustc_query_system[8b69943028d6b97f]::query::caches::DefaultCache<rustc_span[4dd1b2649365341f]::def_id::DefId, &[rustc_middle[cba042a01728d163]::traits::ObjectSafetyViolation]>>
  67:     0x7f244a5b2eff - rustc_query_system[8b69943028d6b97f]::query::plumbing::get_query::<rustc_query_impl[6b04a642019f58b7]::queries::object_safety_violations, rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt>
  68:     0x7f244a1b0693 - <rustc_query_impl[6b04a642019f58b7]::Queries as rustc_middle[cba042a01728d163]::ty::query::QueryEngine>::object_safety_violations
  69:     0x7f244b4620fd - <rustc_middle[cba042a01728d163]::ty::context::TyCtxt>::is_object_safe
  70:     0x7f244b11f3a7 - <rustc_trait_selection[972a6de06ac5a492]::traits::fulfill::FulfillProcessor as rustc_data_structures[f80276d6338a5d6a]::obligation_forest::ObligationProcessor>::process_obligation
  71:     0x7f244affcd7c - <rustc_data_structures[f80276d6338a5d6a]::obligation_forest::ObligationForest<rustc_trait_selection[972a6de06ac5a492]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[972a6de06ac5a492]::traits::fulfill::FulfillProcessor>
  72:     0x7f244b11ad17 - <rustc_trait_selection[972a6de06ac5a492]::traits::fulfill::FulfillmentContext as rustc_infer[4839981d34d86096]::traits::engine::TraitEngine>::select_where_possible
  73:     0x7f244b11abc6 - <rustc_trait_selection[972a6de06ac5a492]::traits::fulfill::FulfillmentContext as rustc_infer[4839981d34d86096]::traits::engine::TraitEngine>::select_all_or_error
  74:     0x7f244b017006 - <rustc_trait_selection[972a6de06ac5a492]::traits::engine::ObligationCtxt>::select_all_or_error
  75:     0x7f2448fb9b42 - rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_item_fn
  76:     0x7f2448fad787 - rustc_hir_analysis[dc2d927b109b9899]::check::wfcheck::check_well_formed
  77:     0x7f244a0aceae - <rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind as rustc_query_system[8b69943028d6b97f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_hir[18e52f59420a28ef]::hir_id::OwnerId, ()>::{closure#0}, ()>
  78:     0x7f244a25086f - <rustc_query_system[8b69943028d6b97f]::dep_graph::graph::DepGraph<rustc_middle[cba042a01728d163]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cba042a01728d163]::ty::context::TyCtxt, rustc_hir[18e52f59420a28ef]::hir_id::OwnerId, ()>
  79:     0x7f244a4d3adf - rustc_query_system[8b69943028d6b97f]::query::plumbing::try_execute_query::<rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt, rustc_query_system[8b69943028d6b97f]::query::caches::VecCache<rustc_hir[18e52f59420a28ef]::hir_id::OwnerId, ()>>
  80:     0x7f244a58c163 - rustc_query_system[8b69943028d6b97f]::query::plumbing::get_query::<rustc_query_impl[6b04a642019f58b7]::queries::check_well_formed, rustc_query_impl[6b04a642019f58b7]::plumbing::QueryCtxt>
  81:     0x7f244a1bf630 - <rustc_query_impl[6b04a642019f58b7]::Queries as rustc_middle[cba042a01728d163]::ty::query::QueryEngine>::check_well_formed
