
thread 'rustc' panicked at 'assertion failed: !instance.substs.needs_infer()', compiler/rustc_codegen_llvm/src/mono_item.rs:53:9
stack backtrace:
   0:     0x7f711a89ba9c - std::backtrace_rs::backtrace::libunwind::trace::hf6a6dfd7da937cb0
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7f711a89ba9c - std::backtrace_rs::backtrace::trace_unsynchronized::hc596a19e4891f7f3
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f711a89ba9c - std::sys_common::backtrace::_print_fmt::hb16700db31584325
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7f711a89ba9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h231c4190cfa75162
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7f711a8f8fdc - core::fmt::write::h2a1462b5f8eea807
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/fmt/mod.rs:1163:17
   5:     0x7f711a88bc05 - std::io::Write::write_fmt::h71ddfebc68685972
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/io/mod.rs:1696:15
   6:     0x7f711a89ef60 - std::sys_common::backtrace::_print::hcc197d4bebf2b369
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7f711a89ef60 - std::sys_common::backtrace::print::h335a66af06738c7c
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7f711a89ef60 - std::panicking::default_hook::{{closure}}::h6fac9ac9c8b79e52
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:210:50
   9:     0x7f711a89eb15 - std::panicking::default_hook::h341c1030c6a1161b
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:227:9
  10:     0x7f711b11b2b1 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::h932547f60770f26a
  11:     0x7f711a89f779 - std::panicking::rust_panic_with_hook::h50680ff4b44510c6
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:628:17
  12:     0x7f711a89f202 - std::panicking::begin_panic_handler::{{closure}}::h9371c0fbb1e8465a
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:519:13
  13:     0x7f711a89bf44 - std::sys_common::backtrace::__rust_end_short_backtrace::h9b3efa22a5768c0f
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:139:18
  14:     0x7f711a89f199 - rust_begin_unwind
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:517:5
  15:     0x7f711a863441 - core::panicking::panic_fmt::h23b9203e89cc61cf
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:100:14
  16:     0x7f711a86338d - core::panicking::panic::h0ba7146865b2f9d6
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:50:5
  17:     0x7f711c3655a2 - rustc_codegen_llvm::mono_item::<impl rustc_codegen_ssa::traits::declare::PreDefineMethods for rustc_codegen_llvm::context::CodegenCx>::predefine_fn::h1c521c4ac07b4fbd
  18:     0x7f711c36cf31 - rustc_codegen_llvm::base::compile_codegen_unit::module_codegen::h16153c4046d114f1
  19:     0x7f711cd826a5 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task::h789e443a348261c1
  20:     0x7f711cdc67f3 - rustc_codegen_llvm::base::compile_codegen_unit::hab1bf03723596068
  21:     0x7f711cd8dccb - rustc_codegen_ssa::base::codegen_crate::h7a10ba1a5bbbde6f
  22:     0x7f711cdbae7a - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::ha56c5fc04fb5674a
  23:     0x7f711cd2bf7d - rustc_interface::queries::Queries::ongoing_codegen::h11dabf965b22130b
  24:     0x7f711cd009aa - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hf84cd18c24bd5171
  25:     0x7f711ccee0ee - rustc_span::with_source_map::h6ab8a240e103b5b9
  26:     0x7f711cd002ac - scoped_tls::ScopedKey<T>::set::hd1fbd64c6f645895
  27:     0x7f711cceeef5 - std::sys_common::backtrace::__rust_begin_short_backtrace::h0a1328c9fa7f7448
  28:     0x7f711cd1a962 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h4ea1ced06d6b3e97
  29:     0x7f711a8aa933 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h7bd677a5dc988be6
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/alloc/src/boxed.rs:1691:9
  30:     0x7f711a8aa933 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h7b1c1ba11c4db785
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/alloc/src/boxed.rs:1691:9
  31:     0x7f711a8aa933 - std::sys::unix::thread::Thread::new::thread_start::h9c58c0d12d84e854
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys/unix/thread.rs:106:17
  32:     0x7f711a6835c2 - start_thread
  33:     0x7f711a708584 - __clone
  34:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0 (f1edd0429 2021-11-29) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib

query stack during panic:
end of query stack
