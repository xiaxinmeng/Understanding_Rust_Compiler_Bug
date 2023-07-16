
centril@centrilnas2:~/programming/rust/rust-gamma$ git log -1
commit c58e09f138075ce6b3079f41f9c2f192a15b896c (HEAD -> master, upstream/master)     
Merge: 0d34a877225 ee601584403
Author: bors <bors@rust-lang.org>
Date:   Mon Feb 3 09:54:09 2020 +0000

    Auto merge of #68778 - RalfJung:raw-addr-of, r=eddyb

    add raw-addr-of variant to mir_raw_fat_ptr

    As suggested at https://github.com/rust-lang/rust/pull/48300#discussion_r372520388

    r? @eddyb

centril@centrilnas2:~/programming/rust/rust-gamma$ cat crash.rs 
#![feature(core_intrinsics, const_caller_location, track_caller, const_fn)]

use std::panic::Location;
use std::intrinsics::caller_location;

type L = &'static Location<'static>;

#[track_caller]
const fn foo() -> L {
    caller_location()
}

const fn bar() -> L {
    let x: fn() -> L = foo;
    x()
}

fn main() {
    bar();
}

centril@centrilnas2:~/programming/rust/rust-gamma$ rustc -Z unleash-the-miri-inside-of-you crash.rs      
warning: skipping const checks
  --> crash.rs:15:5
   |
15 |     x()
   |     ^^^

Incorrect number of arguments passed to called function!
  %1 = call align 8 dereferenceable(24) %"core::panic::Location"* @_ZN5crash3foo17he37aca50257bbb93E()   
in function _ZN5crash3bar17hc3bef3cb9fdd7a54E
LLVM ERROR: Broken function found, compilation aborted!
thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', /rustc/48840618382eccb8a799320c8e5d08e3b52f4c42/src/libcore/slice/mod.rs:2791:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

centril@centrilnas2:~/programming/rust/rust-gamma$ cat ref_const.rs 
#![feature(core_intrinsics, const_caller_location, track_caller, const_fn)]

use std::panic::Location;
use std::intrinsics::caller_location;

type L = &'static Location<'static>;

#[track_caller]
const fn foo() -> L {
    caller_location()
}

const fn bar() -> L {
    let x: fn() -> L = foo;
    x()
}

const CTFE: L = bar();

fn main() {
    CTFE;
}

centril@centrilnas2:~/programming/rust/rust-gamma$ RUST_BACKTRACE=1 rustc -Z unleash-the-miri-inside-of-you ref_const.rs
warning: skipping const checks
  --> ref_const.rs:15:5
   |
15 |     x()
   |     ^^^

thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', /rustc/48840618382eccb8a799320c8e5d08e3b52f4c42/src/libcore/slice/mod.rs:2791:10
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1057
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1426
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:204
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:224
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:476
  12: rust_begin_unwind
             at src/libstd/panicking.rs:380
  13: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  14: core::panicking::panic_bounds_check
             at src/libcore/panicking.rs:63
  15: rustc_mir::interpret::eval_context::InterpCx<M>::layout_of_local
  16: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::access_local
  17: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_place_to_op
  18: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_operand
  19: <core::iter::adapters::ResultShunt<I,E> as core::iter::traits::iterator::Iterator>::next
  20: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  21: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::step
  22: rustc_mir::const_eval::eval_queries::const_eval_raw_provider
  23: rustc::ty::query::__query_compute::const_eval_raw
  24: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute
  25: rustc::dep_graph::graph::DepGraph::with_task_impl
  26: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  27: rustc_mir::const_eval::machine::<impl rustc_mir::interpret::eval_context::InterpCx<rustc_mir::const_eval::machine::CompileTimeInterpreter>>::try_eval_const_fn_call
  28: <rustc_mir::const_eval::machine::CompileTimeInterpreter as rustc_mir::interpret::machine::Machine>::find_mir_or_eval_fn
  29: rustc_mir::interpret::terminator::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_fn_call
  30: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::step
  31: rustc_mir::const_eval::eval_queries::const_eval_raw_provider
  32: rustc::ty::query::__query_compute::const_eval_raw
  33: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute
  34: rustc::dep_graph::graph::DepGraph::with_task_impl
  35: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  36: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
  37: rustc::ty::query::__query_compute::const_eval_validated
  38: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_validated>::compute
  39: rustc::dep_graph::graph::DepGraph::with_task_impl
  40: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  41: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
  42: rustc::ty::query::__query_compute::const_eval_validated
  43: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_validated>::compute
  44: rustc::dep_graph::graph::DepGraph::with_task_impl
  45: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  46: rustc::mir::interpret::queries::<impl rustc::ty::context::TyCtxt>::const_eval_poly
  47: <rustc_lint::builtin::UnusedBrokenConst as rustc_lint::passes::LateLintPass>::check_item
  48: <rustc_lint::BuiltinCombinedLateLintPass as rustc_lint::passes::LateLintPass>::check_item
  49: rustc_hir::intravisit::Visitor::visit_nested_item
  50: rustc_hir::intravisit::walk_crate
  51: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  52: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:86
  53: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  54: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:86
  55: rustc_session::utils::<impl rustc_session::session::Session>::time
  56: rustc_interface::passes::analysis
  57: rustc::ty::query::__query_compute::analysis
  58: rustc::dep_graph::graph::DepGraph::with_task_impl
  59: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  60: rustc::ty::context::tls::enter_global
  61: rustc_interface::interface::run_compiler_in_existing_thread_pool
  62: scoped_tls::ScopedKey<T>::set
  63: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0-nightly (488406183 2020-01-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z unleash-the-miri-inside-of-you

query stack during panic:
#0 [const_eval_raw] const-evaluating `bar`
#1 [const_eval_raw] const-evaluating `CTFE`
#2 [const_eval_validated] const-evaluating + checking `CTFE`
#3 [const_eval_validated] const-evaluating + checking `CTFE`
#4 [analysis] running analysis passes on this crate
end of query stack
