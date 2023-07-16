plain
   Compiling tracing-attributes v0.1.22
   Compiling icu_provider_macros v1.0.0
   Compiling chalk-derive v0.87.0
   Compiling derive_more v0.99.17
error: internal compiler error: /checkout/compiler/rustc_codegen_ssa/src/mir/operand.rs:399:17: use of PlaceRef { local: _0, projection: [] } before def
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1603:9
stack backtrace:
   0:     0x7f1fca15f976 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hafd81a0b5d487c53
   1:     0x7f1fca1cdfb8 - core::fmt::write::h4a778e35e4f38324
   1:     0x7f1fca1cdfb8 - core::fmt::write::h4a778e35e4f38324
   2:     0x7f1fca150f31 - std::io::Write::write_fmt::ha216e80cfbe2a850
   3:     0x7f1fca15f745 - std::sys_common::backtrace::print::h1b9ed32e93c301a6
   4:     0x7f1fca162c87 - std::panicking::default_hook::{{closure}}::h1690eea152a3b836
   5:     0x7f1fca1629e6 - std::panicking::default_hook::h5e9cc4e411668de1
   6:     0x7f1fcab34d82 - rustc_driver[4f9bbda5eb597eb7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f1fca1635b3 - std::panicking::rust_panic_with_hook::h0e85789db00c4c44
   8:     0x7f1fcd63d4d3 - std[153e7eca4933144d]::panicking::begin_panic::<rustc_errors[bb1bee83f532b20d]::ExplicitBug>::{closure#0}
   9:     0x7f1fcd630516 - std[153e7eca4933144d]::sys_common::backtrace::__rust_end_short_backtrace::<std[153e7eca4933144d]::panicking::begin_panic<rustc_errors[bb1bee83f532b20d]::ExplicitBug>::{closure#0}, !>
  10:     0x7f1fcaad6876 - std[153e7eca4933144d]::panicking::begin_panic::<rustc_errors[bb1bee83f532b20d]::ExplicitBug>
  11:     0x7f1fcd62d9c6 - std[153e7eca4933144d]::panic::panic_any::<rustc_errors[bb1bee83f532b20d]::ExplicitBug>
  12:     0x7f1fcd62a01a - <rustc_errors[bb1bee83f532b20d]::HandlerInner>::bug::<&alloc[7b8a314fc46dcffe]::string::String>
  13:     0x7f1fcd629bd0 - <rustc_errors[bb1bee83f532b20d]::Handler>::bug::<&alloc[7b8a314fc46dcffe]::string::String>
  14:     0x7f1fcd6a1fb5 - rustc_middle[47749b6675615f27]::util::bug::opt_span_bug_fmt::<rustc_span[641299eb714c5b9d]::span_encoding::Span>::{closure#0}
  15:     0x7f1fcd6a0dac - rustc_middle[47749b6675615f27]::ty::context::tls::with_opt::<rustc_middle[47749b6675615f27]::util::bug::opt_span_bug_fmt<rustc_span[641299eb714c5b9d]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7f1fcd6a0d5e - rustc_middle[47749b6675615f27]::ty::context::tls::with_context_opt::<rustc_middle[47749b6675615f27]::ty::context::tls::with_opt<rustc_middle[47749b6675615f27]::util::bug::opt_span_bug_fmt<rustc_span[641299eb714c5b9d]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7f1fcd6a1ef9 - rustc_middle[47749b6675615f27]::util::bug::opt_span_bug_fmt::<rustc_span[641299eb714c5b9d]::span_encoding::Span>
  18:     0x7f1fcaad6825 - rustc_middle[47749b6675615f27]::util::bug::bug_fmt
  19:     0x7f1fcad6aa57 - <rustc_codegen_ssa[65cd6b1bb0c8ab35]::mir::FunctionCx<rustc_codegen_llvm[a86bd3243de919a2]::builder::Builder>>::codegen_consume
  20:     0x7f1fcad6fdca - <rustc_codegen_ssa[65cd6b1bb0c8ab35]::mir::FunctionCx<rustc_codegen_llvm[a86bd3243de919a2]::builder::Builder>>::codegen_return_terminator
  21:     0x7f1fcad59ac8 - rustc_codegen_ssa[65cd6b1bb0c8ab35]::mir::codegen_mir::<rustc_codegen_llvm[a86bd3243de919a2]::builder::Builder>
  22:     0x7f1fcad3273b - rustc_codegen_ssa[65cd6b1bb0c8ab35]::base::codegen_instance::<rustc_codegen_llvm[a86bd3243de919a2]::builder::Builder>
  23:     0x7f1fcae1d123 - <rustc_middle[47749b6675615f27]::mir::mono::MonoItem as rustc_codegen_ssa[65cd6b1bb0c8ab35]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[a86bd3243de919a2]::builder::Builder>
  24:     0x7f1fcad8373b - rustc_codegen_llvm[a86bd3243de919a2]::base::compile_codegen_unit::module_codegen
  25:     0x7f1fcae8a5d9 - <rustc_query_system[bc68f14b1498214f]::dep_graph::graph::DepGraph<rustc_middle[47749b6675615f27]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[47749b6675615f27]::ty::context::TyCtxt, rustc_span[641299eb714c5b9d]::symbol::Symbol, rustc_codegen_ssa[65cd6b1bb0c8ab35]::ModuleCodegen<rustc_codegen_llvm[a86bd3243de919a2]::ModuleLlvm>>
  26:     0x7f1fcad833a0 - rustc_codegen_llvm[a86bd3243de919a2]::base::compile_codegen_unit
  27:     0x7f1fcad31a4d - rustc_codegen_ssa[65cd6b1bb0c8ab35]::base::codegen_crate::<rustc_codegen_llvm[a86bd3243de919a2]::LlvmCodegenBackend>
  28:     0x7f1fcaea6250 - <rustc_codegen_llvm[a86bd3243de919a2]::LlvmCodegenBackend as rustc_codegen_ssa[65cd6b1bb0c8ab35]::traits::backend::CodegenBackend>::codegen_crate
  29:     0x7f1fcac8477f - <rustc_session[23ef6968998a22e0]::session::Session>::time::<alloc[7b8a314fc46dcffe]::boxed::Box<dyn core[cada5d954255392c]::any::Any>, rustc_interface[75661f553f767c4f]::passes::start_codegen::{closure#0}>
  30:     0x7f1fcac84171 - rustc_interface[75661f553f767c4f]::passes::start_codegen
  31:     0x7f1fcac822f4 - <rustc_interface[75661f553f767c4f]::passes::QueryContext>::enter::<<rustc_interface[75661f553f767c4f]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[cada5d954255392c]::result::Result<alloc[7b8a314fc46dcffe]::boxed::Box<dyn core[cada5d954255392c]::any::Any>, rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>
  32:     0x7f1fcace6afa - <rustc_interface[75661f553f767c4f]::queries::Queries>::ongoing_codegen
  33:     0x7f1fcaba0c17 - <rustc_interface[75661f553f767c4f]::interface::Compiler>::enter::<rustc_driver[4f9bbda5eb597eb7]::run_compiler::{closure#1}::{closure#2}, core[cada5d954255392c]::result::Result<core[cada5d954255392c]::option::Option<rustc_interface[75661f553f767c4f]::queries::Linker>, rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>
  34:     0x7f1fcab3644b - rustc_span[641299eb714c5b9d]::with_source_map::<core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>, rustc_interface[75661f553f767c4f]::interface::run_compiler<core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>, rustc_driver[4f9bbda5eb597eb7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  35:     0x7f1fcab999aa - <scoped_tls[5706bf77a85fe418]::ScopedKey<rustc_span[641299eb714c5b9d]::SessionGlobals>>::set::<rustc_interface[75661f553f767c4f]::interface::run_compiler<core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>, rustc_driver[4f9bbda5eb597eb7]::run_compiler::{closure#1}>::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>
  36:     0x7f1fcab930ca - std[153e7eca4933144d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[75661f553f767c4f]::util::run_in_thread_pool_with_globals<rustc_interface[75661f553f767c4f]::interface::run_compiler<core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>, rustc_driver[4f9bbda5eb597eb7]::run_compiler::{closure#1}>::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>
  37:     0x7f1fcab8c666 - std[153e7eca4933144d]::panic::catch_unwind::<core[cada5d954255392c]::panic::unwind_safe::AssertUnwindSafe<<std[153e7eca4933144d]::thread::Builder>::spawn_unchecked_<rustc_interface[75661f553f767c4f]::util::run_in_thread_pool_with_globals<rustc_interface[75661f553f767c4f]::interface::run_compiler<core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>, rustc_driver[4f9bbda5eb597eb7]::run_compiler::{closure#1}>::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>
  38:     0x7f1fcab3bde4 - <<std[153e7eca4933144d]::thread::Builder>::spawn_unchecked_<rustc_interface[75661f553f767c4f]::util::run_in_thread_pool_with_globals<rustc_interface[75661f553f767c4f]::interface::run_compiler<core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>, rustc_driver[4f9bbda5eb597eb7]::run_compiler::{closure#1}>::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>::{closure#1} as core[cada5d954255392c]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7f1fca170bee - std::sys::unix::thread::Thread::new::thread_start::h5d9f8d6d1fd775b4
  40:     0x7f1fc9f05b43 - <unknown>
  41:     0x7f1fc9f97a00 - <unknown>
  42:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.68.0-nightly (c865d26ec 2023-01-08) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type proc-macro -C prefer-dynamic -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -Z binary-dep-depinfo -Z unstable-options -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
