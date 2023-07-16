
Compiling playground v0.0.1 (file:///playground)
warning[E0161]: cannot move a value of type dyn std::ops::FnOnce(): the size of dyn std::ops::FnOnce() cannot be statically determined
 --> src/main.rs:3:5
  |
3 |     f()
  |     ^
  |
  = warning: This error has been downgraded to a warning for backwards compatibility with previous releases.
          It represents potential unsoundness in your code.
          This warning will become a hard error in the future.

thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `Some(([4 x i64]*:  %13 = load [4 x i64]*, [4 x i64]** %12, align 8, !dbg !39, !nonnull !4))`,
 right: `None`', librustc_codegen_llvm/mir/place.rs:100:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:479
   6: std::panicking::continue_panic_fmt
             at libstd/panicking.rs:390
   7: std::panicking::begin_panic_fmt
             at libstd/panicking.rs:345
   8: rustc_codegen_llvm::mir::place::PlaceRef::load
   9: rustc_codegen_llvm::mir::operand::<impl rustc_codegen_llvm::mir::FunctionCx<'a, 'll, 'tcx>>::codegen_consume
  10: rustc_codegen_llvm::mir::operand::<impl rustc_codegen_llvm::mir::FunctionCx<'a, 'll, 'tcx>>::codegen_operand
  11: rustc_codegen_llvm::mir::rvalue::<impl rustc_codegen_llvm::mir::FunctionCx<'a, 'll, 'tcx>>::codegen_rvalue
  12: rustc_codegen_llvm::mir::codegen_mir
  13: rustc_codegen_llvm::base::codegen_instance
  14: rustc_codegen_llvm::mono_item::MonoItemExt::define
  15: rustc_codegen_llvm::base::compile_codegen_unit
  16: rustc::ty::query::__query_compute::compile_codegen_unit
  17: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::compile_codegen_unit<'tcx>>::compute
  18: rustc::dep_graph::graph::DepGraph::with_task_impl
  19: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  20: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  21: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  22: rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::compile_codegen_unit
  23: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  24: rustc::util::common::time
  25: rustc_driver::driver::phase_4_codegen
  26: rustc_driver::driver::compile_input::{{closure}}
  27: rustc::ty::context::tls::enter_context
  28: <std::thread::local::LocalKey<T>>::with
  29: rustc::ty::context::TyCtxt::create_and_enter
  30: rustc_driver::driver::compile_input
  31: rustc_driver::run_compiler_with_pool
  32: syntax::with_globals
  33: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  34: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  35: rustc_driver::run
  36: rustc_driver::main
  37: std::rt::lang_start::{{closure}}
  38: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  39: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  40: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at libstd/rt.rs:58
  41: main
  42: __libc_start_main
  43: <unknown>
query stack during panic:
#0 [compile_codegen_unit] compile_codegen_unit
end of query stack
error: internal compiler error: broken MIR in DefId(0/0:3 ~ playground[6a81]::main[0]) (NoSolution): could not prove Binder(TraitPredicate(<dyn std::ops::FnOnce() as std::marker::Sized>))
 --> src/main.rs:3:5
  |
3 |     f()
  |     ^

thread 'main' panicked at 'no errors encountered even though `delay_span_bug` issued', librustc_errors/lib.rs:314:17
stack backtrace:
   0:     0x7f2b7be5b33e - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h1410ee01a7694834
                               at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7f2b7be340f6 - std::sys_common::backtrace::print::h1962f4c30d928b1d
                               at libstd/sys_common/backtrace.rs:71
                               at libstd/sys_common/backtrace.rs:59
   2:     0x7f2b7be31bdd - std::panicking::default_hook::{{closure}}::h66071f1a30ddacaa
                               at libstd/panicking.rs:211
   3:     0x7f2b7be31950 - std::panicking::default_hook::h8288f681d56575d2
                               at libstd/panicking.rs:227
   4:     0x7f2b78547d45 - rustc::util::common::panic_hook::hba0ca6620bd19f58
   5:     0x7f2b7be32363 - std::panicking::rust_panic_with_hook::h46fa5def146c1328
                               at libstd/panicking.rs:479
   6:     0x7f2b771d3396 - std::panicking::begin_panic::h7a665eab6b05efb6
   7:     0x7f2b771cc574 - <rustc_errors::Handler as core::ops::drop::Drop>::drop::hd437c359f6e33d6a
   8:     0x7f2b7c27f2e0 - core::ptr::drop_in_place::h914354111fa0d390
   9:     0x7f2b7c28b234 - rustc_driver::run_compiler_with_pool::h2b086979f97a3943
  10:     0x7f2b7c1e532e - syntax::with_globals::hb19a09c6ec76932b
  11:     0x7f2b7c1d4062 - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h24130c7acac79061
  12:     0x7f2b7be70a19 - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:102
  13:     0x7f2b7c287b9c - rustc_driver::run::h89085067bde9c1b8
  14:     0x7f2b7c2955ca - rustc_driver::main::hb407a4684dbf6522
  15:     0x55e914152992 - std::rt::lang_start::{{closure}}::h8f9801ea24c231a5
  16:     0x7f2b7be31da2 - std::panicking::try::do_call::hc3bbfe344c9dd679
                               at libstd/rt.rs:59
                               at libstd/panicking.rs:310
  17:     0x7f2b7be70a19 - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:102
  18:     0x7f2b7be48955 - std::rt::lang_start_internal::h6831243cb445d5dc
                               at libstd/panicking.rs:289
                               at libstd/panic.rs:392
                               at libstd/rt.rs:58
  19:     0x55e914152983 - main
  20:     0x7f2b7ba3282f - __libc_start_main
  21:     0x55e914152838 - <unknown>
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
error: Could not compile `playground`.

To learn more, run the command again with --verbose.
