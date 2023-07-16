
$ git clone https://github.com/pyca/cryptography
$ cd cryptography/src/rust/
$ RUSTFLAGS="-Cinstrument-coverage" cargo +nightly build --release
   Compiling pyo3 v0.15.2
thread 'rustc' panicked at 'No counters provided the source_hash for used function: Instance { def: Item(WithOptConstParam { did: DefId(0:6218 ~ pyo3[f11a]::python::{impl#6}::run::{closure#0}), const_param_did: None }), substs: [ReErased, i32, extern "rust-call" fn((&types::any::PyAny,)), ()] }', compiler/rustc_codegen_ssa/src/coverageinfo/map.rs:156:9
stack backtrace:
   0:     0x7f2b03aa28a0 - std::backtrace_rs::backtrace::libunwind::trace::h9ae2a0a86525a721
                               at /rustc/848090dcd18553b790461132ca9d2a020aeea9a2/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7f2b03aa28a0 - std::backtrace_rs::backtrace::trace_unsynchronized::h2db65d70ea604a8b
                               at /rustc/848090dcd18553b790461132ca9d2a020aeea9a2/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f2b03aa28a0 - std::sys_common::backtrace::_print_fmt::ha87108be4ced4d7b
                               at /rustc/848090dcd18553b790461132ca9d2a020aeea9a2/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f2b03aa28a0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8289c789e2444f52
                               at /rustc/848090dcd18553b790461132ca9d2a020aeea9a2/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f2b03afbe7c - core::fmt::write::h4bc61850e148256a
                               at /rustc/848090dcd18553b790461132ca9d2a020aeea9a2/library/core/src/fmt/mod.rs:1198:17
   5:     0x7f2b03a93ea5 - std::io::Write::write_fmt::h39fafbe9deae7dce
                               at /rustc/848090dcd18553b790461132ca9d2a020aeea9a2/library/std/src/io/mod.rs:1672:15
   6:     0x7f2b03aa5531 - std::sys_common::backtrace::_print::h48b22fe50876db47
                               at /rustc/848090dcd18553b790461132ca9d2a020aeea9a2/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f2b03aa5531 - std::sys_common::backtrace::print::h4deb0c8c0ba6e812
                               at /rustc/848090dcd18553b790461132ca9d2a020aeea9a2/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f2b03aa5531 - std::panicking::default_hook::{{closure}}::hec18832ee0d2183c
                               at /rustc/848090dcd18553b790461132ca9d2a020aeea9a2/library/std/src/panicking.rs:295:22
   9:     0x7f2b03aa5203 - std::panicking::default_hook::h6fd5dc4e9c4f675e
                               at /rustc/848090dcd18553b790461132ca9d2a020aeea9a2/library/std/src/panicking.rs:314:9
  10:     0x7f2b04351af1 - rustc_driver[c212bfd01fb1bd4e]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f2b03aa5d06 - std::panicking::rust_panic_with_hook::hb4a45181e5abcce3
                               at /rustc/848090dcd18553b790461132ca9d2a020aeea9a2/library/std/src/panicking.rs:702:17
  12:     0x7f2b03aa5b57 - std::panicking::begin_panic_handler::{{closure}}::h8b4f06c0b51d9e35
                               at /rustc/848090dcd18553b790461132ca9d2a020aeea9a2/library/std/src/panicking.rs:588:13
  13:     0x7f2b03aa2d84 - std::sys_common::backtrace::__rust_end_short_backtrace::hc81506ddb6f68297
                               at /rustc/848090dcd18553b790461132ca9d2a020aeea9a2/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7f2b03aa5882 - rust_begin_unwind
                               at /rustc/848090dcd18553b790461132ca9d2a020aeea9a2/library/std/src/panicking.rs:584:5
  15:     0x7f2b03a69b23 - core::panicking::panic_fmt::h16a5c315f7ebffa5
                               at /rustc/848090dcd18553b790461132ca9d2a020aeea9a2/library/core/src/panicking.rs:142:14
  16:     0x7f2b04e8199a - <rustc_codegen_ssa[4eadd18c013d620f]::coverageinfo::map::FunctionCoverage>::get_expressions_and_counter_regions
  17:     0x7f2b04479338 - rustc_codegen_llvm[1092e7de2567b279]::coverageinfo::mapgen::finalize
  18:     0x7f2b056d393e - rustc_codegen_llvm[1092e7de2567b279]::base::compile_codegen_unit::module_codegen
  19:     0x7f2b06577049 - <rustc_query_system[606dbbc21d5be994]::dep_graph::graph::DepGraph<rustc_middle[339c612205d209e0]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[339c612205d209e0]::ty::context::TyCtxt, rustc_span[bd4aa58f7c6f9add]::symbol::Symbol, rustc_codegen_ssa[4eadd18c013d620f]::ModuleCodegen<rustc_codegen_llvm[1092e7de2567b279]::ModuleLlvm>>
  20:     0x7f2b065a6229 - rustc_codegen_llvm[1092e7de2567b279]::base::compile_codegen_unit
  21:     0x7f2b0657a3b9 - rustc_codegen_ssa[4eadd18c013d620f]::base::codegen_crate::<rustc_codegen_llvm[1092e7de2567b279]::LlvmCodegenBackend>
  22:     0x7f2b065bd291 - <rustc_codegen_llvm[1092e7de2567b279]::LlvmCodegenBackend as rustc_codegen_ssa[4eadd18c013d620f]::traits::backend::CodegenBackend>::codegen_crate
  23:     0x7f2b0653ede7 - <rustc_session[7be89286315efa93]::session::Session>::time::<alloc[2307dd647311a06c]::boxed::Box<dyn core[c7b8860b6e72b2f6]::any::Any>, rustc_interface[e455626695b2bf4c]::passes::start_codegen::{closure#0}>
  24:     0x7f2b0653b773 - <rustc_interface[e455626695b2bf4c]::passes::QueryContext>::enter::<<rustc_interface[e455626695b2bf4c]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[c7b8860b6e72b2f6]::result::Result<alloc[2307dd647311a06c]::boxed::Box<dyn core[c7b8860b6e72b2f6]::any::Any>, rustc_errors[4d7b9c19931775fa]::ErrorGuaranteed>>
  25:     0x7f2b06531c83 - <rustc_interface[e455626695b2bf4c]::queries::Queries>::ongoing_codegen
  26:     0x7f2b064f9611 - <rustc_interface[e455626695b2bf4c]::interface::Compiler>::enter::<rustc_driver[c212bfd01fb1bd4e]::run_compiler::{closure#1}::{closure#2}, core[c7b8860b6e72b2f6]::result::Result<core[c7b8860b6e72b2f6]::option::Option<rustc_interface[e455626695b2bf4c]::queries::Linker>, rustc_errors[4d7b9c19931775fa]::ErrorGuaranteed>>
  27:     0x7f2b064f579f - rustc_span[bd4aa58f7c6f9add]::with_source_map::<core[c7b8860b6e72b2f6]::result::Result<(), rustc_errors[4d7b9c19931775fa]::ErrorGuaranteed>, rustc_interface[e455626695b2bf4c]::interface::create_compiler_and_run<core[c7b8860b6e72b2f6]::result::Result<(), rustc_errors[4d7b9c19931775fa]::ErrorGuaranteed>, rustc_driver[c212bfd01fb1bd4e]::run_compiler::{closure#1}>::{closure#1}>
  28:     0x7f2b06511e50 - rustc_interface[e455626695b2bf4c]::interface::create_compiler_and_run::<core[c7b8860b6e72b2f6]::result::Result<(), rustc_errors[4d7b9c19931775fa]::ErrorGuaranteed>, rustc_driver[c212bfd01fb1bd4e]::run_compiler::{closure#1}>
  29:     0x7f2b06525e22 - <scoped_tls[ae38fa8e7a890916]::ScopedKey<rustc_span[bd4aa58f7c6f9add]::SessionGlobals>>::set::<rustc_interface[e455626695b2bf4c]::interface::run_compiler<core[c7b8860b6e72b2f6]::result::Result<(), rustc_errors[4d7b9c19931775fa]::ErrorGuaranteed>, rustc_driver[c212bfd01fb1bd4e]::run_compiler::{closure#1}>::{closure#0}, core[c7b8860b6e72b2f6]::result::Result<(), rustc_errors[4d7b9c19931775fa]::ErrorGuaranteed>>
  30:     0x7f2b064f7cef - std[c85d7afaac2a6227]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e455626695b2bf4c]::util::run_in_thread_pool_with_globals<rustc_interface[e455626695b2bf4c]::interface::run_compiler<core[c7b8860b6e72b2f6]::result::Result<(), rustc_errors[4d7b9c19931775fa]::ErrorGuaranteed>, rustc_driver[c212bfd01fb1bd4e]::run_compiler::{closure#1}>::{closure#0}, core[c7b8860b6e72b2f6]::result::Result<(), rustc_errors[4d7b9c19931775fa]::ErrorGuaranteed>>::{closure#0}, core[c7b8860b6e72b2f6]::result::Result<(), rustc_errors[4d7b9c19931775fa]::ErrorGuaranteed>>
  31:     0x7f2b065122a9 - <<std[c85d7afaac2a6227]::thread::Builder>::spawn_unchecked_<rustc_interface[e455626695b2bf4c]::util::run_in_thread_pool_with_globals<rustc_interface[e455626695b2bf4c]::interface::run_compiler<core[c7b8860b6e72b2f6]::result::Result<(), rustc_errors[4d7b9c19931775fa]::ErrorGuaranteed>, rustc_driver[c212bfd01fb1bd4e]::run_compiler::{closure#1}>::{closure#0}, core[c7b8860b6e72b2f6]::result::Result<(), rustc_errors[4d7b9c19931775fa]::ErrorGuaranteed>>::{closure#0}, core[c7b8860b6e72b2f6]::result::Result<(), rustc_errors[4d7b9c19931775fa]::ErrorGuaranteed>>::{closure#1} as core[c7b8860b6e72b2f6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f2b03aaf753 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h713f45ba125dd07b
                               at /rustc/848090dcd18553b790461132ca9d2a020aeea9a2/library/alloc/src/boxed.rs:1935:9
  33:     0x7f2b03aaf753 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h5469f4a0b2b35b34
                               at /rustc/848090dcd18553b790461132ca9d2a020aeea9a2/library/alloc/src/boxed.rs:1935:9
  34:     0x7f2b03aaf753 - std::sys::unix::thread::Thread::new::thread_start::h81d52548323558fa
                               at /rustc/848090dcd18553b790461132ca9d2a020aeea9a2/library/std/src/sys/unix/thread.rs:108:17
  35:     0x7f2b0386cb43 - start_thread
                               at ./nptl/./nptl/pthread_create.c:442:8
  36:     0x7f2b038fea00 - clone3
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
  37:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (848090dcd 2022-07-22) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C linker-plugin-lto -C overflow-checks=on -C instrument-coverage

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `pyo3`
