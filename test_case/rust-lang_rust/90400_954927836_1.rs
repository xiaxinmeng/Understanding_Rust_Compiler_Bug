
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: ErrorReported', compiler/rustc_monomorphize/src/collector.rs:894:84
stack backtrace:
   0: rust_begin_unwind
             at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/std/src/panicking.rs:495:5
   1: core::panicking::panic_fmt
             at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/core/src/panicking.rs:106:14
   2: core::result::unwrap_failed
             at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/core/src/result.rs:1617:5
   3: rustc_monomorphize::collector::collect_neighbours
   4: rustc_monomorphize::collector::collect_items_rec
   5: rustc_monomorphize::collector::collect_items_rec
   6: <rustc_session::session::Session>::time::<(), rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}>
   7: rustc_monomorphize::collector::collect_crate_mono_items
   8: rustc_monomorphize::partitioning::collect_and_partition_mono_items
   9: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>>
  10: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>
  11: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items
  12: rustc_codegen_ssa::base::codegen_crate::<rustc_codegen_llvm::LlvmCodegenBackend>
  13: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  14: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
  15: <rustc_interface::passes::QueryContext>::enter::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>
  16: <rustc_interface::queries::Queries>::ongoing_codegen
  17: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  18: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}>
  19: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (c390d69a6 2021-10-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: internal compiler error: Encountered error `Unimplemented` selecting `Binder(<u32 as Bar>, [])` during codegen
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/codegen.rs:68:32

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1167:13
stack backtrace:
   0:     0x7f8aec4c076c - std::backtrace_rs::backtrace::libunwind::trace::hc6c3491277866fea
                               at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f8aec4c076c - std::backtrace_rs::backtrace::trace_unsynchronized::h4524f073368a5b13
                               at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f8aec4c076c - std::sys_common::backtrace::_print_fmt::h0d0cace6159902af
                               at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7f8aec4c076c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3e6af6f05919a7fc
                               at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7f8aec51d9ac - core::fmt::write::h72801a82c94e6ff1
                               at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/core/src/fmt/mod.rs:1149:17
   5:     0x7f8aec4b0e95 - std::io::Write::write_fmt::ha4f5d34aaccbac84
                               at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/std/src/io/mod.rs:1696:15
   6:     0x7f8aec4c39c0 - std::sys_common::backtrace::_print::heed69f5ce9a8e189
                               at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7f8aec4c39c0 - std::sys_common::backtrace::print::h5f3918bd80c09252
                               at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7f8aec4c39c0 - std::panicking::default_hook::{{closure}}::h5af30648530eb3d0
                               at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/std/src/panicking.rs:210:50
   9:     0x7f8aec4c356b - std::panicking::default_hook::he88d5fb1ba1b4c19
                               at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/std/src/panicking.rs:227:9
  10:     0x7f8aecc57e31 - rustc_driver[98914a17e63058c]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f8aec4c41d9 - std::panicking::rust_panic_with_hook::h01febc308b2b313b
                               at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/std/src/panicking.rs:607:17
  12:     0x7f8aec4c3c90 - std::panicking::begin_panic_handler::{{closure}}::h24a6d13f5560b71f
                               at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/std/src/panicking.rs:499:13
  13:     0x7f8aec4c0c14 - std::sys_common::backtrace::__rust_end_short_backtrace::h3e2917f0da9fbc5c
                               at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/std/src/sys_common/backtrace.rs:139:18
  14:     0x7f8aec4c3bf9 - rust_begin_unwind
                               at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/std/src/panicking.rs:495:5
  15:     0x7f8aec489151 - core::panicking::panic_fmt::h7b8580d81fcbbacd
                               at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/core/src/panicking.rs:106:14
  16:     0x7f8aedd62bac - core[cc79c391059f8e46]::panicking::panic_display::<&str>
  17:     0x7f8aef446666 - <rustc_errors[932abaefb61eb634]::HandlerInner>::flush_delayed
  18:     0x7f8aef444d7d - <rustc_errors[932abaefb61eb634]::HandlerInner as core[cc79c391059f8e46]::ops::drop::Drop>::drop
  19:     0x7f8aeeb2a346 - core[cc79c391059f8e46]::ptr::drop_in_place::<rustc_session[3b6910ad19e800a6]::parse::ParseSess>
  20:     0x7f8aeeb2ca3a - <alloc[9a4bc13598ff604f]::rc::Rc<rustc_session[3b6910ad19e800a6]::session::Session> as core[cc79c391059f8e46]::ops::drop::Drop>::drop
  21:     0x7f8aeeb0786d - core[cc79c391059f8e46]::ptr::drop_in_place::<rustc_interface[71a95cb40f833645]::interface::Compiler>
  22:     0x7f8aeeb076e5 - rustc_span[b6a32fa5db97fd22]::with_source_map::<core[cc79c391059f8e46]::result::Result<(), rustc_errors[932abaefb61eb634]::ErrorReported>, rustc_interface[71a95cb40f833645]::interface::create_compiler_and_run<core[cc79c391059f8e46]::result::Result<(), rustc_errors[932abaefb61eb634]::ErrorReported>, rustc_driver[98914a17e63058c]::run_compiler::{closure#1}>::{closure#0}>
  23:     0x7f8aeeb179c0 - <scoped_tls[3fea4c3dcac147b1]::ScopedKey<rustc_span[b6a32fa5db97fd22]::SessionGlobals>>::set::<rustc_interface[71a95cb40f833645]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[71a95cb40f833645]::interface::run_compiler<core[cc79c391059f8e46]::result::Result<(), rustc_errors[932abaefb61eb634]::ErrorReported>, rustc_driver[98914a17e63058c]::run_compiler::{closure#1}>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[932abaefb61eb634]::ErrorReported>>::{closure#0}::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[932abaefb61eb634]::ErrorReported>>
  24:     0x7f8aeeb09a15 - std[fcea40badc263c8f]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[71a95cb40f833645]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[71a95cb40f833645]::interface::run_compiler<core[cc79c391059f8e46]::result::Result<(), rustc_errors[932abaefb61eb634]::ErrorReported>, rustc_driver[98914a17e63058c]::run_compiler::{closure#1}>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[932abaefb61eb634]::ErrorReported>>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[932abaefb61eb634]::ErrorReported>>
  25:     0x7f8aeeb2c0a2 - <<std[fcea40badc263c8f]::thread::Builder>::spawn_unchecked<rustc_interface[71a95cb40f833645]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[71a95cb40f833645]::interface::run_compiler<core[cc79c391059f8e46]::result::Result<(), rustc_errors[932abaefb61eb634]::ErrorReported>, rustc_driver[98914a17e63058c]::run_compiler::{closure#1}>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[932abaefb61eb634]::ErrorReported>>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[932abaefb61eb634]::ErrorReported>>::{closure#1} as core[cc79c391059f8e46]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  26:     0x7f8aec4cf4c3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hd81bd86213781012
                               at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/alloc/src/boxed.rs:1691:9
  27:     0x7f8aec4cf4c3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h7b3e346f5d8f6d6a
                               at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/alloc/src/boxed.rs:1691:9
  28:     0x7f8aec4cf4c3 - std::sys::unix::thread::Thread::new::thread_start::ha575792f17151d60
                               at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/std/src/sys/unix/thread.rs:106:17
  29:     0x7f8aec408609 - start_thread
  30:     0x7f8aec31c293 - clone
  31:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (c390d69a6 2021-10-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
