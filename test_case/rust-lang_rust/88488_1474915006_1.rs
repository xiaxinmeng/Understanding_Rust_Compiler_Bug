
thread 'rustc' panicked at 'no entry found for key', compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs:639:9
stack backtrace:
   0: rust_begin_unwind
             at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:579:5
   1: core::panicking::panic_fmt
             at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/core/src/panicking.rs:64:14
   2: core::panicking::panic_display
             at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/core/src/panicking.rs:147:5
   3: core::panicking::panic_str
             at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/core/src/panicking.rs:131:5
   4: core::option::expect_failed
             at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/core/src/option.rs:2045:5
   5: <rustc_metadata::creader::CStore as rustc_session::cstore::CrateStore>::stable_crate_id_to_crate_num
   6: <rustc_span::span_encoding::Span as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   7: <[(rustc_middle::ty::Predicate, rustc_span::span_encoding::Span)] as rustc_middle::ty::codec::RefDecodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   8: rustc_query_impl::plumbing::try_load_from_disk::<rustc_middle::ty::generics::GenericPredicates>
   9: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::predicates_of, rustc_query_impl::plumbing::QueryCtxt>
  10: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::predicates_of
  11: <rustc_trait_selection::traits::wf::WfPredicates>::compute
  12: rustc_trait_selection::traits::wf::obligations
  13: rustc_traits::implied_outlives_bounds::compute_implied_outlives_bounds
  14: <rustc_infer::infer::InferCtxtBuilder as rustc_trait_selection::infer::InferCtxtBuilderExt>::enter_canonical_trait_query::<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Ty>, alloc::vec::Vec<rustc_middle::traits::query::OutlivesBound>, rustc_traits::implied_outlives_bounds::implied_outlives_bounds::{closure#0}>
  15: rustc_traits::implied_outlives_bounds::implied_outlives_bounds
  16: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::implied_outlives_bounds, rustc_query_impl::plumbing::QueryCtxt>
  17: <rustc_middle::ty::ParamEnvAnd<rustc_trait_selection::traits::query::type_op::implied_outlives_bounds::ImpliedOutlivesBounds> as rustc_trait_selection::traits::query::type_op::TypeOp>::fully_perform
  18: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::outlives_bounds::InferCtxtExt>::implied_outlives_bounds
  19: <rustc_infer::infer::outlives::env::OutlivesEnvironment>::with_bounds::<core::iter::adapters::flatten::Flatten<core::iter::adapters::map::Map<indexmap::set::IntoIter<rustc_middle::ty::Ty>, <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::outlives_bounds::InferCtxtExt>::implied_bounds_tys::{closure#0}>>>
  20: rustc_trait_selection::traits::misc::type_allowed_to_implement_copy
  21: rustc_hir_analysis::coherence::builtin::check_trait
  22: rustc_hir_analysis::coherence::coherent_trait
  23: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::coherent_trait, rustc_query_impl::plumbing::QueryCtxt>
  24: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::coherent_trait
  25: <rustc_session::session::Session>::track_errors::<rustc_hir_analysis::check_crate::{closure#3}, ()>
  26: rustc_hir_analysis::check_crate
  27: rustc_interface::passes::analysis
  28: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  29: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  30: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
  31: rustc_span::with_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-beta.1 (b955c8271 2023-03-06) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C incremental=[REDACTED] -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z tls-model=initial-exec

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
thread 'rustc' panicked at 'Illegal read of: 48076', /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/compiler/rustc_query_system/src/dep_graph/graph.rs:450:25
stack backtrace:
   0:     0x7feb20b6731a - std::backtrace_rs::backtrace::libunwind::trace::h6afeafcf92b09bcd
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7feb20b6731a - std::backtrace_rs::backtrace::trace_unsynchronized::h6f2afab8efd008d2
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7feb20b6731a - std::sys_common::backtrace::_print_fmt::hf6f0f010fea6e3fd
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7feb20b6731a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c4abbc84b2347e3
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7feb20bcab6e - core::fmt::write::h7604b6ac1e51564a
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/core/src/fmt/mod.rs:1232:17
   5:     0x7feb20b5a1c5 - std::io::Write::write_fmt::h06952c4611dbdea8
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/io/mod.rs:1684:15
   6:     0x7feb20b670e5 - std::sys_common::backtrace::_print::h5a8a2578f0a5a262
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7feb20b670e5 - std::sys_common::backtrace::print::h9cd3257ab29a9457
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7feb20b69e5f - std::panicking::default_hook::{{closure}}::h2efc46d4738c4264
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:271:22
   9:     0x7feb20b69b9b - std::panicking::default_hook::hcbff7ff0fc72f592
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:290:9
  10:     0x7feb23dc56b5 - rustc_driver_impl[b2d0c9d39cec3f08]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7feb20b6a69d - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hc722c46abd4bee52
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/alloc/src/boxed.rs:2001:9
  12:     0x7feb20b6a69d - std::panicking::rust_panic_with_hook::h2a4f4becb061dab8
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:696:13
  13:     0x7feb20b6a419 - std::panicking::begin_panic_handler::{{closure}}::hfbfd1835b102f703
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:583:13
  14:     0x7feb20b67786 - std::sys_common::backtrace::__rust_end_short_backtrace::hff723b81c54b7ea4
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys_common/backtrace.rs:150:18
  15:     0x7feb20b6a122 - rust_begin_unwind
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:579:5
  16:     0x7feb20bc6ec3 - core::panicking::panic_fmt::hf07572e75eddd3b0
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/core/src/panicking.rs:64:14
  17:     0x7feb21e1f4d7 - <rustc_middle[5904c30a6dccf498]::dep_graph::dep_node::DepKind as rustc_query_system[2a4cc30c318fa14d]::dep_graph::DepKind>::read_deps::<<rustc_query_system[2a4cc30c318fa14d]::dep_graph::graph::DepGraph<rustc_middle[5904c30a6dccf498]::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
  18:     0x7feb21f438e3 - <rustc_middle[5904c30a6dccf498]::ty::print::pretty::FmtPrinter>::new
  19:     0x7feb22bd9594 - <rustc_middle[5904c30a6dccf498]::ty::context::TyCtxt>::def_path_str_with_substs
  20:     0x7feb24342ae7 - rustc_middle[5904c30a6dccf498]::query::descs::predicates_defined_on
  21:     0x7feb247716c4 - rustc_query_impl[f5702d406d4b30ad]::plumbing::create_query_frame::<rustc_span[5cba7a76607c212]::def_id::DefId>
  22:     0x7feb247dc316 - <rustc_query_impl[f5702d406d4b30ad]::query_structs::predicates_of::{closure#0}::{closure#0} as core[b162ae3dcbce6770]::ops::function::FnOnce<(rustc_query_impl[f5702d406d4b30ad]::plumbing::QueryCtxt, rustc_span[5cba7a76607c212]::def_id::DefId)>>::call_once
  23:     0x7feb24670620 - <rustc_query_system[2a4cc30c318fa14d]::query::plumbing::QueryState<rustc_span[5cba7a76607c212]::def_id::DefId, rustc_middle[5904c30a6dccf498]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[f5702d406d4b30ad]::plumbing::QueryCtxt>
  24:     0x7feb23583252 - <rustc_query_impl[f5702d406d4b30ad]::Queries>::try_collect_active_jobs
  25:     0x7feb247848c8 - rustc_query_system[2a4cc30c318fa14d]::query::job::print_query_stack::<rustc_query_impl[f5702d406d4b30ad]::plumbing::QueryCtxt>
  26:     0x7feb24181db2 - rustc_interface[4988ae148fb02a7]::interface::try_print_query_stack
  27:     0x7feb23dc7a8e - rustc_driver_impl[b2d0c9d39cec3f08]::report_ice
  28:     0x7feb23dc56fc - rustc_driver_impl[b2d0c9d39cec3f08]::DEFAULT_HOOK::{closure#0}::{closure#0}
  29:     0x7feb20b6a69d - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hc722c46abd4bee52
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/alloc/src/boxed.rs:2001:9
  30:     0x7feb20b6a69d - std::panicking::rust_panic_with_hook::h2a4f4becb061dab8
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:696:13
  31:     0x7feb20b6a419 - std::panicking::begin_panic_handler::{{closure}}::hfbfd1835b102f703
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:583:13
  32:     0x7feb20b67786 - std::sys_common::backtrace::__rust_end_short_backtrace::hff723b81c54b7ea4
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys_common/backtrace.rs:150:18
  33:     0x7feb20b6a122 - rust_begin_unwind
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:579:5
  34:     0x7feb20bc6ec3 - core::panicking::panic_fmt::hf07572e75eddd3b0
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/core/src/panicking.rs:64:14
  35:     0x7feb20bc7031 - core::panicking::panic_display::h8e1404409969f9d1
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/core/src/panicking.rs:147:5
  36:     0x7feb20bc6fdb - core::panicking::panic_str::hf8d20f13eebdbd01
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/core/src/panicking.rs:131:5
  37:     0x7feb20bc6c46 - core::option::expect_failed::h41323982955abec7
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/core/src/option.rs:2045:5
  38:     0x7feb2362a3bc - <rustc_metadata[d8e59722a835656e]::creader::CStore as rustc_session[7ad75fa329ac590f]::cstore::CrateStore>::stable_crate_id_to_crate_num
  39:     0x7feb222a8f02 - <rustc_span[5cba7a76607c212]::span_encoding::Span as rustc_serialize[3fbcda3f3a7af78b]::serialize::Decodable<rustc_query_impl[f5702d406d4b30ad]::on_disk_cache::CacheDecoder>>::decode
  40:     0x7feb223b3180 - <[(rustc_middle[5904c30a6dccf498]::ty::Predicate, rustc_span[5cba7a76607c212]::span_encoding::Span)] as rustc_middle[5904c30a6dccf498]::ty::codec::RefDecodable<rustc_query_impl[f5702d406d4b30ad]::on_disk_cache::CacheDecoder>>::decode
  41:     0x7feb22096d7a - rustc_query_impl[f5702d406d4b30ad]::plumbing::try_load_from_disk::<rustc_middle[5904c30a6dccf498]::ty::generics::GenericPredicates>
  42:     0x7feb22aaa432 - rustc_query_system[2a4cc30c318fa14d]::query::plumbing::try_execute_query::<rustc_query_impl[f5702d406d4b30ad]::queries::predicates_of, rustc_query_impl[f5702d406d4b30ad]::plumbing::QueryCtxt>
  43:     0x7feb22aa8583 - <rustc_query_impl[f5702d406d4b30ad]::Queries as rustc_middle[5904c30a6dccf498]::ty::query::QueryEngine>::predicates_of
  44:     0x7feb224ae65a - <rustc_trait_selection[50b3ba705be25864]::traits::wf::WfPredicates>::compute
  45:     0x7feb224a4981 - rustc_trait_selection[50b3ba705be25864]::traits::wf::obligations
  46:     0x7feb22531109 - rustc_traits[85830f365481b2ff]::implied_outlives_bounds::compute_implied_outlives_bounds
  47:     0x7feb22530610 - <rustc_infer[9e89ecae57d94074]::infer::InferCtxtBuilder as rustc_trait_selection[50b3ba705be25864]::infer::InferCtxtBuilderExt>::enter_canonical_trait_query::<rustc_middle[5904c30a6dccf498]::ty::ParamEnvAnd<rustc_middle[5904c30a6dccf498]::ty::Ty>, alloc[19bf5bcf77274cd1]::vec::Vec<rustc_middle[5904c30a6dccf498]::traits::query::OutlivesBound>, rustc_traits[85830f365481b2ff]::implied_outlives_bounds::implied_outlives_bounds::{closure#0}>
  48:     0x7feb2252ffd1 - rustc_traits[85830f365481b2ff]::implied_outlives_bounds::implied_outlives_bounds
  49:     0x7feb22f9c48c - rustc_query_system[2a4cc30c318fa14d]::query::plumbing::try_execute_query::<rustc_query_impl[f5702d406d4b30ad]::queries::implied_outlives_bounds, rustc_query_impl[f5702d406d4b30ad]::plumbing::QueryCtxt>
  50:     0x7feb225a09d7 - <rustc_middle[5904c30a6dccf498]::ty::ParamEnvAnd<rustc_trait_selection[50b3ba705be25864]::traits::query::type_op::implied_outlives_bounds::ImpliedOutlivesBounds> as rustc_trait_selection[50b3ba705be25864]::traits::query::type_op::TypeOp>::fully_perform
  51:     0x7feb225a0287 - <rustc_infer[9e89ecae57d94074]::infer::InferCtxt as rustc_trait_selection[50b3ba705be25864]::traits::outlives_bounds::InferCtxtExt>::implied_outlives_bounds
  52:     0x7feb22da4854 - <rustc_infer[9e89ecae57d94074]::infer::outlives::env::OutlivesEnvironment>::with_bounds::<core[b162ae3dcbce6770]::iter::adapters::flatten::Flatten<core[b162ae3dcbce6770]::iter::adapters::map::Map<indexmap[a0edb2780134eb61]::set::IntoIter<rustc_middle[5904c30a6dccf498]::ty::Ty>, <rustc_infer[9e89ecae57d94074]::infer::InferCtxt as rustc_trait_selection[50b3ba705be25864]::traits::outlives_bounds::InferCtxtExt>::implied_bounds_tys::{closure#0}>>>
  53:     0x7feb22da3887 - rustc_trait_selection[50b3ba705be25864]::traits::misc::type_allowed_to_implement_copy
  54:     0x7feb232387bd - rustc_hir_analysis[d0511122a2951bb0]::coherence::builtin::check_trait
  55:     0x7feb23237004 - rustc_hir_analysis[d0511122a2951bb0]::coherence::coherent_trait
  56:     0x7feb233e4a47 - rustc_query_system[2a4cc30c318fa14d]::query::plumbing::try_execute_query::<rustc_query_impl[f5702d406d4b30ad]::queries::coherent_trait, rustc_query_impl[f5702d406d4b30ad]::plumbing::QueryCtxt>
  57:     0x7feb233e4364 - <rustc_query_impl[f5702d406d4b30ad]::Queries as rustc_middle[5904c30a6dccf498]::ty::query::QueryEngine>::coherent_trait
  58:     0x7feb22184d25 - <rustc_session[7ad75fa329ac590f]::session::Session>::track_errors::<rustc_hir_analysis[d0511122a2951bb0]::check_crate::{closure#3}, ()>
  59:     0x7feb2218122f - rustc_hir_analysis[d0511122a2951bb0]::check_crate
  60:     0x7feb22177212 - rustc_interface[4988ae148fb02a7]::passes::analysis
  61:     0x7feb23628fb8 - rustc_query_system[2a4cc30c318fa14d]::query::plumbing::try_execute_query::<rustc_query_impl[f5702d406d4b30ad]::queries::analysis, rustc_query_impl[f5702d406d4b30ad]::plumbing::QueryCtxt>
  62:     0x7feb23628a8f - <rustc_query_impl[f5702d406d4b30ad]::Queries as rustc_middle[5904c30a6dccf498]::ty::query::QueryEngine>::analysis
  63:     0x7feb2344d736 - <rustc_middle[5904c30a6dccf498]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[b2d0c9d39cec3f08]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>>
  64:     0x7feb23028958 - rustc_span[5cba7a76607c212]::with_source_map::<core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>, rustc_interface[4988ae148fb02a7]::interface::run_compiler<core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>, rustc_driver_impl[b2d0c9d39cec3f08]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  65:     0x7feb2301ffcc - std[7f3793225a10bff4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4988ae148fb02a7]::util::run_in_thread_pool_with_globals<rustc_interface[4988ae148fb02a7]::interface::run_compiler<core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>, rustc_driver_impl[b2d0c9d39cec3f08]::run_compiler::{closure#1}>::{closure#0}, core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>>
  66:     0x7feb2301f9fa - <<std[7f3793225a10bff4]::thread::Builder>::spawn_unchecked_<rustc_interface[4988ae148fb02a7]::util::run_in_thread_pool_with_globals<rustc_interface[4988ae148fb02a7]::interface::run_compiler<core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>, rustc_driver_impl[b2d0c9d39cec3f08]::run_compiler::{closure#1}>::{closure#0}, core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>>::{closure#1} as core[b162ae3dcbce6770]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  67:     0x7feb20b74593 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h168eb1356e410ac7
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/alloc/src/boxed.rs:1987:9
  68:     0x7feb20b74593 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hc48e839c56707dd0
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/alloc/src/boxed.rs:1987:9
  69:     0x7feb20b74593 - std::sys::unix::thread::Thread::new::thread_start::hdf98ced2fabec86a
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys/unix/thread.rs:108:17
  70:     0x7feb20894b43 - start_thread
                               at ./nptl/./nptl/pthread_create.c:442:8
  71:     0x7feb20926a00 - clone3
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
  72:                0x0 - <unknown>

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-beta.1 (b955c8271 2023-03-06) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C incremental=[REDACTED] -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z tls-model=initial-exec

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
thread panicked while processing panic. aborting.
rustc exited with signal: 6 (SIGABRT) (core dumped)
error: could not compile `rustdoc`

Caused by:
  process didn't exit successfully: `/home/jyn/src/rust2/build/bootstrap/debug/rustc --crate-name rustdoc --edition=2021 src/librustdoc/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=120 --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Zunstable-options --check-cfg 'values(feature, "jemalloc")' --check-cfg 'names()' --check-cfg 'values()' -C metadata=3cb46ca83be16bef -C extra-filename=-3cb46ca83be16bef --out-dir /home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/incremental -L dependency=/home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/release/deps --extern arrayvec=/home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libarrayvec-5e9627a3109f0d7e.rmeta --extern askama=/home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libaskama-8b002f7060c34d88.rmeta --extern itertools=/home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-5bf35759fe845bc7.rmeta --extern minifier=/home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libminifier-750c0ec807f93480.rmeta --extern once_cell=/home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libonce_cell-f442e0474c4f356c.rmeta --extern regex=/home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libregex-b482e273c41bd4d4.rmeta --extern rustdoc_json_types=/home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/librustdoc_json_types-d0cbc30edfc92ac3.rmeta --extern serde=/home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libserde-95319950f910f65b.rmeta --extern serde_json=/home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-72a654be3d4c271d.rmeta --extern smallvec=/home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libsmallvec-8c6a2634519a9954.rmeta --extern tempfile=/home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libtempfile-2009f02edece3d02.rmeta --extern threadpool=/home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libthreadpool-4baa3a0e7ee4df2e.rmeta --extern tracing=/home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libtracing-444062ff7940d597.rmeta --extern tracing_subscriber=/home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_subscriber-6104546c150854a4.rmeta --extern tracing_tree=/home/jyn/src/rust2/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_tree-a5e5396dd9308678.rmeta --cfg=bootstrap -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' '--check-cfg=values(emulate_second_only_system)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Csplit-debuginfo=off -Cllvm-args=-import-instr-limit=10 -Z binary-dep-depinfo` (exit status: 254)
