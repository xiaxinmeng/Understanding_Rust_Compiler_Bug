
stack backtrace:
   0:     0x7f16fbab4670 - std::backtrace_rs::backtrace::libunwind::trace::h69aabb1662a6e4ce
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7f16fbab4670 - std::backtrace_rs::backtrace::trace_unsynchronized::h1e116db256b46f83
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f16fbab4670 - std::sys_common::backtrace::_print_fmt::hdefba6b7fbae0b1b
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f16fbab4670 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hea741479ddca7d34
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f16fbb1058e - core::fmt::write::hefdcf5f70ca57d71
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/core/src/fmt/mod.rs:1209:17
   5:     0x7f16fbaa48e5 - std::io::Write::write_fmt::h3acbe8cd910c2045
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/std/src/io/mod.rs:1682:15
   6:     0x7f16fbab4435 - std::sys_common::backtrace::_print::hb1e998793fbd1f01
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f16fbab4435 - std::sys_common::backtrace::print::h38f8e678fcca0385
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f16fbab723f - std::panicking::default_hook::{{closure}}::h00312d67e48fe9b7
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/std/src/panicking.rs:267:22
   9:     0x7f16fbab6f7a - std::panicking::default_hook::hf51b089ad6e46090
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/std/src/panicking.rs:286:9
  10:     0x7f16fe3fe4a4 - rustc_driver[c9b2b62ac6c06e3e]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f16fbab7a69 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hb12a8c20222b8dca
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/alloc/src/boxed.rs:2001:9
  12:     0x7f16fbab7a69 - std::panicking::rust_panic_with_hook::h4554d3cfa7a292a0
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/std/src/panicking.rs:692:13
  13:     0x7f16fbab77a1 - std::panicking::begin_panic_handler::{{closure}}::h7ae3e772349c68b5
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/std/src/panicking.rs:577:13
  14:     0x7f16fbab4b1c - std::sys_common::backtrace::__rust_end_short_backtrace::ha35716d93f46f9b2
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/std/src/sys_common/backtrace.rs:137:18
  15:     0x7f16fbab7502 - rust_begin_unwind
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/std/src/panicking.rs:575:5
  16:     0x7f16fbb0cf73 - core::panicking::panic_fmt::h7aaf174051fc1580
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/core/src/panicking.rs:65:14
  17:     0x7f16fbb0d04d - core::panicking::panic::ha7b14096bd116fb3
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/core/src/panicking.rs:115:5
  18:     0x7f16fda98237 - rustc_monomorphize[9f50eeaaa090c51c]::collector::collect_roots
  19:     0x7f16fda8eab9 - <rustc_session[2bf63b20a2edd454]::session::Session>::time::<alloc[5be6e33961c043f5]::vec::Vec<rustc_middle[bffa1ce1c8eb01fd]::mir::mono::MonoItem>, rustc_monomorphize[9f50eeaaa090c51c]::collector::collect_crate_mono_items::{closure#0}>
  20:     0x7f16fda8e881 - rustc_monomorphize[9f50eeaaa090c51c]::collector::collect_crate_mono_items
  21:     0x7f16fda8d864 - rustc_monomorphize[9f50eeaaa090c51c]::partitioning::collect_and_partition_mono_items
  22:     0x7f16fe160493 - rustc_query_system[61d798de3920a684]::query::plumbing::try_execute_query::<rustc_query_impl[e483c33c9eb49cbf]::plumbing::QueryCtxt, rustc_query_system[61d798de3920a684]::query::caches::DefaultCache<(), (&std[2479ed694dc13462]::collections::hash::set::HashSet<rustc_span[108591b54e49890d]::def_id::DefId, core[be439ec03a8fafe]::hash::BuildHasherDefault<rustc_hash[2088c137cd470b5b]::FxHasher>>, &[rustc_middle[bffa1ce1c8eb01fd]::mir::mono::CodegenUnit])>>
  23:     0x7f16fe16018b - rustc_query_system[61d798de3920a684]::query::plumbing::get_query::<rustc_query_impl[e483c33c9eb49cbf]::queries::collect_and_partition_mono_items, rustc_query_impl[e483c33c9eb49cbf]::plumbing::QueryCtxt>
  24:     0x7f16fe1600ce - <rustc_query_impl[e483c33c9eb49cbf]::Queries as rustc_middle[bffa1ce1c8eb01fd]::ty::query::QueryEngine>::collect_and_partition_mono_items
  25:     0x7f16fde886b2 - rustc_codegen_ssa[3e6029d997b755fa]::base::codegen_crate::<rustc_codegen_llvm[dc2a2cd560d41bc0]::LlvmCodegenBackend>
  26:     0x7f16fde88482 - <rustc_codegen_llvm[dc2a2cd560d41bc0]::LlvmCodegenBackend as rustc_codegen_ssa[3e6029d997b755fa]::traits::backend::CodegenBackend>::codegen_crate
  27:     0x7f16fd17058c - <rustc_session[2bf63b20a2edd454]::session::Session>::time::<alloc[5be6e33961c043f5]::boxed::Box<dyn core[be439ec03a8fafe]::any::Any>, rustc_interface[776b8bf765eb844b]::passes::start_codegen::{closure#0}>
  28:     0x7f16fd16feaf - <rustc_interface[776b8bf765eb844b]::passes::QueryContext>::enter::<<rustc_interface[776b8bf765eb844b]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[be439ec03a8fafe]::result::Result<alloc[5be6e33961c043f5]::boxed::Box<dyn core[be439ec03a8fafe]::any::Any>, rustc_errors[6e793e5777c0e940]::ErrorGuaranteed>>
  29:     0x7f16fd167c43 - <rustc_interface[776b8bf765eb844b]::queries::Queries>::ongoing_codegen
  30:     0x7f16fd166cb9 - <rustc_interface[776b8bf765eb844b]::interface::Compiler>::enter::<rustc_driver[c9b2b62ac6c06e3e]::run_compiler::{closure#1}::{closure#2}, core[be439ec03a8fafe]::result::Result<core[be439ec03a8fafe]::option::Option<rustc_interface[776b8bf765eb844b]::queries::Linker>, rustc_errors[6e793e5777c0e940]::ErrorGuaranteed>>
  31:     0x7f16fd15e64c - rustc_span[108591b54e49890d]::with_source_map::<core[be439ec03a8fafe]::result::Result<(), rustc_errors[6e793e5777c0e940]::ErrorGuaranteed>, rustc_interface[776b8bf765eb844b]::interface::run_compiler<core[be439ec03a8fafe]::result::Result<(), rustc_errors[6e793e5777c0e940]::ErrorGuaranteed>, rustc_driver[c9b2b62ac6c06e3e]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  32:     0x7f16fd15e012 - <scoped_tls[86b89047439588fa]::ScopedKey<rustc_span[108591b54e49890d]::SessionGlobals>>::set::<rustc_interface[776b8bf765eb844b]::interface::run_compiler<core[be439ec03a8fafe]::result::Result<(), rustc_errors[6e793e5777c0e940]::ErrorGuaranteed>, rustc_driver[c9b2b62ac6c06e3e]::run_compiler::{closure#1}>::{closure#0}, core[be439ec03a8fafe]::result::Result<(), rustc_errors[6e793e5777c0e940]::ErrorGuaranteed>>
  33:     0x7f16fd15c73f - std[2479ed694dc13462]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[776b8bf765eb844b]::util::run_in_thread_pool_with_globals<rustc_interface[776b8bf765eb844b]::interface::run_compiler<core[be439ec03a8fafe]::result::Result<(), rustc_errors[6e793e5777c0e940]::ErrorGuaranteed>, rustc_driver[c9b2b62ac6c06e3e]::run_compiler::{closure#1}>::{closure#0}, core[be439ec03a8fafe]::result::Result<(), rustc_errors[6e793e5777c0e940]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[be439ec03a8fafe]::result::Result<(), rustc_errors[6e793e5777c0e940]::ErrorGuaranteed>>
  34:     0x7f16fd15c5af - <<std[2479ed694dc13462]::thread::Builder>::spawn_unchecked_<rustc_interface[776b8bf765eb844b]::util::run_in_thread_pool_with_globals<rustc_interface[776b8bf765eb844b]::interface::run_compiler<core[be439ec03a8fafe]::result::Result<(), rustc_errors[6e793e5777c0e940]::ErrorGuaranteed>, rustc_driver[c9b2b62ac6c06e3e]::run_compiler::{closure#1}>::{closure#0}, core[be439ec03a8fafe]::result::Result<(), rustc_errors[6e793e5777c0e940]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[be439ec03a8fafe]::result::Result<(), rustc_errors[6e793e5777c0e940]::ErrorGuaranteed>>::{closure#1} as core[be439ec03a8fafe]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7f16fbac1583 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h779641ff7afa8556
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/alloc/src/boxed.rs:1987:9
  36:     0x7f16fbac1583 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hc2bdaa059464d4ed
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/alloc/src/boxed.rs:1987:9
  37:     0x7f16fbac1583 - std::sys::unix::thread::Thread::new::thread_start::h1f9fd65c81f5df69
                               at /rustc/6e95b6da885f42a4e1314595089fa4295e329d11/library/std/src/sys/unix/thread.rs:108:17
  38:     0x7f16fb98beae - start_thread
                               at /builddir/glibc-2.32/nptl/pthread_create.c:463:8
  39:     0x7f16fb8bb2ff - __GI___clone
                               at /builddir/glibc-2.32/misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:95
  40:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (6e95b6da8 2022-10-22) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -Z force-unstable-if-unmarked -C passes=sancov-module -C llvm-args=-sanitizer-coverage-level=4 -C llvm-args=-sanitizer-coverage-trace-compares -C llvm-args=-sanitizer-coverage-inline-8bit-counters -C llvm-args=-sanitizer-coverage-pc-table -C link-dead-code -Z sanitizer=address -C llvm-args=-sanitizer-coverage-stack-depth -C debug-assertions -C codegen-units=1 -Z sanitizer=address

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
