
Building {core} stage0 library artifacts (x86_64-unknown-linux-gnu) 
   Compiling core v0.0.0 (/home/jyn/src/rust3/library/core)
thread 'rustc' panicked at 'index out of bounds: the len is 78 but the index is 144', compiler/rustc_query_impl/src/on_disk_cache.rs:717:40
stack backtrace:
   0:     0x7f9ba582231a - std::backtrace_rs::backtrace::libunwind::trace::h6afeafcf92b09bcd
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f9ba582231a - std::backtrace_rs::backtrace::trace_unsynchronized::h6f2afab8efd008d2
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f9ba582231a - std::sys_common::backtrace::_print_fmt::hf6f0f010fea6e3fd
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f9ba582231a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c4abbc84b2347e3
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f9ba5885b6e - core::fmt::write::h7604b6ac1e51564a
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/core/src/fmt/mod.rs:1232:17
   5:     0x7f9ba58151c5 - std::io::Write::write_fmt::h06952c4611dbdea8
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/io/mod.rs:1684:15
   6:     0x7f9ba58220e5 - std::sys_common::backtrace::_print::h5a8a2578f0a5a262
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f9ba58220e5 - std::sys_common::backtrace::print::h9cd3257ab29a9457
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f9ba5824e5f - std::panicking::default_hook::{{closure}}::h2efc46d4738c4264
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:271:22
   9:     0x7f9ba5824b9b - std::panicking::default_hook::hcbff7ff0fc72f592
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:290:9
  10:     0x7f9ba45c56b5 - rustc_driver_impl[b2d0c9d39cec3f08]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f9ba582569d - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hc722c46abd4bee52
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/alloc/src/boxed.rs:2001:9
  12:     0x7f9ba582569d - std::panicking::rust_panic_with_hook::h2a4f4becb061dab8
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:696:13
  13:     0x7f9ba5825419 - std::panicking::begin_panic_handler::{{closure}}::hfbfd1835b102f703
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:583:13
  14:     0x7f9ba5822786 - std::sys_common::backtrace::__rust_end_short_backtrace::hff723b81c54b7ea4
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys_common/backtrace.rs:150:18
  15:     0x7f9ba5825122 - rust_begin_unwind
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:579:5
  16:     0x7f9ba5881ec3 - core::panicking::panic_fmt::hf07572e75eddd3b0
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/core/src/panicking.rs:64:14
  17:     0x7f9ba58820b2 - core::panicking::panic_bounds_check::heb764f164d16c0f3
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/core/src/panicking.rs:159:5
  18:     0x7f9ba2aa968a - <rustc_span[5cba7a76607c212]::span_encoding::Span as rustc_serialize[3fbcda3f3a7af78b]::serialize::Decodable<rustc_query_impl[f5702d406d4b30ad]::on_disk_cache::CacheDecoder>>::decode
  19:     0x7f9ba4fd738d - <rustc_span[5cba7a76607c212]::hygiene::ExpnData as rustc_serialize[3fbcda3f3a7af78b]::serialize::Decodable<rustc_query_impl[f5702d406d4b30ad]::on_disk_cache::CacheDecoder>>::decode
  20:     0x7f9ba4f546da - <rustc_query_impl[f5702d406d4b30ad]::on_disk_cache::CacheDecoder as rustc_type_ir[99160ce57ac15fd9]::codec::TyDecoder>::with_position::<<rustc_span[5cba7a76607c212]::hygiene::ExpnId as rustc_serialize[3fbcda3f3a7af78b]::serialize::Decodable<rustc_query_impl[f5702d406d4b30ad]::on_disk_cache::CacheDecoder>>::decode::{closure#1}, rustc_span[5cba7a76607c212]::hygiene::ExpnData>
  21:     0x7f9ba2aac00d - <rustc_span[5cba7a76607c212]::hygiene::SyntaxContextData as rustc_serialize[3fbcda3f3a7af78b]::serialize::Decodable<rustc_query_impl[f5702d406d4b30ad]::on_disk_cache::CacheDecoder>>::decode
  22:     0x7f9ba2aa85dc - <rustc_span[5cba7a76607c212]::span_encoding::Span as rustc_serialize[3fbcda3f3a7af78b]::serialize::Decodable<rustc_query_impl[f5702d406d4b30ad]::on_disk_cache::CacheDecoder>>::decode
  23:     0x7f9ba31812b0 - rustc_query_system[2a4cc30c318fa14d]::query::plumbing::try_execute_query::<rustc_query_impl[f5702d406d4b30ad]::queries::def_span, rustc_query_impl[f5702d406d4b30ad]::plumbing::QueryCtxt>
  24:     0x7f9ba2c2f977 - rustc_ty_utils[2fe753b497c67f71]::ty::param_env
  25:     0x7f9ba2c2d8c6 - rustc_query_system[2a4cc30c318fa14d]::query::plumbing::try_execute_query::<rustc_query_impl[f5702d406d4b30ad]::queries::param_env, rustc_query_impl[f5702d406d4b30ad]::plumbing::QueryCtxt>
  26:     0x7f9ba2c2cb60 - <rustc_query_impl[f5702d406d4b30ad]::Queries as rustc_middle[5904c30a6dccf498]::ty::query::QueryEngine>::param_env
  27:     0x7f9ba2c45cce - rustc_mir_build[f38372ded6a67f80]::thir::pattern::check_match::check_match
  28:     0x7f9ba2c44cfc - rustc_query_system[2a4cc30c318fa14d]::query::plumbing::try_execute_query::<rustc_query_impl[f5702d406d4b30ad]::queries::check_match, rustc_query_impl[f5702d406d4b30ad]::plumbing::QueryCtxt>
  29:     0x7f9ba3cf15d6 - rustc_data_structures[f96f114103bf97c3]::sync::par_for_each_in::<&[rustc_span[5cba7a76607c212]::def_id::LocalDefId], <rustc_middle[5904c30a6dccf498]::hir::map::Map>::par_body_owners<rustc_interface[4988ae148fb02a7]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>
  30:     0x7f9ba3cf119e - <rustc_session[7ad75fa329ac590f]::session::Session>::time::<(), rustc_interface[4988ae148fb02a7]::passes::analysis::{closure#1}::{closure#0}::{closure#0}>
  31:     0x7f9ba29776ee - <rustc_session[7ad75fa329ac590f]::session::Session>::time::<(), rustc_interface[4988ae148fb02a7]::passes::analysis::{closure#1}>
  32:     0x7f9ba2977239 - rustc_interface[4988ae148fb02a7]::passes::analysis
  33:     0x7f9ba3e28fb8 - rustc_query_system[2a4cc30c318fa14d]::query::plumbing::try_execute_query::<rustc_query_impl[f5702d406d4b30ad]::queries::analysis, rustc_query_impl[f5702d406d4b30ad]::plumbing::QueryCtxt>
  34:     0x7f9ba3e28a8f - <rustc_query_impl[f5702d406d4b30ad]::Queries as rustc_middle[5904c30a6dccf498]::ty::query::QueryEngine>::analysis
  35:     0x7f9ba3c4d736 - <rustc_middle[5904c30a6dccf498]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[b2d0c9d39cec3f08]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>>
  36:     0x7f9ba3828958 - rustc_span[5cba7a76607c212]::with_source_map::<core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>, rustc_interface[4988ae148fb02a7]::interface::run_compiler<core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>, rustc_driver_impl[b2d0c9d39cec3f08]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  37:     0x7f9ba381ffcc - std[7f3793225a10bff4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4988ae148fb02a7]::util::run_in_thread_pool_with_globals<rustc_interface[4988ae148fb02a7]::interface::run_compiler<core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>, rustc_driver_impl[b2d0c9d39cec3f08]::run_compiler::{closure#1}>::{closure#0}, core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>>
  38:     0x7f9ba381f9fa - <<std[7f3793225a10bff4]::thread::Builder>::spawn_unchecked_<rustc_interface[4988ae148fb02a7]::util::run_in_thread_pool_with_globals<rustc_interface[4988ae148fb02a7]::interface::run_compiler<core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>, rustc_driver_impl[b2d0c9d39cec3f08]::run_compiler::{closure#1}>::{closure#0}, core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>>::{closure#1} as core[b162ae3dcbce6770]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7f9ba582f593 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h168eb1356e410ac7
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/alloc/src/boxed.rs:1987:9
  40:     0x7f9ba582f593 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hc48e839c56707dd0
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/alloc/src/boxed.rs:1987:9
  41:     0x7f9ba582f593 - std::sys::unix::thread::Thread::new::thread_start::hdf98ced2fabec86a
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys/unix/thread.rs:108:17
  42:     0x7f9ba1094b43 - start_thread
                               at ./nptl/./nptl/pthread_create.c:442:8
  43:     0x7f9ba1126a00 - clone3
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
  44:                0x0 - <unknown>

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-beta.1 (b955c8271 2023-03-06) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C incremental=[REDACTED] -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C lto=off -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
thread 'rustc' panicked at 'Illegal read of: 346469', /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/compiler/rustc_query_system/src/dep_graph/graph.rs:450:25
stack backtrace:
   0:     0x7f9ba582231a - std::backtrace_rs::backtrace::libunwind::trace::h6afeafcf92b09bcd
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f9ba582231a - std::backtrace_rs::backtrace::trace_unsynchronized::h6f2afab8efd008d2
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f9ba582231a - std::sys_common::backtrace::_print_fmt::hf6f0f010fea6e3fd
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f9ba582231a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c4abbc84b2347e3
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f9ba5885b6e - core::fmt::write::h7604b6ac1e51564a
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/core/src/fmt/mod.rs:1232:17
   5:     0x7f9ba58151c5 - std::io::Write::write_fmt::h06952c4611dbdea8
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/io/mod.rs:1684:15
   6:     0x7f9ba58220e5 - std::sys_common::backtrace::_print::h5a8a2578f0a5a262
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f9ba58220e5 - std::sys_common::backtrace::print::h9cd3257ab29a9457
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f9ba5824e5f - std::panicking::default_hook::{{closure}}::h2efc46d4738c4264
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:271:22
   9:     0x7f9ba5824b9b - std::panicking::default_hook::hcbff7ff0fc72f592
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:290:9
  10:     0x7f9ba45c56b5 - rustc_driver_impl[b2d0c9d39cec3f08]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f9ba582569d - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hc722c46abd4bee52
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/alloc/src/boxed.rs:2001:9
  12:     0x7f9ba582569d - std::panicking::rust_panic_with_hook::h2a4f4becb061dab8
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:696:13
  13:     0x7f9ba5825419 - std::panicking::begin_panic_handler::{{closure}}::hfbfd1835b102f703
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:583:13
  14:     0x7f9ba5822786 - std::sys_common::backtrace::__rust_end_short_backtrace::hff723b81c54b7ea4
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys_common/backtrace.rs:150:18
  15:     0x7f9ba5825122 - rust_begin_unwind
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:579:5
  16:     0x7f9ba5881ec3 - core::panicking::panic_fmt::hf07572e75eddd3b0
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/core/src/panicking.rs:64:14
  17:     0x7f9ba261f4d7 - <rustc_middle[5904c30a6dccf498]::dep_graph::dep_node::DepKind as rustc_query_system[2a4cc30c318fa14d]::dep_graph::DepKind>::read_deps::<<rustc_query_system[2a4cc30c318fa14d]::dep_graph::graph::DepGraph<rustc_middle[5904c30a6dccf498]::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
  18:     0x7f9ba27438e3 - <rustc_middle[5904c30a6dccf498]::ty::print::pretty::FmtPrinter>::new
  19:     0x7f9ba33d9594 - <rustc_middle[5904c30a6dccf498]::ty::context::TyCtxt>::def_path_str_with_substs
  20:     0x7f9ba4b47ca7 - rustc_middle[5904c30a6dccf498]::query::descs::check_match
  21:     0x7f9ba4f716c4 - rustc_query_impl[f5702d406d4b30ad]::plumbing::create_query_frame::<rustc_span[5cba7a76607c212]::def_id::DefId>
  22:     0x7f9ba4fdbbe9 - <rustc_query_impl[f5702d406d4b30ad]::query_structs::check_match::{closure#0}::{closure#0} as core[b162ae3dcbce6770]::ops::function::FnOnce<(rustc_query_impl[f5702d406d4b30ad]::plumbing::QueryCtxt, rustc_span[5cba7a76607c212]::def_id::DefId)>>::call_once
  23:     0x7f9ba4e70620 - <rustc_query_system[2a4cc30c318fa14d]::query::plumbing::QueryState<rustc_span[5cba7a76607c212]::def_id::DefId, rustc_middle[5904c30a6dccf498]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[f5702d406d4b30ad]::plumbing::QueryCtxt>
  24:     0x7f9ba3d83252 - <rustc_query_impl[f5702d406d4b30ad]::Queries>::try_collect_active_jobs
  25:     0x7f9ba4f848c8 - rustc_query_system[2a4cc30c318fa14d]::query::job::print_query_stack::<rustc_query_impl[f5702d406d4b30ad]::plumbing::QueryCtxt>
  26:     0x7f9ba4981db2 - rustc_interface[4988ae148fb02a7]::interface::try_print_query_stack
  27:     0x7f9ba45c7a8e - rustc_driver_impl[b2d0c9d39cec3f08]::report_ice
  28:     0x7f9ba45c56fc - rustc_driver_impl[b2d0c9d39cec3f08]::DEFAULT_HOOK::{closure#0}::{closure#0}
  29:     0x7f9ba582569d - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hc722c46abd4bee52
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/alloc/src/boxed.rs:2001:9
  30:     0x7f9ba582569d - std::panicking::rust_panic_with_hook::h2a4f4becb061dab8
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:696:13
  31:     0x7f9ba5825419 - std::panicking::begin_panic_handler::{{closure}}::hfbfd1835b102f703
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:583:13
  32:     0x7f9ba5822786 - std::sys_common::backtrace::__rust_end_short_backtrace::hff723b81c54b7ea4
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys_common/backtrace.rs:150:18
  33:     0x7f9ba5825122 - rust_begin_unwind
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/panicking.rs:579:5
  34:     0x7f9ba5881ec3 - core::panicking::panic_fmt::hf07572e75eddd3b0
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/core/src/panicking.rs:64:14
  35:     0x7f9ba58820b2 - core::panicking::panic_bounds_check::heb764f164d16c0f3
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/core/src/panicking.rs:159:5
  36:     0x7f9ba2aa968a - <rustc_span[5cba7a76607c212]::span_encoding::Span as rustc_serialize[3fbcda3f3a7af78b]::serialize::Decodable<rustc_query_impl[f5702d406d4b30ad]::on_disk_cache::CacheDecoder>>::decode
  37:     0x7f9ba4fd738d - <rustc_span[5cba7a76607c212]::hygiene::ExpnData as rustc_serialize[3fbcda3f3a7af78b]::serialize::Decodable<rustc_query_impl[f5702d406d4b30ad]::on_disk_cache::CacheDecoder>>::decode
  38:     0x7f9ba4f546da - <rustc_query_impl[f5702d406d4b30ad]::on_disk_cache::CacheDecoder as rustc_type_ir[99160ce57ac15fd9]::codec::TyDecoder>::with_position::<<rustc_span[5cba7a76607c212]::hygiene::ExpnId as rustc_serialize[3fbcda3f3a7af78b]::serialize::Decodable<rustc_query_impl[f5702d406d4b30ad]::on_disk_cache::CacheDecoder>>::decode::{closure#1}, rustc_span[5cba7a76607c212]::hygiene::ExpnData>
  39:     0x7f9ba2aac00d - <rustc_span[5cba7a76607c212]::hygiene::SyntaxContextData as rustc_serialize[3fbcda3f3a7af78b]::serialize::Decodable<rustc_query_impl[f5702d406d4b30ad]::on_disk_cache::CacheDecoder>>::decode
  40:     0x7f9ba2aa85dc - <rustc_span[5cba7a76607c212]::span_encoding::Span as rustc_serialize[3fbcda3f3a7af78b]::serialize::Decodable<rustc_query_impl[f5702d406d4b30ad]::on_disk_cache::CacheDecoder>>::decode
  41:     0x7f9ba31812b0 - rustc_query_system[2a4cc30c318fa14d]::query::plumbing::try_execute_query::<rustc_query_impl[f5702d406d4b30ad]::queries::def_span, rustc_query_impl[f5702d406d4b30ad]::plumbing::QueryCtxt>
  42:     0x7f9ba2c2f977 - rustc_ty_utils[2fe753b497c67f71]::ty::param_env
  43:     0x7f9ba2c2d8c6 - rustc_query_system[2a4cc30c318fa14d]::query::plumbing::try_execute_query::<rustc_query_impl[f5702d406d4b30ad]::queries::param_env, rustc_query_impl[f5702d406d4b30ad]::plumbing::QueryCtxt>
  44:     0x7f9ba2c2cb60 - <rustc_query_impl[f5702d406d4b30ad]::Queries as rustc_middle[5904c30a6dccf498]::ty::query::QueryEngine>::param_env
  45:     0x7f9ba2c45cce - rustc_mir_build[f38372ded6a67f80]::thir::pattern::check_match::check_match
  46:     0x7f9ba2c44cfc - rustc_query_system[2a4cc30c318fa14d]::query::plumbing::try_execute_query::<rustc_query_impl[f5702d406d4b30ad]::queries::check_match, rustc_query_impl[f5702d406d4b30ad]::plumbing::QueryCtxt>
  47:     0x7f9ba3cf15d6 - rustc_data_structures[f96f114103bf97c3]::sync::par_for_each_in::<&[rustc_span[5cba7a76607c212]::def_id::LocalDefId], <rustc_middle[5904c30a6dccf498]::hir::map::Map>::par_body_owners<rustc_interface[4988ae148fb02a7]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>
  48:     0x7f9ba3cf119e - <rustc_session[7ad75fa329ac590f]::session::Session>::time::<(), rustc_interface[4988ae148fb02a7]::passes::analysis::{closure#1}::{closure#0}::{closure#0}>
  49:     0x7f9ba29776ee - <rustc_session[7ad75fa329ac590f]::session::Session>::time::<(), rustc_interface[4988ae148fb02a7]::passes::analysis::{closure#1}>
  50:     0x7f9ba2977239 - rustc_interface[4988ae148fb02a7]::passes::analysis
  51:     0x7f9ba3e28fb8 - rustc_query_system[2a4cc30c318fa14d]::query::plumbing::try_execute_query::<rustc_query_impl[f5702d406d4b30ad]::queries::analysis, rustc_query_impl[f5702d406d4b30ad]::plumbing::QueryCtxt>
  52:     0x7f9ba3e28a8f - <rustc_query_impl[f5702d406d4b30ad]::Queries as rustc_middle[5904c30a6dccf498]::ty::query::QueryEngine>::analysis
  53:     0x7f9ba3c4d736 - <rustc_middle[5904c30a6dccf498]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[b2d0c9d39cec3f08]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>>
  54:     0x7f9ba3828958 - rustc_span[5cba7a76607c212]::with_source_map::<core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>, rustc_interface[4988ae148fb02a7]::interface::run_compiler<core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>, rustc_driver_impl[b2d0c9d39cec3f08]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  55:     0x7f9ba381ffcc - std[7f3793225a10bff4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4988ae148fb02a7]::util::run_in_thread_pool_with_globals<rustc_interface[4988ae148fb02a7]::interface::run_compiler<core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>, rustc_driver_impl[b2d0c9d39cec3f08]::run_compiler::{closure#1}>::{closure#0}, core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>>
  56:     0x7f9ba381f9fa - <<std[7f3793225a10bff4]::thread::Builder>::spawn_unchecked_<rustc_interface[4988ae148fb02a7]::util::run_in_thread_pool_with_globals<rustc_interface[4988ae148fb02a7]::interface::run_compiler<core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>, rustc_driver_impl[b2d0c9d39cec3f08]::run_compiler::{closure#1}>::{closure#0}, core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b162ae3dcbce6770]::result::Result<(), rustc_span[5cba7a76607c212]::ErrorGuaranteed>>::{closure#1} as core[b162ae3dcbce6770]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  57:     0x7f9ba582f593 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h168eb1356e410ac7
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/alloc/src/boxed.rs:1987:9
  58:     0x7f9ba582f593 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hc48e839c56707dd0
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/alloc/src/boxed.rs:1987:9
  59:     0x7f9ba582f593 - std::sys::unix::thread::Thread::new::thread_start::hdf98ced2fabec86a
                               at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys/unix/thread.rs:108:17
  60:     0x7f9ba1094b43 - start_thread
                               at ./nptl/./nptl/pthread_create.c:442:8
  61:     0x7f9ba1126a00 - clone3
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
  62:                0x0 - <unknown>

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-beta.1 (b955c8271 2023-03-06) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C incremental=[REDACTED] -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C lto=off -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
thread panicked while processing panic. aborting.
rustc exited with signal: 6 (SIGABRT) (core dumped)
error: could not compile `core`

Caused by:
  process didn't exit successfully: `/home/jyn/src/rust3/build/bootstrap/debug/rustc --crate-name core --edition=2021 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=129 --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' -C metadata=999232a2b2133b8a -C extra-filename=-999232a2b2133b8a --out-dir /home/jyn/src/rust3/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/home/jyn/src/rust3/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/incremental -L dependency=/home/jyn/src/rust3/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/jyn/src/rust3/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa")' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Csplit-debuginfo=off -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Clto=off '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo` (exit status: 254)
Build completed unsuccessfully in 0:00:08
