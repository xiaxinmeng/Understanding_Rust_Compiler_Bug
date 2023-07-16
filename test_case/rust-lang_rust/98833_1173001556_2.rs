
$ RUSTFLAGS="-Cinstrument-coverage" cargo +nightly build --release
   Compiling num-integer v0.1.45
   Compiling chrono v0.4.19
thread 'rustc' panicked at 'No counters provided the source_hash for used function: Instance { def: Item(WithOptConstParam { did: DefId(0:53 ~ num_integer[ff7c]::roots::log2), const_param_did: None }), substs: [u128] }', compiler/rustc_codegen_ssa/src/coverageinfo/map.rs:156:9
stack backtrace:
   0:        0x1018dbf68 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha443feebef1054ca
   1:        0x101928698 - core::fmt::write::h326d27028239f921
   2:        0x1018ced1c - std::io::Write::write_fmt::h2e4d7ac9e7b0a445
   3:        0x1018de9e8 - std::panicking::default_hook::{{closure}}::hb60da310c8c45b08
   4:        0x1018de70c - std::panicking::default_hook::h7728ce77595813b9
   5:        0x108e40614 - rustc_driver[71d987a190a02667]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:        0x1018df0ac - std::panicking::rust_panic_with_hook::he11bb99bed5691b3
   7:        0x1018def54 - std::panicking::begin_panic_handler::{{closure}}::h15cc183105a01e55
   8:        0x1018dc46c - std::sys_common::backtrace::__rust_end_short_backtrace::h2e03eb47752209e1
   9:        0x1018decac - _rust_begin_unwind
  10:        0x101953930 - core::panicking::panic_fmt::h14b6ee096d68264c
  11:        0x10c010fe4 - <rustc_codegen_ssa[264ae2e5154ae899]::coverageinfo::map::FunctionCoverage>::get_expressions_and_counter_regions
  12:        0x10906c0f8 - rustc_codegen_llvm[e6b07e8dca291197]::coverageinfo::mapgen::finalize
  13:        0x10900872c - rustc_codegen_llvm[e6b07e8dca291197]::base::compile_codegen_unit::module_codegen
  14:        0x108f9cd34 - <rustc_query_system[24210b10db0c4feb]::dep_graph::graph::DepGraph<rustc_middle[fcca612a70a3d256]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[fcca612a70a3d256]::ty::context::TyCtxt, rustc_span[3feaa12df53193d3]::symbol::Symbol, rustc_codegen_ssa[264ae2e5154ae899]::ModuleCodegen<rustc_codegen_llvm[e6b07e8dca291197]::ModuleLlvm>>
  15:        0x109007e38 - rustc_codegen_llvm[e6b07e8dca291197]::base::compile_codegen_unit
  16:        0x108fa0204 - rustc_codegen_ssa[264ae2e5154ae899]::base::codegen_crate::<rustc_codegen_llvm[e6b07e8dca291197]::LlvmCodegenBackend>
  17:        0x10906e938 - <rustc_codegen_llvm[e6b07e8dca291197]::LlvmCodegenBackend as rustc_codegen_ssa[264ae2e5154ae899]::traits::backend::CodegenBackend>::codegen_crate
  18:        0x108ee743c - <rustc_session[bab2e65aebcec4dc]::session::Session>::time::<alloc[bc27fb785c4b0855]::boxed::Box<dyn core[666029bfe11acc21]::any::Any>, rustc_interface[b724eb991156d0f4]::passes::start_codegen::{closure#0}>
  19:        0x108ebfe38 - <rustc_interface[b724eb991156d0f4]::passes::QueryContext>::enter::<<rustc_interface[b724eb991156d0f4]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[666029bfe11acc21]::result::Result<alloc[bc27fb785c4b0855]::boxed::Box<dyn core[666029bfe11acc21]::any::Any>, rustc_errors[2b5a7313142c52d3]::ErrorGuaranteed>>
  20:        0x108edbcd0 - <rustc_interface[b724eb991156d0f4]::queries::Queries>::ongoing_codegen
  21:        0x108dd3c24 - <rustc_interface[b724eb991156d0f4]::interface::Compiler>::enter::<rustc_driver[71d987a190a02667]::run_compiler::{closure#1}::{closure#2}, core[666029bfe11acc21]::result::Result<core[666029bfe11acc21]::option::Option<rustc_interface[b724eb991156d0f4]::queries::Linker>, rustc_errors[2b5a7313142c52d3]::ErrorGuaranteed>>
  22:        0x108dcaf00 - rustc_span[3feaa12df53193d3]::with_source_map::<core[666029bfe11acc21]::result::Result<(), rustc_errors[2b5a7313142c52d3]::ErrorGuaranteed>, rustc_interface[b724eb991156d0f4]::interface::create_compiler_and_run<core[666029bfe11acc21]::result::Result<(), rustc_errors[2b5a7313142c52d3]::ErrorGuaranteed>, rustc_driver[71d987a190a02667]::run_compiler::{closure#1}>::{closure#1}>
  23:        0x108dee1bc - rustc_interface[b724eb991156d0f4]::interface::create_compiler_and_run::<core[666029bfe11acc21]::result::Result<(), rustc_errors[2b5a7313142c52d3]::ErrorGuaranteed>, rustc_driver[71d987a190a02667]::run_compiler::{closure#1}>
  24:        0x108dc7ea8 - <scoped_tls[d5a79b7ec1ef2205]::ScopedKey<rustc_span[3feaa12df53193d3]::SessionGlobals>>::set::<rustc_interface[b724eb991156d0f4]::interface::run_compiler<core[666029bfe11acc21]::result::Result<(), rustc_errors[2b5a7313142c52d3]::ErrorGuaranteed>, rustc_driver[71d987a190a02667]::run_compiler::{closure#1}>::{closure#0}, core[666029bfe11acc21]::result::Result<(), rustc_errors[2b5a7313142c52d3]::ErrorGuaranteed>>
  25:        0x108dd0bd8 - std[6562109629247121]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[b724eb991156d0f4]::util::run_in_thread_pool_with_globals<rustc_interface[b724eb991156d0f4]::interface::run_compiler<core[666029bfe11acc21]::result::Result<(), rustc_errors[2b5a7313142c52d3]::ErrorGuaranteed>, rustc_driver[71d987a190a02667]::run_compiler::{closure#1}>::{closure#0}, core[666029bfe11acc21]::result::Result<(), rustc_errors[2b5a7313142c52d3]::ErrorGuaranteed>>::{closure#0}, core[666029bfe11acc21]::result::Result<(), rustc_errors[2b5a7313142c52d3]::ErrorGuaranteed>>
  26:        0x108e024ec - <<std[6562109629247121]::thread::Builder>::spawn_unchecked_<rustc_interface[b724eb991156d0f4]::util::run_in_thread_pool_with_globals<rustc_interface[b724eb991156d0f4]::interface::run_compiler<core[666029bfe11acc21]::result::Result<(), rustc_errors[2b5a7313142c52d3]::ErrorGuaranteed>, rustc_driver[71d987a190a02667]::run_compiler::{closure#1}>::{closure#0}, core[666029bfe11acc21]::result::Result<(), rustc_errors[2b5a7313142c52d3]::ErrorGuaranteed>>::{closure#0}, core[666029bfe11acc21]::result::Result<(), rustc_errors[2b5a7313142c52d3]::ErrorGuaranteed>>::{closure#1} as core[666029bfe11acc21]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:        0x1018e7184 - std::sys::unix::thread::Thread::new::thread_start::h00b5207c93c970dd
  28:        0x1ba22826c - __pthread_deallocate

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (f2d93935f 2022-07-02) running on aarch64-apple-darwin

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C instrument-coverage

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `num-integer`
warning: build failed, waiting for other jobs to finish...
