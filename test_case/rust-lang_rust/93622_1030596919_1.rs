
thread 'rustc' panicked at 'fat_pointer_kind() - Encountered unexpected `pointee_ty`: [closure@src/lib.rs:4:14: 4:21]', compiler/rustc_codegen_llvm/src/debuginfo/utils.rs:102:13
stack backtrace:
   0:     0x7fd74e146ebc - std::backtrace_rs::backtrace::libunwind::trace::hddd5a7e459ce4327
                               at /rustc/4e8fb743ccbec27344b2dd42de7057f41d4ebfdd/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fd74e146ebc - std::backtrace_rs::backtrace::trace_unsynchronized::h69db3e09cfd8bf11
                               at /rustc/4e8fb743ccbec27344b2dd42de7057f41d4ebfdd/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fd74e146ebc - std::sys_common::backtrace::_print_fmt::h83b461b46804fdab
                               at /rustc/4e8fb743ccbec27344b2dd42de7057f41d4ebfdd/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7fd74e146ebc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2f6e5cd8872f3d74
                               at /rustc/4e8fb743ccbec27344b2dd42de7057f41d4ebfdd/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7fd74e1a82dc - core::fmt::write::hd64a00f3787c90bd
                               at /rustc/4e8fb743ccbec27344b2dd42de7057f41d4ebfdd/library/core/src/fmt/mod.rs:1190:17
   5:     0x7fd74e136383 - std::io::Write::write_fmt::h6347402c88d40325
                               at /rustc/4e8fb743ccbec27344b2dd42de7057f41d4ebfdd/library/std/src/io/mod.rs:1653:15
   6:     0x7fd74e14b181 - std::sys_common::backtrace::_print::h989064c3e0b39e3c
                               at /rustc/4e8fb743ccbec27344b2dd42de7057f41d4ebfdd/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7fd74e14b181 - std::sys_common::backtrace::print::h4af8c55ce731a045
                               at /rustc/4e8fb743ccbec27344b2dd42de7057f41d4ebfdd/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7fd74e14b181 - std::panicking::default_hook::{{closure}}::h41ef6f6a9fac6b53
                               at /rustc/4e8fb743ccbec27344b2dd42de7057f41d4ebfdd/library/std/src/panicking.rs:295:22
   9:     0x7fd74e14ae3f - std::panicking::default_hook::hbaf11e031e0e4a5f
                               at /rustc/4e8fb743ccbec27344b2dd42de7057f41d4ebfdd/library/std/src/panicking.rs:314:9
  10:     0x7fd74e928541 - rustc_driver[7b59b981d4c23d31]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fd74e14ba5b - std::panicking::rust_panic_with_hook::h4788f0bd625b1005
                               at /rustc/4e8fb743ccbec27344b2dd42de7057f41d4ebfdd/library/std/src/panicking.rs:702:17
  12:     0x7fd74e14b717 - std::panicking::begin_panic_handler::{{closure}}::h3fc193fd0625168a
                               at /rustc/4e8fb743ccbec27344b2dd42de7057f41d4ebfdd/library/std/src/panicking.rs:588:13
  13:     0x7fd74e147364 - std::sys_common::backtrace::__rust_end_short_backtrace::h379714bd69912a88
                               at /rustc/4e8fb743ccbec27344b2dd42de7057f41d4ebfdd/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7fd74e14b429 - rust_begin_unwind
                               at /rustc/4e8fb743ccbec27344b2dd42de7057f41d4ebfdd/library/std/src/panicking.rs:584:5
  15:     0x7fd74e112993 - core::panicking::panic_fmt::hc13e2c29da5b89d3
                               at /rustc/4e8fb743ccbec27344b2dd42de7057f41d4ebfdd/library/core/src/panicking.rs:135:14
  16:     0x7fd74fb63631 - rustc_codegen_llvm[b175ffb6fe16318c]::debuginfo::utils::fat_pointer_kind
  17:     0x7fd74fb65fb3 - rustc_codegen_llvm[b175ffb6fe16318c]::debuginfo::metadata::pointer_or_reference_metadata
  18:     0x7fd74fb66d04 - rustc_codegen_llvm[b175ffb6fe16318c]::debuginfo::metadata::type_metadata
  19:     0x7fd74fb80e5b - <rustc_codegen_llvm[b175ffb6fe16318c]::context::CodegenCx as rustc_codegen_ssa[caeff511e0898096]::traits::debuginfo::DebugInfoMethods>::dbg_scope_fn
  20:     0x7fd74fbab083 - rustc_codegen_ssa[caeff511e0898096]::mir::codegen_mir::<rustc_codegen_llvm[b175ffb6fe16318c]::builder::Builder>
  21:     0x7fd74fb74e62 - rustc_codegen_llvm[b175ffb6fe16318c]::base::compile_codegen_unit::module_codegen
  22:     0x7fd750802896 - <rustc_query_system[59fab566a51c2c24]::dep_graph::graph::DepGraph<rustc_middle[e664fb30aec2b69e]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[e664fb30aec2b69e]::ty::context::TyCtxt, rustc_span[26d1c5a8904e1196]::symbol::Symbol, rustc_codegen_ssa[caeff511e0898096]::ModuleCodegen<rustc_codegen_llvm[b175ffb6fe16318c]::ModuleLlvm>>
  23:     0x7fd7507fb1ae - rustc_codegen_llvm[b175ffb6fe16318c]::base::compile_codegen_unit
  24:     0x7fd75082b1b2 - <rustc_codegen_llvm[b175ffb6fe16318c]::LlvmCodegenBackend as rustc_codegen_ssa[caeff511e0898096]::traits::backend::CodegenBackend>::codegen_crate
  25:     0x7fd7507d9337 - <rustc_session[bcbf86d149cb5e94]::session::Session>::time::<alloc[53f90fd9233e64f9]::boxed::Box<dyn core[90c82cfbe27f5d33]::any::Any>, rustc_interface[be440f50dcda62f9]::passes::start_codegen::{closure#0}>
  26:     0x7fd7507c1007 - <rustc_interface[be440f50dcda62f9]::passes::QueryContext>::enter::<<rustc_interface[be440f50dcda62f9]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[90c82cfbe27f5d33]::result::Result<alloc[53f90fd9233e64f9]::boxed::Box<dyn core[90c82cfbe27f5d33]::any::Any>, rustc_errors[b77837aed2b0d3e7]::ErrorReported>>
  27:     0x7fd7507b6c9f - <rustc_interface[be440f50dcda62f9]::queries::Queries>::ongoing_codegen
  28:     0x7fd75079684d - <rustc_interface[be440f50dcda62f9]::interface::Compiler>::enter::<rustc_driver[7b59b981d4c23d31]::run_compiler::{closure#1}::{closure#2}, core[90c82cfbe27f5d33]::result::Result<core[90c82cfbe27f5d33]::option::Option<rustc_interface[be440f50dcda62f9]::queries::Linker>, rustc_errors[b77837aed2b0d3e7]::ErrorReported>>
  29:     0x7fd750799876 - rustc_span[26d1c5a8904e1196]::with_source_map::<core[90c82cfbe27f5d33]::result::Result<(), rustc_errors[b77837aed2b0d3e7]::ErrorReported>, rustc_interface[be440f50dcda62f9]::interface::create_compiler_and_run<core[90c82cfbe27f5d33]::result::Result<(), rustc_errors[b77837aed2b0d3e7]::ErrorReported>, rustc_driver[7b59b981d4c23d31]::run_compiler::{closure#1}>::{closure#1}>
  30:     0x7fd7507961fe - rustc_interface[be440f50dcda62f9]::interface::create_compiler_and_run::<core[90c82cfbe27f5d33]::result::Result<(), rustc_errors[b77837aed2b0d3e7]::ErrorReported>, rustc_driver[7b59b981d4c23d31]::run_compiler::{closure#1}>
  31:     0x7fd75077c302 - std[daefdbcf0dca51d0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[be440f50dcda62f9]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[be440f50dcda62f9]::interface::run_compiler<core[90c82cfbe27f5d33]::result::Result<(), rustc_errors[b77837aed2b0d3e7]::ErrorReported>, rustc_driver[7b59b981d4c23d31]::run_compiler::{closure#1}>::{closure#0}, core[90c82cfbe27f5d33]::result::Result<(), rustc_errors[b77837aed2b0d3e7]::ErrorReported>>::{closure#0}, core[90c82cfbe27f5d33]::result::Result<(), rustc_errors[b77837aed2b0d3e7]::ErrorReported>>
  32:     0x7fd750779a19 - <<std[daefdbcf0dca51d0]::thread::Builder>::spawn_unchecked_<rustc_interface[be440f50dcda62f9]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[be440f50dcda62f9]::interface::run_compiler<core[90c82cfbe27f5d33]::result::Result<(), rustc_errors[b77837aed2b0d3e7]::ErrorReported>, rustc_driver[7b59b981d4c23d31]::run_compiler::{closure#1}>::{closure#0}, core[90c82cfbe27f5d33]::result::Result<(), rustc_errors[b77837aed2b0d3e7]::ErrorReported>>::{closure#0}, core[90c82cfbe27f5d33]::result::Result<(), rustc_errors[b77837aed2b0d3e7]::ErrorReported>>::{closure#1} as core[90c82cfbe27f5d33]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  33:     0x7fd74e157803 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hed49a6c6e767ba83
                               at /rustc/4e8fb743ccbec27344b2dd42de7057f41d4ebfdd/library/alloc/src/boxed.rs:1854:9
  34:     0x7fd74e157803 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h14238373d01dc9bc
                               at /rustc/4e8fb743ccbec27344b2dd42de7057f41d4ebfdd/library/alloc/src/boxed.rs:1854:9
  35:     0x7fd74e157803 - std::sys::unix::thread::Thread::new::thread_start::hc6dbf73cd24f68cc
                               at /rustc/4e8fb743ccbec27344b2dd42de7057f41d4ebfdd/library/std/src/sys/unix/thread.rs:108:17
  36:     0x7fd74e08b609 - start_thread
  37:     0x7fd74dfa5293 - clone
  38:                0x0 - <unknown>
