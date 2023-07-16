
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_monomorphize/src/collector.rs:961:13
stack backtrace:
   0:     0x7f724636ced0 - std::backtrace_rs::backtrace::libunwind::trace::h6b3f9f41ee650c4f
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7f724636ced0 - std::backtrace_rs::backtrace::trace_unsynchronized::he56cb04310aea81a
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f724636ced0 - std::sys_common::backtrace::_print_fmt::h6d4ec34d6cd34171
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f724636ced0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h01911f0526d8b05c
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f72463c82ac - core::fmt::write::h49f323f7091ea1c8
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/core/src/fmt/mod.rs:1198:17
   5:     0x7f724635dd85 - std::io::Write::write_fmt::hbf1f476d48d43edd
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/std/src/io/mod.rs:1672:15
   6:     0x7f724636fba1 - std::sys_common::backtrace::_print::h2bbdb393c4180f57
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f724636fba1 - std::sys_common::backtrace::print::h4200c2a0dbbbc055
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f724636fba1 - std::panicking::default_hook::{{closure}}::h393b25bd4d38364f
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/std/src/panicking.rs:295:22
   9:     0x7f724636f873 - std::panicking::default_hook::h70f1f206e4c542c6
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/std/src/panicking.rs:314:9
  10:     0x7f7248bc75f1 - <rustc_driver[72308b4096139644]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[cd150e9e29982b29]::ops::function::FnOnce<(&core[cd150e9e29982b29]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7f72463703d6 - std::panicking::rust_panic_with_hook::hdfebab2861adaf27
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/std/src/panicking.rs:702:17
  12:     0x7f72463701e9 - std::panicking::begin_panic_handler::{{closure}}::h7b45e1150590a6fe
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/std/src/panicking.rs:586:13
  13:     0x7f724636d3b4 - std::sys_common::backtrace::__rust_end_short_backtrace::hcfb5f80ba126c43c
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7f724636ff52 - rust_begin_unwind
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/std/src/panicking.rs:584:5
  15:     0x7f72463c4d83 - core::panicking::panic_fmt::h819a1bea3462d6ca
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/core/src/panicking.rs:142:14
  16:     0x7f72463c4bcd - core::panicking::panic::h9b1240c8697457a7
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/core/src/panicking.rs:48:5
  17:     0x7f724777d488 - rustc_monomorphize[5cdd28e9061c1419]::collector::collect_neighbours
  18:     0x7f7247774206 - rustc_monomorphize[5cdd28e9061c1419]::collector::collect_items_rec
  19:     0x7f72477746ec - rustc_monomorphize[5cdd28e9061c1419]::collector::collect_items_rec
  20:     0x7f72477746ec - rustc_monomorphize[5cdd28e9061c1419]::collector::collect_items_rec
  21:     0x7f7248211abb - <rustc_session[b6be1dcf14d2202f]::session::Session>::time::<(), rustc_monomorphize[5cdd28e9061c1419]::collector::collect_crate_mono_items::{closure#1}>
  22:     0x7f724821168b - rustc_monomorphize[5cdd28e9061c1419]::collector::collect_crate_mono_items
  23:     0x7f7248210356 - rustc_monomorphize[5cdd28e9061c1419]::partitioning::collect_and_partition_mono_items
  24:     0x7f72486dbb75 - rustc_query_system[2682b511cc84fabe]::query::plumbing::try_execute_query::<rustc_query_impl[5e8382c68d202a33]::plumbing::QueryCtxt, rustc_query_system[2682b511cc84fabe]::query::caches::DefaultCache<(), (&std[778605c8b9d7d939]::collections::hash::set::HashSet<rustc_span[7c9df871791ece55]::def_id::DefId, core[cd150e9e29982b29]::hash::BuildHasherDefault<rustc_hash[9783f680492ba222]::FxHasher>>, &[rustc_middle[c6afec08b0d1be81]::mir::mono::CodegenUnit])>>
  25:     0x7f72486db8d0 - rustc_query_system[2682b511cc84fabe]::query::plumbing::get_query::<rustc_query_impl[5e8382c68d202a33]::queries::collect_and_partition_mono_items, rustc_query_impl[5e8382c68d202a33]::plumbing::QueryCtxt>
  26:     0x7f72486db822 - <rustc_query_impl[5e8382c68d202a33]::Queries as rustc_middle[c6afec08b0d1be81]::ty::query::QueryEngine>::collect_and_partition_mono_items
  27:     0x7f724886e586 - rustc_codegen_ssa[72bdf18198f6cb76]::base::codegen_crate::<rustc_codegen_llvm[d9a63b68ef949d5a]::LlvmCodegenBackend>
  28:     0x7f724886e351 - <rustc_codegen_llvm[d9a63b68ef949d5a]::LlvmCodegenBackend as rustc_codegen_ssa[72bdf18198f6cb76]::traits::backend::CodegenBackend>::codegen_crate
  29:     0x7f72482b7ec7 - <rustc_session[b6be1dcf14d2202f]::session::Session>::time::<alloc[110eb368cf5d93ca]::boxed::Box<dyn core[cd150e9e29982b29]::any::Any>, rustc_interface[8ca32d3d634051a7]::passes::start_codegen::{closure#0}>
  30:     0x7f72482b77e3 - <rustc_interface[8ca32d3d634051a7]::passes::QueryContext>::enter::<<rustc_interface[8ca32d3d634051a7]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[cd150e9e29982b29]::result::Result<alloc[110eb368cf5d93ca]::boxed::Box<dyn core[cd150e9e29982b29]::any::Any>, rustc_errors[5ec552083a70b038]::ErrorGuaranteed>>
  31:     0x7f72482af233 - <rustc_interface[8ca32d3d634051a7]::queries::Queries>::ongoing_codegen
  32:     0x7f72482ae291 - <rustc_interface[8ca32d3d634051a7]::interface::Compiler>::enter::<rustc_driver[72308b4096139644]::run_compiler::{closure#1}::{closure#2}, core[cd150e9e29982b29]::result::Result<core[cd150e9e29982b29]::option::Option<rustc_interface[8ca32d3d634051a7]::queries::Linker>, rustc_errors[5ec552083a70b038]::ErrorGuaranteed>>
  33:     0x7f72482a929f - rustc_span[7c9df871791ece55]::with_source_map::<core[cd150e9e29982b29]::result::Result<(), rustc_errors[5ec552083a70b038]::ErrorGuaranteed>, rustc_interface[8ca32d3d634051a7]::interface::create_compiler_and_run<core[cd150e9e29982b29]::result::Result<(), rustc_errors[5ec552083a70b038]::ErrorGuaranteed>, rustc_driver[72308b4096139644]::run_compiler::{closure#1}>::{closure#1}>
  34:     0x7f72482a8c90 - rustc_interface[8ca32d3d634051a7]::interface::create_compiler_and_run::<core[cd150e9e29982b29]::result::Result<(), rustc_errors[5ec552083a70b038]::ErrorGuaranteed>, rustc_driver[72308b4096139644]::run_compiler::{closure#1}>
  35:     0x7f72482a7722 - <scoped_tls[3f6da10b6a2c80a0]::ScopedKey<rustc_span[7c9df871791ece55]::SessionGlobals>>::set::<rustc_interface[8ca32d3d634051a7]::interface::run_compiler<core[cd150e9e29982b29]::result::Result<(), rustc_errors[5ec552083a70b038]::ErrorGuaranteed>, rustc_driver[72308b4096139644]::run_compiler::{closure#1}>::{closure#0}, core[cd150e9e29982b29]::result::Result<(), rustc_errors[5ec552083a70b038]::ErrorGuaranteed>>
  36:     0x7f72482a6bdf - std[778605c8b9d7d939]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8ca32d3d634051a7]::util::run_in_thread_pool_with_globals<rustc_interface[8ca32d3d634051a7]::interface::run_compiler<core[cd150e9e29982b29]::result::Result<(), rustc_errors[5ec552083a70b038]::ErrorGuaranteed>, rustc_driver[72308b4096139644]::run_compiler::{closure#1}>::{closure#0}, core[cd150e9e29982b29]::result::Result<(), rustc_errors[5ec552083a70b038]::ErrorGuaranteed>>::{closure#0}, core[cd150e9e29982b29]::result::Result<(), rustc_errors[5ec552083a70b038]::ErrorGuaranteed>>
  37:     0x7f72489e53e9 - <<std[778605c8b9d7d939]::thread::Builder>::spawn_unchecked_<rustc_interface[8ca32d3d634051a7]::util::run_in_thread_pool_with_globals<rustc_interface[8ca32d3d634051a7]::interface::run_compiler<core[cd150e9e29982b29]::result::Result<(), rustc_errors[5ec552083a70b038]::ErrorGuaranteed>, rustc_driver[72308b4096139644]::run_compiler::{closure#1}>::{closure#0}, core[cd150e9e29982b29]::result::Result<(), rustc_errors[5ec552083a70b038]::ErrorGuaranteed>>::{closure#0}, core[cd150e9e29982b29]::result::Result<(), rustc_errors[5ec552083a70b038]::ErrorGuaranteed>>::{closure#1} as core[cd150e9e29982b29]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f724637a193 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h1a098e41f44eb0a6
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/alloc/src/boxed.rs:1935:9
  39:     0x7f724637a193 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hc5709f2569386b76
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/alloc/src/boxed.rs:1935:9
  40:     0x7f724637a193 - std::sys::unix::thread::Thread::new::thread_start::h0444ab80c205eb75
                               at /rustc/f8588549c3c3d45c32b404210cada01e2a45def3/library/std/src/sys/unix/thread.rs:108:17
  41:     0x7f724608c54d - <unknown>
  42:     0x7f7246111874 - clone
  43:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (f8588549c 2022-07-18) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
