
thread '<unnamed>' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_mir_transform/src/check_unsafety.rs:472:36
stack backtrace:
   0:     0x7f89db332e60 - std::backtrace_rs::backtrace::libunwind::trace::h623ca7c2545da367
                               at /rustc/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f89db332e60 - std::backtrace_rs::backtrace::trace_unsynchronized::h339e54194b7bf843
                               at /rustc/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f89db332e60 - std::sys_common::backtrace::_print_fmt::h39130984054109fd
                               at /rustc/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f89db332e60 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfa5e4aa18ece27ff
                               at /rustc/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f89db39531e - core::fmt::write::h89f6ec0de92eb2ba
                               at /rustc/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/core/src/fmt/mod.rs:1208:17
   5:     0x7f89db3232e5 - std::io::Write::write_fmt::hf0e465d1b13cde08
                               at /rustc/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/std/src/io/mod.rs:1682:15
   6:     0x7f89db332c25 - std::sys_common::backtrace::_print::h5c3a6b9e92733123
                               at /rustc/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f89db332c25 - std::sys_common::backtrace::print::h617c6080818fd336
                               at /rustc/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f89db3359cf - std::panicking::default_hook::{{closure}}::ha4c000959a58a266
                               at /rustc/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/std/src/panicking.rs:267:22
   9:     0x7f89db33570a - std::panicking::default_hook::h80287e6b1a8ab5a3
                               at /rustc/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/std/src/panicking.rs:286:9
  10:     0x7f89db3361dc - std::panicking::rust_panic_with_hook::hd38f216792535192
                               at /rustc/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/std/src/panicking.rs:688:13
  11:     0x7f89db335f77 - std::panicking::begin_panic_handler::{{closure}}::h04dbfee15e75c657
                               at /rustc/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/std/src/panicking.rs:579:13
  12:     0x7f89db33330c - std::sys_common::backtrace::__rust_end_short_backtrace::hefea495a2b02629d
                               at /rustc/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/std/src/sys_common/backtrace.rs:137:18
  13:     0x7f89db335c92 - rust_begin_unwind
                               at /rustc/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/std/src/panicking.rs:575:5
  14:     0x7f89db391cf3 - core::panicking::panic_fmt::h97a0144c9d72a8f8
                               at /rustc/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/core/src/panicking.rs:65:14
  15:     0x7f89dcd1db25 - rustc_mir_transform[90ab4d560697ca18]::check_unsafety::unsafety_check_result
  16:     0x7f89dcd1b3f8 - <rustc_mir_transform[90ab4d560697ca18]::check_unsafety::provide::{closure#0} as core[3fabb2b3983330a3]::ops::function::FnOnce<(rustc_middle[abcfb14e24642545]::ty::context::TyCtxt, rustc_span[9a6606dc9b427585]::def_id::LocalDefId)>>::call_once
  17:     0x7f89ded22480 - <rustc_middle[abcfb14e24642545]::dep_graph::dep_node::DepKind as rustc_query_system[a61e76100b0caf6f]::dep_graph::DepKind>::with_deps::<rustc_query_system[a61e76100b0caf6f]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[804366da8be58efc]::plumbing::QueryCtxt, rustc_span[9a6606dc9b427585]::def_id::LocalDefId, &rustc_middle[abcfb14e24642545]::mir::query::UnsafetyCheckResult>::{closure#1}, &rustc_middle[abcfb14e24642545]::mir::query::UnsafetyCheckResult>
  18:     0x7f89dcd1b907 - rustc_query_system[a61e76100b0caf6f]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[804366da8be58efc]::plumbing::QueryCtxt, rustc_span[9a6606dc9b427585]::def_id::LocalDefId, &rustc_middle[abcfb14e24642545]::mir::query::UnsafetyCheckResult>
  19:     0x7f89dcd1a763 - rustc_query_system[a61e76100b0caf6f]::query::plumbing::try_execute_query::<rustc_query_impl[804366da8be58efc]::plumbing::QueryCtxt, rustc_query_system[a61e76100b0caf6f]::query::caches::DefaultCache<rustc_span[9a6606dc9b427585]::def_id::LocalDefId, &rustc_middle[abcfb14e24642545]::mir::query::UnsafetyCheckResult>>
  20:     0x7f89ddf03c64 - <rustc_query_impl[804366da8be58efc]::Queries as rustc_middle[abcfb14e24642545]::ty::query::QueryEngine>::unsafety_check_result
  21:     0x7f89dc92aae7 - rustc_mir_transform[90ab4d560697ca18]::check_unsafety::check_unsafety
  22:     0x7f89dc929b21 - <rustc_session[3b411079dabf57a]::session::Session>::time::<(), rustc_interface[1c4c4049c89b9f16]::passes::analysis::{closure#3}>
  23:     0x7f89dc92553e - rustc_interface[1c4c4049c89b9f16]::passes::analysis
  24:     0x7f89ddd27d2e - <rustc_query_system[a61e76100b0caf6f]::dep_graph::graph::DepGraph<rustc_middle[abcfb14e24642545]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[abcfb14e24642545]::ty::context::TyCtxt, (), core[3fabb2b3983330a3]::result::Result<(), rustc_errors[5b60f759d3f57066]::ErrorGuaranteed>>
  25:     0x7f89ddd270a4 - rustc_query_system[a61e76100b0caf6f]::query::plumbing::try_execute_query::<rustc_query_impl[804366da8be58efc]::plumbing::QueryCtxt, rustc_query_system[a61e76100b0caf6f]::query::caches::DefaultCache<(), core[3fabb2b3983330a3]::result::Result<(), rustc_errors[5b60f759d3f57066]::ErrorGuaranteed>>>
  26:     0x7f89ddd26b27 - rustc_query_system[a61e76100b0caf6f]::query::plumbing::get_query::<rustc_query_impl[804366da8be58efc]::queries::analysis, rustc_query_impl[804366da8be58efc]::plumbing::QueryCtxt>
  27:     0x7f89dd80dd7e - <rustc_interface[1c4c4049c89b9f16]::passes::QueryContext>::enter::<rustc_driver[264bd666a4ec3967]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[3fabb2b3983330a3]::result::Result<(), rustc_errors[5b60f759d3f57066]::ErrorGuaranteed>>
  28:     0x7f89dd80af5f - <rustc_interface[1c4c4049c89b9f16]::interface::Compiler>::enter::<rustc_driver[264bd666a4ec3967]::run_compiler::{closure#1}::{closure#2}, core[3fabb2b3983330a3]::result::Result<core[3fabb2b3983330a3]::option::Option<rustc_interface[1c4c4049c89b9f16]::queries::Linker>, rustc_errors[5b60f759d3f57066]::ErrorGuaranteed>>
  29:     0x7f89dd805fb2 - rustc_span[9a6606dc9b427585]::with_source_map::<core[3fabb2b3983330a3]::result::Result<(), rustc_errors[5b60f759d3f57066]::ErrorGuaranteed>, rustc_interface[1c4c4049c89b9f16]::interface::run_compiler<core[3fabb2b3983330a3]::result::Result<(), rustc_errors[5b60f759d3f57066]::ErrorGuaranteed>, rustc_driver[264bd666a4ec3967]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  30:     0x7f89dd805a8c - <scoped_tls[1c7f0121e168e3b1]::ScopedKey<rustc_span[9a6606dc9b427585]::SessionGlobals>>::set::<rustc_interface[1c4c4049c89b9f16]::interface::run_compiler<core[3fabb2b3983330a3]::result::Result<(), rustc_errors[5b60f759d3f57066]::ErrorGuaranteed>, rustc_driver[264bd666a4ec3967]::run_compiler::{closure#1}>::{closure#0}, core[3fabb2b3983330a3]::result::Result<(), rustc_errors[5b60f759d3f57066]::ErrorGuaranteed>>
  31:     0x7f89dd805078 - std[a12c3934657dc52f]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1c4c4049c89b9f16]::util::run_in_thread_pool_with_globals<rustc_interface[1c4c4049c89b9f16]::interface::run_compiler<core[3fabb2b3983330a3]::result::Result<(), rustc_errors[5b60f759d3f57066]::ErrorGuaranteed>, rustc_driver[264bd666a4ec3967]::run_compiler::{closure#1}>::{closure#0}, core[3fabb2b3983330a3]::result::Result<(), rustc_errors[5b60f759d3f57066]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3fabb2b3983330a3]::result::Result<(), rustc_errors[5b60f759d3f57066]::ErrorGuaranteed>>
  32:     0x7f89dd804d9c - <<std[a12c3934657dc52f]::thread::Builder>::spawn_unchecked_<rustc_interface[1c4c4049c89b9f16]::util::run_in_thread_pool_with_globals<rustc_interface[1c4c4049c89b9f16]::interface::run_compiler<core[3fabb2b3983330a3]::result::Result<(), rustc_errors[5b60f759d3f57066]::ErrorGuaranteed>, rustc_driver[264bd666a4ec3967]::run_compiler::{closure#1}>::{closure#0}, core[3fabb2b3983330a3]::result::Result<(), rustc_errors[5b60f759d3f57066]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3fabb2b3983330a3]::result::Result<(), rustc_errors[5b60f759d3f57066]::ErrorGuaranteed>>::{closure#1} as core[3fabb2b3983330a3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  33:     0x7f89df274c63 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::he19adc936b97b661
                               at /rustc/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/alloc/src/boxed.rs:2000:9
  34:     0x7f89df274c63 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h6b08bc0be1f88378
                               at /rustc/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/alloc/src/boxed.rs:2000:9
  35:     0x7f89df274c63 - std::sys::unix::thread::Thread::new::thread_start::hb8a4d02754a9d514
                               at /rustc/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/std/src/sys/unix/thread.rs:108:17
  36:     0x7f89db1eb609 - start_thread
                               at /build/glibc-SzIz7B/glibc-2.31/nptl/pthread_create.c:477:8
  37:     0x7f89db10e133 - clone
                               at /build/glibc-SzIz7B/glibc-2.31/misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:95
  38:                0x0 - <unknown>
