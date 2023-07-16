plain
   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
error: internal compiler error: compiler/rustc_codegen_ssa/src/mir/analyze.rs:308:21: funclet bb1526 has 2 parents - bb1168 and bb1525
     |
     |
2061 | /     pub fn from_json(obj: Json) -> Result<(Target, TargetWarnings), String> {
2062 | |         // While ugly, this code must remain this way to retain
2063 | |         // compatibility with existing JSON fields and the internal
...    |
2603 | |         ))
2604 | |     }
     | |_____^
     | |_____^

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:978:33
stack backtrace:
   0:     0x7f830e8d8976 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hafd81a0b5d487c53
   1:     0x7f830e946fb8 - core::fmt::write::h4a778e35e4f38324
   2:     0x7f830e8c9f31 - std::io::Write::write_fmt::ha216e80cfbe2a850
   3:     0x7f830e8d8745 - std::sys_common::backtrace::print::h1b9ed32e93c301a6
   4:     0x7f830e8dbc87 - std::panicking::default_hook::{{closure}}::h1690eea152a3b836
   5:     0x7f830e8db9e6 - std::panicking::default_hook::h5e9cc4e411668de1
   6:     0x7f830f2af342 - rustc_driver[4f9bbda5eb597eb7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f830e8dc5b3 - std::panicking::rust_panic_with_hook::h0e85789db00c4c44
   8:     0x7f83110e6393 - std[153e7eca4933144d]::panicking::begin_panic::<rustc_errors[bb1bee83f532b20d]::ExplicitBug>::{closure#0}
   9:     0x7f83110e5646 - std[153e7eca4933144d]::sys_common::backtrace::__rust_end_short_backtrace::<std[153e7eca4933144d]::panicking::begin_panic<rustc_errors[bb1bee83f532b20d]::ExplicitBug>::{closure#0}, !>
  10:     0x7f830f1e82c6 - std[153e7eca4933144d]::panicking::begin_panic::<rustc_errors[bb1bee83f532b20d]::ExplicitBug>
  11:     0x7f8311094be6 - std[153e7eca4933144d]::panic::panic_any::<rustc_errors[bb1bee83f532b20d]::ExplicitBug>
  12:     0x7f831109173b - <rustc_errors[bb1bee83f532b20d]::HandlerInner>::span_bug::<rustc_span[641299eb714c5b9d]::span_encoding::Span, &alloc[7b8a314fc46dcffe]::string::String>
  13:     0x7f8311091380 - <rustc_errors[bb1bee83f532b20d]::Handler>::span_bug::<rustc_span[641299eb714c5b9d]::span_encoding::Span, &alloc[7b8a314fc46dcffe]::string::String>
  14:     0x7f831113d695 - rustc_middle[47749b6675615f27]::util::bug::opt_span_bug_fmt::<rustc_span[641299eb714c5b9d]::span_encoding::Span>::{closure#0}
  15:     0x7f831113d70c - rustc_middle[47749b6675615f27]::ty::context::tls::with_opt::<rustc_middle[47749b6675615f27]::util::bug::opt_span_bug_fmt<rustc_span[641299eb714c5b9d]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7f831113bcce - rustc_middle[47749b6675615f27]::ty::context::tls::with_context_opt::<rustc_middle[47749b6675615f27]::ty::context::tls::with_opt<rustc_middle[47749b6675615f27]::util::bug::opt_span_bug_fmt<rustc_span[641299eb714c5b9d]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7f831113bb99 - rustc_middle[47749b6675615f27]::util::bug::opt_span_bug_fmt::<rustc_span[641299eb714c5b9d]::span_encoding::Span>
  18:     0x7f830f1e8747 - rustc_middle[47749b6675615f27]::util::bug::span_bug_fmt::<rustc_span[641299eb714c5b9d]::span_encoding::Span>
  19:     0x7f83110fa8ae - rustc_codegen_ssa[65cd6b1bb0c8ab35]::mir::analyze::cleanup_kinds::propagate::{closure#0}
  20:     0x7f83110fa192 - rustc_codegen_ssa[65cd6b1bb0c8ab35]::mir::analyze::cleanup_kinds
  21:     0x7f830f4d27b1 - rustc_codegen_ssa[65cd6b1bb0c8ab35]::mir::codegen_mir::<rustc_codegen_llvm[a86bd3243de919a2]::builder::Builder>
  22:     0x7f830f4acceb - rustc_codegen_ssa[65cd6b1bb0c8ab35]::base::codegen_instance::<rustc_codegen_llvm[a86bd3243de919a2]::builder::Builder>
  23:     0x7f830f597313 - <rustc_middle[47749b6675615f27]::mir::mono::MonoItem as rustc_codegen_ssa[65cd6b1bb0c8ab35]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[a86bd3243de919a2]::builder::Builder>
  24:     0x7f830f4fdd5b - rustc_codegen_llvm[a86bd3243de919a2]::base::compile_codegen_unit::module_codegen
  25:     0x7f830f604999 - <rustc_query_system[bc68f14b1498214f]::dep_graph::graph::DepGraph<rustc_middle[47749b6675615f27]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[47749b6675615f27]::ty::context::TyCtxt, rustc_span[641299eb714c5b9d]::symbol::Symbol, rustc_codegen_ssa[65cd6b1bb0c8ab35]::ModuleCodegen<rustc_codegen_llvm[a86bd3243de919a2]::ModuleLlvm>>
  26:     0x7f830f4fd9c0 - rustc_codegen_llvm[a86bd3243de919a2]::base::compile_codegen_unit
  27:     0x7f830f4abffd - rustc_codegen_ssa[65cd6b1bb0c8ab35]::base::codegen_crate::<rustc_codegen_llvm[a86bd3243de919a2]::LlvmCodegenBackend>
  28:     0x7f830f61fb10 - <rustc_codegen_llvm[a86bd3243de919a2]::LlvmCodegenBackend as rustc_codegen_ssa[65cd6b1bb0c8ab35]::traits::backend::CodegenBackend>::codegen_crate
  29:     0x7f830f3fed3f - <rustc_session[23ef6968998a22e0]::session::Session>::time::<alloc[7b8a314fc46dcffe]::boxed::Box<dyn core[cada5d954255392c]::any::Any>, rustc_interface[75661f553f767c4f]::passes::start_codegen::{closure#0}>
  30:     0x7f830f3fe731 - rustc_interface[75661f553f767c4f]::passes::start_codegen
  31:     0x7f830f3fc8b4 - <rustc_interface[75661f553f767c4f]::passes::QueryContext>::enter::<<rustc_interface[75661f553f767c4f]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[cada5d954255392c]::result::Result<alloc[7b8a314fc46dcffe]::boxed::Box<dyn core[cada5d954255392c]::any::Any>, rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>
  32:     0x7f830f4610ba - <rustc_interface[75661f553f767c4f]::queries::Queries>::ongoing_codegen
  33:     0x7f830f31b1d7 - <rustc_interface[75661f553f767c4f]::interface::Compiler>::enter::<rustc_driver[4f9bbda5eb597eb7]::run_compiler::{closure#1}::{closure#2}, core[cada5d954255392c]::result::Result<core[cada5d954255392c]::option::Option<rustc_interface[75661f553f767c4f]::queries::Linker>, rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>
  34:     0x7f830f2b0a0b - rustc_span[641299eb714c5b9d]::with_source_map::<core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>, rustc_interface[75661f553f767c4f]::interface::run_compiler<core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>, rustc_driver[4f9bbda5eb597eb7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  35:     0x7f830f313f6a - <scoped_tls[5706bf77a85fe418]::ScopedKey<rustc_span[641299eb714c5b9d]::SessionGlobals>>::set::<rustc_interface[75661f553f767c4f]::interface::run_compiler<core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>, rustc_driver[4f9bbda5eb597eb7]::run_compiler::{closure#1}>::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>
  36:     0x7f830f30d68a - std[153e7eca4933144d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[75661f553f767c4f]::util::run_in_thread_pool_with_globals<rustc_interface[75661f553f767c4f]::interface::run_compiler<core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>, rustc_driver[4f9bbda5eb597eb7]::run_compiler::{closure#1}>::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>
  37:     0x7f830f306c26 - std[153e7eca4933144d]::panic::catch_unwind::<core[cada5d954255392c]::panic::unwind_safe::AssertUnwindSafe<<std[153e7eca4933144d]::thread::Builder>::spawn_unchecked_<rustc_interface[75661f553f767c4f]::util::run_in_thread_pool_with_globals<rustc_interface[75661f553f767c4f]::interface::run_compiler<core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>, rustc_driver[4f9bbda5eb597eb7]::run_compiler::{closure#1}>::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>
  38:     0x7f830f2b63a4 - <<std[153e7eca4933144d]::thread::Builder>::spawn_unchecked_<rustc_interface[75661f553f767c4f]::util::run_in_thread_pool_with_globals<rustc_interface[75661f553f767c4f]::interface::run_compiler<core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>, rustc_driver[4f9bbda5eb597eb7]::run_compiler::{closure#1}>::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[bb1bee83f532b20d]::ErrorGuaranteed>>::{closure#1} as core[cada5d954255392c]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7f830e8e9bee - std::sys::unix::thread::Thread::new::thread_start::h5d9f8d6d1fd775b4
  40:     0x7f830e67eb43 - <unknown>
  41:     0x7f830e710a00 - <unknown>
  42:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.68.0-nightly (0b208a1d5 2023-01-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
