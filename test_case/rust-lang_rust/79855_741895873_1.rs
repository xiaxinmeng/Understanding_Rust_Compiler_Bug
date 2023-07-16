
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_middle/src/ty/query/mod.rs:235:5
stack backtrace:
   0:     0x7f96fa8816a7 - std::backtrace_rs::backtrace::libunwind::trace::h746c3e9529d524bc
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7f96fa8816a7 - std::backtrace_rs::backtrace::trace_unsynchronized::h86340908ff889faa
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f96fa8816a7 - std::sys_common::backtrace::_print_fmt::h43f85f9b18230404
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7f96fa8816a7 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc132ae1a5b5aa7cd
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7f96fa8f551c - core::fmt::write::hdf023a0036d2a25f
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/core/src/fmt/mod.rs:1078:17
   5:     0x7f96fa873692 - std::io::Write::write_fmt::h8580846154bcb66a
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/io/mod.rs:1519:15
   6:     0x7f96fa8853a5 - std::sys_common::backtrace::_print::h7ee55fed88d107a3
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7f96fa8853a5 - std::sys_common::backtrace::print::h54a7d3e52a524177
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7f96fa8853a5 - std::panicking::default_hook::{{closure}}::h60921e857bf55a40
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/panicking.rs:208:50
   9:     0x7f96fa884efa - std::panicking::default_hook::hf0f9afb1017317fc
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/panicking.rs:225:9
  10:     0x7f96fb13ca68 - rustc_driver::report_ice::hff78d76a39ffbb86
  11:     0x7f96dcd82a66 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hd8394cbae821e63e
                               at /home/stefanp/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:1342:9
  12:     0x7f96dcdb68bb - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::hf6a1e776581f2f5c
                               at /home/stefanp/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:320:21
  13:     0x7f96fa885ca6 - std::panicking::rust_panic_with_hook::h8d66bf42b407aaea
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/panicking.rs:595:17
  14:     0x7f96fa885797 - std::panicking::begin_panic_handler::{{closure}}::hde71edcd925d0c5e
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/panicking.rs:495:13
  15:     0x7f96fa881b6c - std::sys_common::backtrace::__rust_end_short_backtrace::h8a3c7d6cea578919
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/sys_common/backtrace.rs:141:18
  16:     0x7f96fa885729 - rust_begin_unwind
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/panicking.rs:493:5
  17:     0x7f96fa8f1931 - core::panicking::panic_fmt::h20225113c4a2f8fd
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/core/src/panicking.rs:92:14
  18:     0x7f96fa8f187d - core::panicking::panic::h35b77276aa4b0c12
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/core/src/panicking.rs:50:5
  19:     0x7f96fdb1c835 - rustc_middle::ty::query::try_load_from_on_disk_cache::hfa4775df5c5e0180
  20:     0x7f96fcdf73ca - rustc_query_system::dep_graph::graph::DepGraph<K>::exec_cache_promotions::h8caa69177622351e
  21:     0x7f96fce0fc21 - rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::hab5675af7737b3e6
  22:     0x7f96fcdb33bb - rustc_incremental::persist::save::save_in::h7f4f43356280dc0a
  23:     0x7f96fcdae1d1 - rustc_data_structures::sync::join::h0120008f852881ce
  24:     0x7f96fce0ecc2 - rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::h2893ce60eec57bb6
  25:     0x7f96fcdb28ef - rustc_incremental::persist::save::save_dep_graph::hffd3fb2ecc639a78
  26:     0x7f96fcc8ddaa - rustc_codegen_ssa::base::finalize_tcx::h68106e2729498b3e
  27:     0x7f96fb5ab457 - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::h067fffb3870bc5b0
  28:     0x7f96fb35ab2e - rustc_session::utils::<impl rustc_session::session::Session>::time::had158f21ec5bf4d1
  29:     0x7f96fb39d82c - rustc_interface::passes::QueryContext::enter::h40067ad7feabcbd0
  30:     0x7f96fb3f5c93 - rustc_interface::queries::Queries::ongoing_codegen::h4fc36fc05972247d
  31:     0x7f96fb0e56e2 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hd899306a06575d0c
  32:     0x7f96fb1787f7 - rustc_span::with_source_map::ha4e07ff263d0dc1d
  33:     0x7f96fb0e688b - rustc_interface::interface::create_compiler_and_run::h1d6d732867d1f489
  34:     0x7f96fb192d60 - scoped_tls::ScopedKey<T>::set::h39c0aa543118d3f3
  35:     0x7f96fb199546 - std::sys_common::backtrace::__rust_begin_short_backtrace::h1e5aa72fb9cd6d86
  36:     0x7f96fb0ee27a - core::ops::function::FnOnce::call_once{{vtable.shim}}::hc793837e985b77ce
  37:     0x7f96fa89565a - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hea1090dbdcecbf5a
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/alloc/src/boxed.rs:1328:9
  38:     0x7f96fa89565a - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h8d5723d3912bd325
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/alloc/src/boxed.rs:1328:9
  39:     0x7f96fa89565a - std::sys::unix::thread::Thread::new::thread_start::hc17a425ca2995724
                               at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/sys/unix/thread.rs:71:17
  40:     0x7f96fa79d3e9 - start_thread
  41:     0x7f96fa6ba293 - __GI___clone
  42:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-nightly (3d6705aa5 2020-12-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
