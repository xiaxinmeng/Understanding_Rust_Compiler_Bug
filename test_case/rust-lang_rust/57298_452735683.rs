rust
$ time RUST_BACKTRACE=1 ./target/debug/cargo-clippy 
!! LD_LIBRARY_PATH=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:
!! Executing '/home/xftroxgpx/build/2nonpkgs/rust.stuff/cargo/cargo//target/release//cargo' in pwd='/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy' with args: 'check'
    Checking unicode-normalization v0.1.7
    Checking regex-syntax v0.6.4
    Checking thread_local v0.3.6
    Checking itertools v0.7.11
error: internal compiler error: src/librustc/ty/context.rs:246: node type $crate::lazy::Lazy<Mutex<ThreadIdManager>> (id=2800) with HirId::owner DefId(0/0:134 ~ thread_local[6342]::thread_id[0]::{{impl}}[3]::deref[0]::__stability[0]::LAZY[0]) cannot be placed in TypeckTables with local_id_root DefId(0/0:131 ~ thread_local[6342]::thread_id[0]::{{impl}}[3]::deref[0]::__stability[0])

thread '<unnamed>' panicked at 'Box<Any>', src/librustc_errors/lib.rs:590:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: rustc::ty::context::validate_hir_id_for_typeck_tables::{{closure}}
  15: rustc::ty::context::tls::with::{{closure}}
  16: rustc::ty::context::tls::with_context::{{closure}}
  17: rustc::ty::context::tls::with_context_opt
  18: rustc::ty::context::tls::with_context
  19: rustc::ty::context::tls::with
  20: rustc::ty::context::TypeckTables::node_id_to_type_opt
  21: <clippy_lints::random_state::Pass as rustc::lint::LateLintPass<'a, 'tcx>>::check_ty
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints/src/random_state.rs:38
  22: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_ty
  23: rustc::hir::intravisit::walk_item
  24: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::lint::context::LintContext<'tcx>>::with_lint_attrs
  25: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  26: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_decl
  27: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_stmt
  28: rustc::hir::intravisit::walk_block
  29: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_block
  30: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::lint::context::LintContext<'tcx>>::with_lint_attrs
  31: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  32: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_body
  33: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_nested_body
  34: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_fn
  35: rustc::hir::intravisit::walk_item
  36: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::lint::context::LintContext<'tcx>>::with_lint_attrs
  37: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  38: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_decl
  39: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_stmt
  40: rustc::hir::intravisit::walk_block
  41: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_block
  42: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::lint::context::LintContext<'tcx>>::with_lint_attrs
  43: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  44: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_body
  45: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_nested_body
  46: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_fn
  47: rustc::hir::intravisit::walk_impl_item
  48: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::lint::context::LintContext<'tcx>>::with_lint_attrs
  49: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_impl_item
  50: rustc::hir::intravisit::walk_impl_item_ref
  51: rustc::hir::intravisit::walk_item
  52: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::lint::context::LintContext<'tcx>>::with_lint_attrs
  53: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  54: rustc::hir::intravisit::walk_mod
  55: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_mod
  56: rustc::hir::intravisit::walk_item
  57: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::lint::context::LintContext<'tcx>>::with_lint_attrs
  58: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  59: rustc::hir::intravisit::walk_mod
  60: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_mod
  61: rustc::hir::intravisit::walk_crate
  62: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::lint::context::LintContext<'tcx>>::with_lint_attrs
  63: rustc::lint::context::check_crate
  64: rustc::util::common::time
  65: rustc_rayon_core::tlv::with
  66: <std::thread::local::LocalKey<T>>::with
  67: rustc::ty::context::tls::enter_global
  68: rustc::ty::context::TyCtxt::create_and_enter
  69: rustc_driver::driver::phase_3_run_analysis_passes
  70: rustc_driver::driver::compile_input
  71: rustc_driver::run_compiler_with_pool
  72: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  73: std::panicking::try::do_call
  74: __rust_maybe_catch_panic
  75: std::panicking::try
  76: std::panic::catch_unwind
  77: rustc_rayon_core::unwind::halt_unwinding
  78: <rustc_rayon_core::job::StackJob<L, F, R> as rustc_rayon_core::job::Job>::execute
  79: rustc_rayon_core::registry::WorkerThread::wait_until_cold
  80: rustc_rayon_core::registry::main_loop::{{closure}}
  81: <scoped_tls::ScopedKey<T>>::set
  82: <std::thread::local::LocalKey<T>>::with
  83: <scoped_tls::ScopedKey<T>>::set
  84: rustc_rayon_core::thread_pool::ThreadPool::scoped_pool::{{closure}}
  85: __rust_maybe_catch_panic
query stack during panic:
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.33.0-dev (9d5481282 2019-01-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `thread_local`.
warning: build failed, waiting for other jobs to finish...
^C  Building [=============================>                          ] 62/114

real	0m5.080s
user	0m0.003s
sys	0m0.001s
