
 Documenting rustc_infer v0.0.0 (/home/infrandomness/Documents/Dev/rust/compiler/rustc_infer)
thread 'rustc' panicked at 'no entry found for key', compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs:564:9
stack backtrace:
   0:     0x7fd57b09dfed - std::backtrace_rs::backtrace::libunwind::trace::ha8406fd4a233ad1f
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fd57b09dfed - std::backtrace_rs::backtrace::trace_unsynchronized::h95f26ca4bd23bd0f
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fd57b09dfed - std::sys_common::backtrace::_print_fmt::h9955f0b025c061cd
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7fd57b09dfed - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1025ecd2bac85833
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7fd57b0f9c7c - core::fmt::write::hd5fa34bae8a7f0ea
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/core/src/fmt/mod.rs:1196:17
   5:     0x7fd57b08f6c1 - std::io::Write::write_fmt::h203cec764662c27d
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/io/mod.rs:1654:15
   6:     0x7fd57b0a0d05 - std::sys_common::backtrace::_print::h330c8fadbab13d8b
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7fd57b0a0d05 - std::sys_common::backtrace::print::h38d4cf1c3e16e656
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7fd57b0a0d05 - std::panicking::default_hook::{{closure}}::h56cdf194ec124abb
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/panicking.rs:295:22
   9:     0x7fd57b0a0979 - std::panicking::default_hook::h9929fdc60733f795
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/panicking.rs:314:9
  10:     0x7fd57b8ea7e1 - rustc_driver[25b442836e214e3a]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fd567f1e789 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hba67a6b74d57afdc
 Documenting rustc_incremental v0.0.0 (/home/infrandomness/Documents/Dev/rust/compiler/rustc_incremental)
  12:     0x7fd567f254ab - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::h65019d64185dfc24
  13:     0x7fd57b0a14d6 - std::panicking::rust_panic_with_hook::h490228be5a55104d
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/panicking.rs:702:17
  14:     0x7fd57b0a12d7 - std::panicking::begin_panic_handler::{{closure}}::hfd963b6683c36656
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/panicking.rs:588:13
  15:     0x7fd57b09e4a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h2912e3decfe2cba4
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/sys_common/backtrace.rs:138:18
  16:     0x7fd57b0a1009 - rust_begin_unwind
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/panicking.rs:584:5
  17:     0x7fd57b066283 - core::panicking::panic_fmt::hf4ba906a2c8693a2
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/core/src/panicking.rs:142:14
  18:     0x7fd57b0f68e1 - core::panicking::panic_display::h85c49ea53beff92d
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/core/src/panicking.rs:72:5
  19:     0x7fd57b0f688b - core::panicking::panic_str::h54aad4367eb7a446
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/core/src/panicking.rs:56:5
  20:     0x7fd57b0660f6 - core::option::expect_failed::h1c20531357267673
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/core/src/option.rs:1854:5
  21:     0x7fd57d49c2ea - <rustc_metadata[861e9325ed4d7b3f]::creader::CStore as rustc_session[42681064e675acd6]::cstore::CrateStore>::stable_crate_id_to_crate_num
  22:     0x7fd57d3a52e1 - <rustc_span[9389a2ea4399dbf9]::span_encoding::Span as rustc_serialize[d140f8edb489d2a6]::serialize::Decodable<rustc_query_impl[d28b29c5c0f26ce3]::on_disk_cache::CacheDecoder>>::decode
  23:     0x7fd57d337ef1 - <[(rustc_middle[b283510e1dd05d73]::ty::Predicate, rustc_span[9389a2ea4399dbf9]::span_encoding::Span)] as rustc_middle[b283510e1dd05d73]::ty::codec::RefDecodable<rustc_query_impl[d28b29c5c0f26ce3]::on_disk_cache::CacheDecoder>>::decode
  24:     0x7fd57d341bc5 - <rustc_query_impl[d28b29c5c0f26ce3]::on_disk_cache::OnDiskCache>::try_load_query_result::<rustc_middle[b283510e1dd05d73]::ty::generics::GenericPredicates>
  25:     0x7fd57d416da0 - <<rustc_query_impl[d28b29c5c0f26ce3]::queries::predicates_of as rustc_query_system[d6f3c753593960fb]::query::config::QueryDescription<rustc_query_impl[d28b29c5c0f26ce3]::plumbing::QueryCtxt>>::TRY_LOAD_FROM_DISK::{closure#0} as core[4a8947a89ffd1b77]::ops::function::FnOnce<(rustc_query_impl[d28b29c5c0f26ce3]::plumbing::QueryCtxt, rustc_query_system[d6f3c753593960fb]::dep_graph::serialized::SerializedDepNodeIndex)>>::call_once
  26:     0x7fd57d3f000e - rustc_query_system[d6f3c753593960fb]::query::plumbing::get_query::<rustc_query_impl[d28b29c5c0f26ce3]::queries::predicates_of, rustc_query_impl[d28b29c5c0f26ce3]::plumbing::QueryCtxt>
  27:     0x7fd57d390f2e - <rustc_query_impl[d28b29c5c0f26ce3]::Queries as rustc_middle[b283510e1dd05d73]::ty::query::QueryEngine>::predicates_of
  28:     0x7fd57cc8f0d9 - <rustc_privacy[c9a24284be258ba6]::ReachEverythingInTheInterfaceVisitor>::predicates
  29:     0x7fd57cc8b076 - <rustc_privacy[c9a24284be258ba6]::EmbargoVisitor as rustc_hir[374f189791373ac]::intravisit::Visitor>::visit_item
  30:     0x7fd57cc8ccfb - <rustc_privacy[c9a24284be258ba6]::EmbargoVisitor as rustc_hir[374f189791373ac]::intravisit::Visitor>::visit_item
  31:     0x7fd57daf43fb - rustc_privacy[c9a24284be258ba6]::privacy_access_levels
  32:     0x7fd57de49ed4 - <rustc_query_system[d6f3c753593960fb]::dep_graph::graph::DepGraph<rustc_middle[b283510e1dd05d73]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b283510e1dd05d73]::ty::context::TyCtxt, (), &rustc_middle[b283510e1dd05d73]::middle::privacy::AccessLevels>
  33:     0x7fd57def20f1 - rustc_query_system[d6f3c753593960fb]::query::plumbing::try_execute_query::<rustc_query_impl[d28b29c5c0f26ce3]::plumbing::QueryCtxt, rustc_query_system[d6f3c753593960fb]::query::caches::DefaultCache<(), &rustc_middle[b283510e1dd05d73]::middle::privacy::AccessLevels>>
  34:     0x7fd57df2040f - rustc_query_system[d6f3c753593960fb]::query::plumbing::get_query::<rustc_query_impl[d28b29c5c0f26ce3]::queries::privacy_access_levels, rustc_query_impl[d28b29c5c0f26ce3]::plumbing::QueryCtxt>
  35:     0x7fd57dcd90e4 - rustc_passes[c56600e865d3cc74]::stability::check_unused_or_stable_features
  36:     0x7fd57d99ffd6 - <rustc_session[42681064e675acd6]::session::Session>::time::<(), rustc_interface[1214bec19156c976]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
  37:     0x7fd57d9a0f2e - <rustc_session[42681064e675acd6]::session::Session>::time::<(), rustc_interface[1214bec19156c976]::passes::analysis::{closure#0}>
  38:     0x7fd57d99ba9e - rustc_interface[1214bec19156c976]::passes::analysis
  39:     0x7fd57de46a75 - <rustc_query_system[d6f3c753593960fb]::dep_graph::graph::DepGraph<rustc_middle[b283510e1dd05d73]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b283510e1dd05d73]::ty::context::TyCtxt, (), core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>>
  40:     0x7fd57deec181 - rustc_query_system[d6f3c753593960fb]::query::plumbing::try_execute_query::<rustc_query_impl[d28b29c5c0f26ce3]::plumbing::QueryCtxt, rustc_query_system[d6f3c753593960fb]::query::caches::DefaultCache<(), core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>>>
  41:     0x7fd57df2d49e - rustc_query_system[d6f3c753593960fb]::query::plumbing::get_query::<rustc_query_impl[d28b29c5c0f26ce3]::queries::analysis, rustc_query_impl[d28b29c5c0f26ce3]::plumbing::QueryCtxt>
  42:     0x7fd57d97c527 - <rustc_interface[1214bec19156c976]::passes::QueryContext>::enter::<rustc_driver[25b442836e214e3a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>>
  43:     0x7fd57d967588 - <rustc_interface[1214bec19156c976]::interface::Compiler>::enter::<rustc_driver[25b442836e214e3a]::run_compiler::{closure#1}::{closure#2}, core[4a8947a89ffd1b77]::result::Result<core[4a8947a89ffd1b77]::option::Option<rustc_interface[1214bec19156c976]::queries::Linker>, rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>>
  44:     0x7fd57d98fc7f - rustc_span[9389a2ea4399dbf9]::with_source_map::<core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>, rustc_interface[1214bec19156c976]::interface::create_compiler_and_run<core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>, rustc_driver[25b442836e214e3a]::run_compiler::{closure#1}>::{closure#1}>
  45:     0x7fd57d9682f4 - rustc_interface[1214bec19156c976]::interface::create_compiler_and_run::<core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>, rustc_driver[25b442836e214e3a]::run_compiler::{closure#1}>
  46:     0x7fd57d965a92 - <scoped_tls[fdc467f1f45e3091]::ScopedKey<rustc_span[9389a2ea4399dbf9]::SessionGlobals>>::set::<rustc_interface[1214bec19156c976]::interface::run_compiler<core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>, rustc_driver[25b442836e214e3a]::run_compiler::{closure#1}>::{closure#0}, core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>>
  47:     0x7fd57d97cbaf - std[49987cacb63ffdeb]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1214bec19156c976]::util::run_in_thread_pool_with_globals<rustc_interface[1214bec19156c976]::interface::run_compiler<core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>, rustc_driver[25b442836e214e3a]::run_compiler::{closure#1}>::{closure#0}, core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>>::{closure#0}, core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>>
  48:     0x7fd57d97cce9 - <<std[49987cacb63ffdeb]::thread::Builder>::spawn_unchecked_<rustc_interface[1214bec19156c976]::util::run_in_thread_pool_with_globals<rustc_interface[1214bec19156c976]::interface::run_compiler<core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>, rustc_driver[25b442836e214e3a]::run_compiler::{closure#1}>::{closure#0}, core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>>::{closure#0}, core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>>::{closure#1} as core[4a8947a89ffd1b77]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  49:     0x7fd57b0ab3f3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h997dc5bc9efdb91e
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/alloc/src/boxed.rs:1872:9
  50:     0x7fd57b0ab3f3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h482cb6422469721b
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/alloc/src/boxed.rs:1872:9
  51:     0x7fd57b0ab3f3 - std::sys::unix::thread::Thread::new::thread_start::hcd1011649240eef0
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/sys/unix/thread.rs:108:17
  52:     0x7fd57aeb0e8d - start_thread
  53:     0x7fd57af31350 - __clone3
  54:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-beta.2 (daf68b1f7 2022-05-19) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C incremental -C symbol-mangling-version=v0 -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z force-unstable-if-unmarked

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
thread 'rustc' panicked at 'Illegal read of: 67397', /rustc/daf68b1f766e67ffe040260b15c218301853386a/compiler/rustc_query_system/src/dep_graph/graph.rs:436:25
stack backtrace:
   0:     0x7fd57b09dfed - std::backtrace_rs::backtrace::libunwind::trace::ha8406fd4a233ad1f
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fd57b09dfed - std::backtrace_rs::backtrace::trace_unsynchronized::h95f26ca4bd23bd0f
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fd57b09dfed - std::sys_common::backtrace::_print_fmt::h9955f0b025c061cd
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7fd57b09dfed - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1025ecd2bac85833
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7fd57b0f9c7c - core::fmt::write::hd5fa34bae8a7f0ea
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/core/src/fmt/mod.rs:1196:17
   5:     0x7fd57b08f6c1 - std::io::Write::write_fmt::h203cec764662c27d
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/io/mod.rs:1654:15
   6:     0x7fd57b0a0d05 - std::sys_common::backtrace::_print::h330c8fadbab13d8b
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7fd57b0a0d05 - std::sys_common::backtrace::print::h38d4cf1c3e16e656
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7fd57b0a0d05 - std::panicking::default_hook::{{closure}}::h56cdf194ec124abb
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/panicking.rs:295:22
   9:     0x7fd57b0a0979 - std::panicking::default_hook::h9929fdc60733f795
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/panicking.rs:314:9
  10:     0x7fd57b8ea7e1 - rustc_driver[25b442836e214e3a]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fd567f1e789 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hba67a6b74d57afdc
  12:     0x7fd567f254ab - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::h65019d64185dfc24
  13:     0x7fd57b0a14d6 - std::panicking::rust_panic_with_hook::h490228be5a55104d
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/panicking.rs:702:17
  14:     0x7fd57b0a12d7 - std::panicking::begin_panic_handler::{{closure}}::hfd963b6683c36656
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/panicking.rs:588:13
  15:     0x7fd57b09e4a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h2912e3decfe2cba4
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/sys_common/backtrace.rs:138:18
  16:     0x7fd57b0a1009 - rust_begin_unwind
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/panicking.rs:584:5
  17:     0x7fd57b066283 - core::panicking::panic_fmt::hf4ba906a2c8693a2
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/core/src/panicking.rs:142:14
  18:     0x7fd57d34594d - <rustc_middle[b283510e1dd05d73]::dep_graph::dep_node::DepKind as rustc_query_system[d6f3c753593960fb]::dep_graph::DepKind>::read_deps::<<rustc_query_system[d6f3c753593960fb]::dep_graph::graph::DepGraph<rustc_middle[b283510e1dd05d73]::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
  19:     0x7fd57d41080a - rustc_query_system[d6f3c753593960fb]::query::plumbing::get_query::<rustc_query_impl[d28b29c5c0f26ce3]::queries::def_span, rustc_query_impl[d28b29c5c0f26ce3]::plumbing::QueryCtxt>
  20:     0x7fd57d39336e - <rustc_query_impl[d28b29c5c0f26ce3]::Queries as rustc_middle[b283510e1dd05d73]::ty::query::QueryEngine>::def_span
  21:     0x7fd57c1a14a6 - <rustc_span[9389a2ea4399dbf9]::def_id::DefId as rustc_query_impl[d28b29c5c0f26ce3]::keys::Key>::default_span
  22:     0x7fd57c387389 - rustc_query_impl[d28b29c5c0f26ce3]::make_query::predicates_of
  23:     0x7fd57c28d829 - <rustc_query_system[d6f3c753593960fb]::query::plumbing::QueryState<rustc_span[9389a2ea4399dbf9]::def_id::DefId>>::try_collect_active_jobs::<rustc_query_impl[d28b29c5c0f26ce3]::plumbing::QueryCtxt>
  24:     0x7fd57c26f61c - <rustc_query_impl[d28b29c5c0f26ce3]::Queries>::try_collect_active_jobs
  25:     0x7fd57c206fae - <rustc_query_impl[d28b29c5c0f26ce3]::plumbing::QueryCtxt>::try_print_query_stack
  26:     0x7fd57b98a202 - rustc_interface[1214bec19156c976]::interface::try_print_query_stack
  27:     0x7fd57b8eb4b5 - rustc_driver[25b442836e214e3a]::report_ice
  28:     0x7fd567f1e789 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hba67a6b74d57afdc
  29:     0x7fd567f254ab - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::h65019d64185dfc24
  30:     0x7fd57b0a14d6 - std::panicking::rust_panic_with_hook::h490228be5a55104d
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/panicking.rs:702:17
  31:     0x7fd57b0a12d7 - std::panicking::begin_panic_handler::{{closure}}::hfd963b6683c36656
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/panicking.rs:588:13
  32:     0x7fd57b09e4a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h2912e3decfe2cba4
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/sys_common/backtrace.rs:138:18
  33:     0x7fd57b0a1009 - rust_begin_unwind
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/panicking.rs:584:5
  34:     0x7fd57b066283 - core::panicking::panic_fmt::hf4ba906a2c8693a2
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/core/src/panicking.rs:142:14
  35:     0x7fd57b0f68e1 - core::panicking::panic_display::h85c49ea53beff92d
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/core/src/panicking.rs:72:5
  36:     0x7fd57b0f688b - core::panicking::panic_str::h54aad4367eb7a446
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/core/src/panicking.rs:56:5
  37:     0x7fd57b0660f6 - core::option::expect_failed::h1c20531357267673
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/core/src/option.rs:1854:5
  38:     0x7fd57d49c2ea - <rustc_metadata[861e9325ed4d7b3f]::creader::CStore as rustc_session[42681064e675acd6]::cstore::CrateStore>::stable_crate_id_to_crate_num
  39:     0x7fd57d3a52e1 - <rustc_span[9389a2ea4399dbf9]::span_encoding::Span as rustc_serialize[d140f8edb489d2a6]::serialize::Decodable<rustc_query_impl[d28b29c5c0f26ce3]::on_disk_cache::CacheDecoder>>::decode
  40:     0x7fd57d337ef1 - <[(rustc_middle[b283510e1dd05d73]::ty::Predicate, rustc_span[9389a2ea4399dbf9]::span_encoding::Span)] as rustc_middle[b283510e1dd05d73]::ty::codec::RefDecodable<rustc_query_impl[d28b29c5c0f26ce3]::on_disk_cache::CacheDecoder>>::decode
  41:     0x7fd57d341bc5 - <rustc_query_impl[d28b29c5c0f26ce3]::on_disk_cache::OnDiskCache>::try_load_query_result::<rustc_middle[b283510e1dd05d73]::ty::generics::GenericPredicates>
  42:     0x7fd57d416da0 - <<rustc_query_impl[d28b29c5c0f26ce3]::queries::predicates_of as rustc_query_system[d6f3c753593960fb]::query::config::QueryDescription<rustc_query_impl[d28b29c5c0f26ce3]::plumbing::QueryCtxt>>::TRY_LOAD_FROM_DISK::{closure#0} as core[4a8947a89ffd1b77]::ops::function::FnOnce<(rustc_query_impl[d28b29c5c0f26ce3]::plumbing::QueryCtxt, rustc_query_system[d6f3c753593960fb]::dep_graph::serialized::SerializedDepNodeIndex)>>::call_once
  43:     0x7fd57d3f000e - rustc_query_system[d6f3c753593960fb]::query::plumbing::get_query::<rustc_query_impl[d28b29c5c0f26ce3]::queries::predicates_of, rustc_query_impl[d28b29c5c0f26ce3]::plumbing::QueryCtxt>
  44:     0x7fd57d390f2e - <rustc_query_impl[d28b29c5c0f26ce3]::Queries as rustc_middle[b283510e1dd05d73]::ty::query::QueryEngine>::predicates_of
  45:     0x7fd57cc8f0d9 - <rustc_privacy[c9a24284be258ba6]::ReachEverythingInTheInterfaceVisitor>::predicates
  46:     0x7fd57cc8b076 - <rustc_privacy[c9a24284be258ba6]::EmbargoVisitor as rustc_hir[374f189791373ac]::intravisit::Visitor>::visit_item
  47:     0x7fd57cc8ccfb - <rustc_privacy[c9a24284be258ba6]::EmbargoVisitor as rustc_hir[374f189791373ac]::intravisit::Visitor>::visit_item
  48:     0x7fd57daf43fb - rustc_privacy[c9a24284be258ba6]::privacy_access_levels
  49:     0x7fd57de49ed4 - <rustc_query_system[d6f3c753593960fb]::dep_graph::graph::DepGraph<rustc_middle[b283510e1dd05d73]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b283510e1dd05d73]::ty::context::TyCtxt, (), &rustc_middle[b283510e1dd05d73]::middle::privacy::AccessLevels>
  50:     0x7fd57def20f1 - rustc_query_system[d6f3c753593960fb]::query::plumbing::try_execute_query::<rustc_query_impl[d28b29c5c0f26ce3]::plumbing::QueryCtxt, rustc_query_system[d6f3c753593960fb]::query::caches::DefaultCache<(), &rustc_middle[b283510e1dd05d73]::middle::privacy::AccessLevels>>
  51:     0x7fd57df2040f - rustc_query_system[d6f3c753593960fb]::query::plumbing::get_query::<rustc_query_impl[d28b29c5c0f26ce3]::queries::privacy_access_levels, rustc_query_impl[d28b29c5c0f26ce3]::plumbing::QueryCtxt>
  52:     0x7fd57dcd90e4 - rustc_passes[c56600e865d3cc74]::stability::check_unused_or_stable_features
  53:     0x7fd57d99ffd6 - <rustc_session[42681064e675acd6]::session::Session>::time::<(), rustc_interface[1214bec19156c976]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
  54:     0x7fd57d9a0f2e - <rustc_session[42681064e675acd6]::session::Session>::time::<(), rustc_interface[1214bec19156c976]::passes::analysis::{closure#0}>
  55:     0x7fd57d99ba9e - rustc_interface[1214bec19156c976]::passes::analysis
  56:     0x7fd57de46a75 - <rustc_query_system[d6f3c753593960fb]::dep_graph::graph::DepGraph<rustc_middle[b283510e1dd05d73]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b283510e1dd05d73]::ty::context::TyCtxt, (), core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>>
  57:     0x7fd57deec181 - rustc_query_system[d6f3c753593960fb]::query::plumbing::try_execute_query::<rustc_query_impl[d28b29c5c0f26ce3]::plumbing::QueryCtxt, rustc_query_system[d6f3c753593960fb]::query::caches::DefaultCache<(), core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>>>
  58:     0x7fd57df2d49e - rustc_query_system[d6f3c753593960fb]::query::plumbing::get_query::<rustc_query_impl[d28b29c5c0f26ce3]::queries::analysis, rustc_query_impl[d28b29c5c0f26ce3]::plumbing::QueryCtxt>
  59:     0x7fd57d97c527 - <rustc_interface[1214bec19156c976]::passes::QueryContext>::enter::<rustc_driver[25b442836e214e3a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>>
  60:     0x7fd57d967588 - <rustc_interface[1214bec19156c976]::interface::Compiler>::enter::<rustc_driver[25b442836e214e3a]::run_compiler::{closure#1}::{closure#2}, core[4a8947a89ffd1b77]::result::Result<core[4a8947a89ffd1b77]::option::Option<rustc_interface[1214bec19156c976]::queries::Linker>, rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>>
  61:     0x7fd57d98fc7f - rustc_span[9389a2ea4399dbf9]::with_source_map::<core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>, rustc_interface[1214bec19156c976]::interface::create_compiler_and_run<core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>, rustc_driver[25b442836e214e3a]::run_compiler::{closure#1}>::{closure#1}>
  62:     0x7fd57d9682f4 - rustc_interface[1214bec19156c976]::interface::create_compiler_and_run::<core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>, rustc_driver[25b442836e214e3a]::run_compiler::{closure#1}>
  63:     0x7fd57d965a92 - <scoped_tls[fdc467f1f45e3091]::ScopedKey<rustc_span[9389a2ea4399dbf9]::SessionGlobals>>::set::<rustc_interface[1214bec19156c976]::interface::run_compiler<core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>, rustc_driver[25b442836e214e3a]::run_compiler::{closure#1}>::{closure#0}, core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>>
  64:     0x7fd57d97cbaf - std[49987cacb63ffdeb]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1214bec19156c976]::util::run_in_thread_pool_with_globals<rustc_interface[1214bec19156c976]::interface::run_compiler<core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>, rustc_driver[25b442836e214e3a]::run_compiler::{closure#1}>::{closure#0}, core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>>::{closure#0}, core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>>
  65:     0x7fd57d97cce9 - <<std[49987cacb63ffdeb]::thread::Builder>::spawn_unchecked_<rustc_interface[1214bec19156c976]::util::run_in_thread_pool_with_globals<rustc_interface[1214bec19156c976]::interface::run_compiler<core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>, rustc_driver[25b442836e214e3a]::run_compiler::{closure#1}>::{closure#0}, core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>>::{closure#0}, core[4a8947a89ffd1b77]::result::Result<(), rustc_errors[afa4f82815d8f672]::ErrorGuaranteed>>::{closure#1} as core[4a8947a89ffd1b77]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  66:     0x7fd57b0ab3f3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h997dc5bc9efdb91e
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/alloc/src/boxed.rs:1872:9
  67:     0x7fd57b0ab3f3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h482cb6422469721b
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/alloc/src/boxed.rs:1872:9
  68:     0x7fd57b0ab3f3 - std::sys::unix::thread::Thread::new::thread_start::hcd1011649240eef0
                               at /rustc/daf68b1f766e67ffe040260b15c218301853386a/library/std/src/sys/unix/thread.rs:108:17
  69:     0x7fd57aeb0e8d - start_thread
  70:     0x7fd57af31350 - __clone3
  71:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-beta.2 (daf68b1f7 2022-05-19) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C incremental -C symbol-mangling-version=v0 -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z force-unstable-if-unmarked

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
 Documenting rustc_symbol_mangling v0.0.0 (/home/infrandomness/Documents/Dev/rust/compiler/rustc_symbol_mangling)
 Documenting rustc_metadata v0.0.0 (/home/infrandomness/Documents/Dev/rust/compiler/rustc_metadata)
 Documenting rustc_passes v0.0.0 (/home/infrandomness/Documents/Dev/rust/compiler/rustc_passes)
 Documenting rustc_query_impl v0.0.0 (/home/infrandomness/Documents/Dev/rust/compiler/rustc_query_impl)
 Documenting rustc_mir_dataflow v0.0.0 (/home/infrandomness/Documents/Dev/rust/compiler/rustc_mir_dataflow)
 Documenting rustc_save_analysis v0.0.0 (/home/infrandomness/Documents/Dev/rust/compiler/rustc_save_analysis)
rustc exited with signal: 6 (core dumped)
error: could not compile `rustc_infer`

Caused by:
  process didn't exit successfully: `/home/infrandomness/Documents/Dev/rust/build/bootstrap/debug/rustc --crate-name rustc_infer --edition=2021 compiler/rustc_infer/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C metadata=b0d66debf4861707 -C extra-filename=-b0d66debf4861707 --out-dir /home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/incremental -L dependency=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_data_structures=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-2c525e2c3660a781.rmeta --extern rustc_errors=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-758010991976da44.rmeta --extern rustc_hir=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-fffe5c0d0b0e80e7.rmeta --extern rustc_index=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-64e9a52a9c0395ea.rmeta --extern rustc_macros=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-2df35b8caa916477.so --extern rustc_middle=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-f4d0356882e605cc.rmeta --extern rustc_serialize=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-4226a8dc8493a13d.rmeta --extern rustc_session=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-e5e981d914c76294.rmeta --extern rustc_span=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-b642bcf1a6affdb2.rmeta --extern rustc_target=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-73c4ec1e9078dec7.rmeta --extern smallvec=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-ada4cfc00bc051a5.rmeta --extern tracing=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-ec52effc85091138.rmeta --cfg=bootstrap -Csymbol-mangling-version=v0 -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Zunstable-options -Csplit-debuginfo=off -Ztls-model=initial-exec -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -L native=/home/infrandomness/Documents/Dev/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/psm-b9043e4a2f1f60fc/out` (exit status: 254)

