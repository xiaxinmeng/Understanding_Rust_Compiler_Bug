
thread 'rustc' panicked at 'assertion failed: i < this.fields.count()', /rustc/ad442399756573dccacb314b6bf8079964bcc72a/compiler/rustc_middle/src/ty/layout.rs:2290:21
stack backtrace:
   0:     0x7f699ab3b13c - std::backtrace_rs::backtrace::libunwind::trace::h793e05efd273d0f4
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f699ab3b13c - std::backtrace_rs::backtrace::trace_unsynchronized::h640b7b86ff610c77
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f699ab3b13c - std::sys_common::backtrace::_print_fmt::h362fa2a4f354f877
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7f699ab3b13c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf439e5ed84c74abd
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7f699ab9837c - core::fmt::write::h72801a82c94e6ff1
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/core/src/fmt/mod.rs:1149:17
   5:     0x7f699ab2b8d5 - std::io::Write::write_fmt::h5562a8b6da0f0339
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/std/src/io/mod.rs:1697:15
   6:     0x7f699ab3e390 - std::sys_common::backtrace::_print::hb29ddd998d02631c
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7f699ab3e390 - std::sys_common::backtrace::print::h81965e3d7c90fbb6
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7f699ab3e390 - std::panicking::default_hook::{{closure}}::h84db205ab6674b38
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/std/src/panicking.rs:211:50
   9:     0x7f699ab3df3b - std::panicking::default_hook::h1bf8bb4159936bca
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/std/src/panicking.rs:228:9
  10:     0x7f699b2d4781 - rustc_driver[e620c7401644acc4]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f699ab3eba9 - std::panicking::rust_panic_with_hook::hf8e86850fbbd03b1
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/std/src/panicking.rs:610:17
  12:     0x7f699ab3e632 - std::panicking::begin_panic_handler::{{closure}}::h590a0d6060ff866e
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/std/src/panicking.rs:500:13
  13:     0x7f699ab3b5f4 - std::sys_common::backtrace::__rust_end_short_backtrace::h260b8bd1c848a03c
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/std/src/sys_common/backtrace.rs:139:18
  14:     0x7f699ab3e5c9 - rust_begin_unwind
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/std/src/panicking.rs:498:5
  15:     0x7f699ab03631 - core::panicking::panic_fmt::h7b8580d81fcbbacd
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/core/src/panicking.rs:106:14
  16:     0x7f699ab0357d - core::panicking::panic::h50b51d19800453c0
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/core/src/panicking.rs:47:5
  17:     0x7f699c52bbf7 - <&rustc_middle[93690e4789d7fe1d]::ty::TyS as rustc_target[819931b88a693851]::abi::TyAbiInterface<_>>::ty_and_layout_field::field_ty_or_layout::<rustc_codegen_llvm[3ea7e54ed77cfea0]::context::CodegenCx>
  18:     0x7f699c55479c - <rustc_target[819931b88a693851]::abi::TyAndLayout<&rustc_middle[93690e4789d7fe1d]::ty::TyS> as rustc_codegen_llvm[3ea7e54ed77cfea0]::type_of::LayoutLlvmExt>::scalar_pair_element_llvm_type
  19:     0x7f699c55509e - <rustc_target[819931b88a693851]::abi::TyAndLayout<&rustc_middle[93690e4789d7fe1d]::ty::TyS> as rustc_codegen_llvm[3ea7e54ed77cfea0]::type_of::LayoutLlvmExt>::scalar_pair_element_llvm_type
  20:     0x7f699c53e4f4 - <rustc_codegen_llvm[3ea7e54ed77cfea0]::context::CodegenCx as rustc_codegen_ssa[f6374deef2cbb8fd]::traits::declare::PreDefineMethods>::predefine_fn
  21:     0x7f699c54ce3f - rustc_codegen_llvm[3ea7e54ed77cfea0]::base::compile_codegen_unit::module_codegen
  22:     0x7f699d1d2797 - <rustc_query_system[efb192c4209e4e11]::dep_graph::graph::DepGraph<rustc_middle[93690e4789d7fe1d]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[93690e4789d7fe1d]::ty::context::TyCtxt, rustc_span[2d5555579096f1fe]::symbol::Symbol, rustc_codegen_ssa[f6374deef2cbb8fd]::ModuleCodegen<rustc_codegen_llvm[3ea7e54ed77cfea0]::ModuleLlvm>>
  23:     0x7f699d208dd6 - rustc_codegen_llvm[3ea7e54ed77cfea0]::base::compile_codegen_unit
  24:     0x7f699d1e8e58 - <rustc_codegen_llvm[3ea7e54ed77cfea0]::LlvmCodegenBackend as rustc_codegen_ssa[f6374deef2cbb8fd]::traits::backend::CodegenBackend>::codegen_crate
  25:     0x7f699d1a4dc7 - <rustc_session[cec017cef00f19c9]::session::Session>::time::<alloc[9a4bc13598ff604f]::boxed::Box<dyn core[cc79c391059f8e46]::any::Any>, rustc_interface[f89f8228a4e35bc7]::passes::start_codegen::{closure#0}>
  26:     0x7f699d1848d3 - <rustc_interface[f89f8228a4e35bc7]::passes::QueryContext>::enter::<<rustc_interface[f89f8228a4e35bc7]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[cc79c391059f8e46]::result::Result<alloc[9a4bc13598ff604f]::boxed::Box<dyn core[cc79c391059f8e46]::any::Any>, rustc_errors[c8a333c965fedc03]::ErrorReported>>
  27:     0x7f699d17f34f - <rustc_interface[f89f8228a4e35bc7]::queries::Queries>::ongoing_codegen
  28:     0x7f699d163667 - <rustc_interface[f89f8228a4e35bc7]::interface::Compiler>::enter::<rustc_driver[e620c7401644acc4]::run_compiler::{closure#1}::{closure#2}, core[cc79c391059f8e46]::result::Result<core[cc79c391059f8e46]::option::Option<rustc_interface[f89f8228a4e35bc7]::queries::Linker>, rustc_errors[c8a333c965fedc03]::ErrorReported>>
  29:     0x7f699d1545cd - rustc_span[2d5555579096f1fe]::with_source_map::<core[cc79c391059f8e46]::result::Result<(), rustc_errors[c8a333c965fedc03]::ErrorReported>, rustc_interface[f89f8228a4e35bc7]::interface::create_compiler_and_run<core[cc79c391059f8e46]::result::Result<(), rustc_errors[c8a333c965fedc03]::ErrorReported>, rustc_driver[e620c7401644acc4]::run_compiler::{closure#1}>::{closure#1}>
  30:     0x7f699d174b7a - rustc_interface[f89f8228a4e35bc7]::interface::create_compiler_and_run::<core[cc79c391059f8e46]::result::Result<(), rustc_errors[c8a333c965fedc03]::ErrorReported>, rustc_driver[e620c7401644acc4]::run_compiler::{closure#1}>
  31:     0x7f699d157f7b - <scoped_tls[3fea4c3dcac147b1]::ScopedKey<rustc_span[2d5555579096f1fe]::SessionGlobals>>::set::<rustc_interface[f89f8228a4e35bc7]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[f89f8228a4e35bc7]::interface::run_compiler<core[cc79c391059f8e46]::result::Result<(), rustc_errors[c8a333c965fedc03]::ErrorReported>, rustc_driver[e620c7401644acc4]::run_compiler::{closure#1}>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[c8a333c965fedc03]::ErrorReported>>::{closure#0}::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[c8a333c965fedc03]::ErrorReported>>
  32:     0x7f699d157095 - std[a5529df289459975]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f89f8228a4e35bc7]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[f89f8228a4e35bc7]::interface::run_compiler<core[cc79c391059f8e46]::result::Result<(), rustc_errors[c8a333c965fedc03]::ErrorReported>, rustc_driver[e620c7401644acc4]::run_compiler::{closure#1}>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[c8a333c965fedc03]::ErrorReported>>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[c8a333c965fedc03]::ErrorReported>>
  33:     0x7f699d153162 - <<std[a5529df289459975]::thread::Builder>::spawn_unchecked<rustc_interface[f89f8228a4e35bc7]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[f89f8228a4e35bc7]::interface::run_compiler<core[cc79c391059f8e46]::result::Result<(), rustc_errors[c8a333c965fedc03]::ErrorReported>, rustc_driver[e620c7401644acc4]::run_compiler::{closure#1}>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[c8a333c965fedc03]::ErrorReported>>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[c8a333c965fedc03]::ErrorReported>>::{closure#1} as core[cc79c391059f8e46]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7f699ab49e93 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h771719d52c343434
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/alloc/src/boxed.rs:1694:9
  35:     0x7f699ab49e93 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hf441746dfa4b0f57
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/alloc/src/boxed.rs:1694:9
  36:     0x7f699ab49e93 - std::sys::unix::thread::Thread::new::thread_start::hfd168f9d312b29ca
                               at /rustc/ad442399756573dccacb314b6bf8079964bcc72a/library/std/src/sys/unix/thread.rs:106:17
  37:     0x7f699a8ec927 - start_thread
                               at ./nptl/./nptl/pthread_create.c:435:8
  38:     0x7f699a97c9e4 - __clone
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:100
  39:                0x0 - <unknown>
