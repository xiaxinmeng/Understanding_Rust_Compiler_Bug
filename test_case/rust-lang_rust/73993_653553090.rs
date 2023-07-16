
$ RUST_BACKTRACE=1 cargo build --release -p edgeware-runtime
   Compiling edgeware-runtime v3.0.0 (edgeware-node/node/runtime)
   Compiling pallet-authorship v2.0.0-rc3 (https://github.com/hicommonwealth/substrate.git?branch=apopiak-migrations-extraction#6df42b9a)
   Compiling pallet-finality-tracker v2.0.0-rc3 (https://github.com/hicommonwealth/substrate.git?branch=apopiak-migrations-extraction#6df42b9a)
   Compiling pallet-utility v2.0.0-rc3 (https://github.com/hicommonwealth/substrate.git?branch=apopiak-migrations-extraction#6df42b9a)
   Compiling pallet-randomness-collective-flip v2.0.0-rc3 (https://github.com/hicommonwealth/substrate.git?branch=apopiak-migrations-extraction#6df42b9a)
   Compiling pallet-collective v2.0.0-rc3 (https://github.com/hicommonwealth/substrate.git?branch=apopiak-migrations-extraction#6df42b9a)
   Compiling pallet-multisig v2.0.0-rc3 (https://github.com/hicommonwealth/substrate.git?branch=apopiak-migrations-extraction#6df42b9a)
   Compiling pallet-indices v2.0.0-rc3 (https://github.com/hicommonwealth/substrate.git?branch=apopiak-migrations-extraction#6df42b9a)
   Compiling pallet-sudo v2.0.0-rc3 (https://github.com/hicommonwealth/substrate.git?branch=apopiak-migrations-extraction#6df42b9a)
error: failed to run custom build command for `edgeware-runtime v3.0.0 (edgeware-node/node/runtime)`

Caused by:
  process didn't exit successfully: `edgeware-node/target/release/build/edgeware-runtime-7e6efc23021a0699/build-script-build` (exit code: 1)
--- stdout
Executing build command: "rustup" "run" "nightly" "cargo" "rustc" "--target=wasm32-unknown-unknown" "--manifest-path=edgeware-node/target/release/wbuild/edgeware-runtime/Cargo.toml" "--color=always" "--release"

--- stderr
   Compiling wasm-build-runner-impl v1.0.0 (edgeware-node/target/release/wbuild-runner/edgeware-runtime16727120478097692426)
    Finished release [optimized] target(s) in 1.43s
     Running `edgeware-node/target/release/wbuild-runner/edgeware-runtime16727120478097692426/target/x86_64-unknown-linux-gnu/release/wasm-build-runner-impl`
   Compiling sp-arithmetic v2.0.0 (https://github.com/hicommonwealth/substrate.git?rev=00a400f82539e2f78e8ddbcd98aea512c87c5f3c#00a400f8)
   Compiling primitive-types v0.7.2
   Compiling primitive-types v0.6.2
   Compiling sp-npos-elections v2.0.0-rc3 (https://github.com/hicommonwealth/substrate.git?branch=apopiak-migrations-extraction#6df42b9a)
   Compiling finality-grandpa v0.12.3
thread 'rustc' panicked at 'if we got here, it must be const', src/librustc_mir/transform/const_prop.rs:1006:34
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1076
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1537
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:217
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:524
  12: rust_begin_unwind
             at src/libstd/panicking.rs:431
  13: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  14: core::option::expect_failed
             at src/libcore/option.rs:1261
  15: <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_terminator::{{closure}}
  16: <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_terminator
  17: <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_body
  18: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
  19: rustc_mir::transform::run_passes
  20: rustc_mir::transform::run_optimization_passes
  21: rustc_mir::transform::optimized_mir
  22: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::optimized_mir>::compute
  23: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  24: rustc_data_structures::stack::ensure_sufficient_stack
  25: rustc_query_system::query::plumbing::get_query_impl
  26: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::instance_mir
  27: rustc_mir::monomorphize::collector::collect_neighbours
  28: rustc_data_structures::stack::ensure_sufficient_stack
  29: rustc_mir::monomorphize::collector::collect_items_rec
  30: rustc_mir::monomorphize::collector::collect_crate_mono_items
  31: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  32: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::collect_and_partition_mono_items>::compute
  33: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  34: rustc_query_system::query::plumbing::get_query_impl
  35: rustc_codegen_ssa::base::codegen_crate
  36: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  37: rustc_session::utils::<impl rustc_session::session::Session>::time
  38: rustc_interface::passes::start_codegen
  39: rustc_middle::ty::context::tls::enter_global
  40: rustc_interface::queries::Queries::ongoing_codegen
  41: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  42: rustc_span::with_source_map
  43: rustc_interface::interface::run_compiler_in_existing_thread_pool
  44: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.46.0-nightly (3503f565e 2020-07-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C panic=abort -C linker-plugin-lto -C link-arg=--export-table -C link-arg=--export=__heap_base -C link-arg=--import-memory --crate-type rlib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] optimizing MIR for `<fixed64::Fixed64 as core::ops::Div>::div`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: could not compile `sp-arithmetic`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed

warning: build failed, waiting for other jobs to finish...
error: build failed
