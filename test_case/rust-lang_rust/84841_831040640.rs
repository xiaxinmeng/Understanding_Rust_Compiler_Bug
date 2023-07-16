
⣿
Standard Error

   Compiling playground v0.0.1 (/playground)
thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', /cargo/registry/src/github.com-1ecc6299db9ec823/ena-0.14.0/src/snapshot_vec.rs:199:10
stack backtrace:
   0: rust_begin_unwind
             at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/panicking.rs:493:5
   1: core::panicking::panic_fmt
             at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/core/src/panicking.rs:92:14
   2: core::panicking::panic_bounds_check
             at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/core/src/panicking.rs:69:5
   3: ena::unify::UnificationTable<S>::probe_value
   4: <rustc_infer::infer::canonical::canonicalizer::Canonicalizer as rustc_middle::ty::fold::TypeFolder>::fold_const
   5: rustc_middle::ty::fold::TypeFoldable::fold_with
   6: <rustc_infer::infer::canonical::canonicalizer::Canonicalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
   7: rustc_middle::ty::fold::TypeFoldable::fold_with
   8: rustc_middle::ty::fold::TypeFoldable::fold_with
   9: rustc_middle::ty::fold::TypeFoldable::fold_with
  10: rustc_infer::infer::canonical::canonicalizer::Canonicalizer::canonicalize
  11: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  12: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  13: rustc_infer::infer::InferCtxtBuilder::enter
  14: rustc_trait_selection::traits::type_implements_trait
  15: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::type_implements_trait>::compute
  16: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  17: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  18: rustc_data_structures::stack::ensure_sufficient_stack
  19: rustc_query_system::query::plumbing::force_query_with_job
  20: rustc_query_system::query::plumbing::get_query_impl
  21: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::suggestions::InferCtxtExt>::suggest_await_before_try
  22: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_selection_error
  23: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors
  24: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types
  25: rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::confirm_builtin_call
  26: rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_call
  27: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
  28: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  29: rustc_typeck::check::_match::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::demand_scrutinee_type
  30: rustc_typeck::check::_match::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_match
  31: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
  32: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  33: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_stmt
  34: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected
  35: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  36: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  37: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected
  38: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  39: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr
  40: rustc_typeck::check::check::check_fn
  41: rustc_typeck::check::closure::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_closure
  42: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
  43: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  44: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types
  45: rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::confirm_builtin_call
  46: rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_call
  47: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
  48: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  49: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr
  50: rustc_typeck::check::check::check_fn
  51: rustc_infer::infer::InferCtxtBuilder::enter
  52: rustc_typeck::check::typeck
  53: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  54: rustc_data_structures::stack::ensure_sufficient_stack
  55: rustc_query_system::query::plumbing::force_query_with_job
  56: rustc_query_system::query::plumbing::get_query_impl
  57: rustc_middle::ty::context::TyCtxt::typeck_opt_const_arg
  58: rustc_infer::infer::InferCtxtBuilder::enter
  59: rustc_mir_build::build::mir_built
  60: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_built>::compute
  61: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  62: rustc_data_structures::stack::ensure_sufficient_stack
  63: rustc_query_system::query::plumbing::force_query_with_job
  64: rustc_query_system::query::plumbing::get_query_impl
  65: rustc_mir::transform::check_unsafety::unsafety_check_result
  66: core::ops::function::FnOnce::call_once
  67: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::unsafety_check_result>::compute
  68: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  69: rustc_data_structures::stack::ensure_sufficient_stack
  70: rustc_query_system::query::plumbing::force_query_with_job
  71: rustc_query_system::query::plumbing::get_query_impl
  72: rustc_query_system::query::plumbing::ensure_query_impl
  73: rustc_mir::transform::mir_const
  74: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_const>::compute
  75: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  76: rustc_data_structures::stack::ensure_sufficient_stack
  77: rustc_query_system::query::plumbing::force_query_with_job
  78: rustc_query_system::query::plumbing::get_query_impl
  79: rustc_mir::transform::mir_promoted
  80: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_promoted>::compute
  81: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  82: rustc_data_structures::stack::ensure_sufficient_stack
  83: rustc_query_system::query::plumbing::force_query_with_job
  84: rustc_query_system::query::plumbing::get_query_impl
  85: rustc_mir::borrow_check::mir_borrowck
  86: core::ops::function::FnOnce::call_once
  87: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_borrowck>::compute
  88: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  89: rustc_data_structures::stack::ensure_sufficient_stack
  90: rustc_query_system::query::plumbing::force_query_with_job
  91: rustc_query_system::query::plumbing::get_query_impl
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0 (2fd73fabe 2021-03-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [type_implements_trait] evaluating `type_implements_trait` `(DefId(2:9578 ~ core[4fe0]::future::future::Future), impl std::future::Future, [], ParamEnv { caller_bounds: [], reveal: UserFacing })`
#1 [typeck] type-checking `main`
#2 [mir_built] building MIR for `main`
#3 [unsafety_check_result] unsafety-checking `main`
#4 [mir_const] processing MIR for `main`
#5 [mir_promoted] processing `main`
#6 [mir_borrowck] borrow-checking `main`
#7 [type_of] computing type of `main::{opaque#0}`
#8 [check_mod_item_types] checking item types in top-level module
#9 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: trimmed_def_paths constructed
  |
  = note: delayed at    0: rustc_errors::Handler::delay_good_path_bug
             1: rustc_middle::ty::print::pretty::trimmed_def_paths
             2: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::trimmed_def_paths>::compute
             3: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             4: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             5: rustc_data_structures::stack::ensure_sufficient_stack
             6: rustc_query_system::query::plumbing::force_query_with_job
             7: rustc_query_system::query::plumbing::get_query_impl
             8: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path
             9: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path
            10: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::pretty::PrettyPrinter>::in_binder
            11: std::thread::local::LocalKey<T>::with
            12: rustc_middle::ty::print::pretty::PrettyPrinter::pretty_print_type
            13: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_type
            14: rustc_middle::ty::print::pretty::<impl core::fmt::Display for &rustc_middle::ty::TyS>::fmt
            15: core::fmt::write
                       at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/core/src/fmt/mod.rs:1096:17
            16: core::fmt::Write::write_fmt
            17: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::on_unimplemented::InferCtxtExt>::on_unimplemented_note
            18: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_selection_error
            19: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors
            20: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types
            21: rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::confirm_builtin_call
            22: rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_call
            23: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
            24: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
            25: rustc_typeck::check::_match::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::demand_scrutinee_type
            26: rustc_typeck::check::_match::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_match
            27: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
            28: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
            29: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_stmt
            30: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected
            31: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
            32: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
            33: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected
            34: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
            35: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr
            36: rustc_typeck::check::check::check_fn
            37: rustc_typeck::check::closure::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_closure
            38: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
            39: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
            40: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types
            41: rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::confirm_builtin_call
            42: rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_call
            43: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
            44: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
            45: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr
            46: rustc_typeck::check::check::check_fn
            47: rustc_infer::infer::InferCtxtBuilder::enter
            48: rustc_typeck::check::typeck
            49: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
            50: rustc_data_structures::stack::ensure_sufficient_stack
            51: rustc_query_system::query::plumbing::force_query_with_job
            52: rustc_query_system::query::plumbing::get_query_impl
            53: rustc_middle::ty::context::TyCtxt::typeck_opt_const_arg
            54: rustc_infer::infer::InferCtxtBuilder::enter
            55: rustc_mir_build::build::mir_built
            56: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_built>::compute
            57: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
            58: rustc_data_structures::stack::ensure_sufficient_stack
            59: rustc_query_system::query::plumbing::force_query_with_job
            60: rustc_query_system::query::plumbing::get_query_impl
            61: rustc_mir::transform::check_unsafety::unsafety_check_result
            62: core::ops::function::FnOnce::call_once
            63: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::unsafety_check_result>::compute
            64: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
            65: rustc_data_structures::stack::ensure_sufficient_stack
            66: rustc_query_system::query::plumbing::force_query_with_job
            67: rustc_query_system::query::plumbing::get_query_impl
            68: rustc_query_system::query::plumbing::ensure_query_impl
            69: rustc_mir::transform::mir_const
            70: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_const>::compute
            71: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
            72: rustc_data_structures::stack::ensure_sufficient_stack
            73: rustc_query_system::query::plumbing::force_query_with_job
            74: rustc_query_system::query::plumbing::get_query_impl
            75: rustc_mir::transform::mir_promoted
            76: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_promoted>::compute
            77: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
            78: rustc_data_structures::stack::ensure_sufficient_stack
            79: rustc_query_system::query::plumbing::force_query_with_job
            80: rustc_query_system::query::plumbing::get_query_impl
            81: rustc_mir::borrow_check::mir_borrowck
            82: core::ops::function::FnOnce::call_once
            83: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_borrowck>::compute
            84: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
            85: rustc_data_structures::stack::ensure_sufficient_stack
            86: rustc_query_system::query::plumbing::force_query_with_job
            87: rustc_query_system::query::plumbing::get_query_impl
            88: rustc_typeck::collect::type_of::type_of
            89: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::type_of>::compute
            90: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
            91: rustc_data_structures::stack::ensure_sufficient_stack
            92: rustc_query_system::query::plumbing::force_query_with_job
            93: rustc_query_system::query::plumbing::get_query_impl
            94: rustc_typeck::check::check::check_item_type
            95: rustc_middle::hir::map::Map::visit_item_likes_in_module
            96: rustc_typeck::check::check::check_mod_item_types
            97: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::check_mod_item_types>::compute
            98: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
            99: rustc_data_structures::stack::ensure_sufficient_stack
           100: rustc_query_system::query::plumbing::force_query_with_job
           101: rustc_query_system::query::plumbing::get_query_impl
           102: rustc_query_system::query::plumbing::ensure_query_impl
           103: rustc_session::utils::<impl rustc_session::session::Session>::time
           104: rustc_typeck::check_crate
           105: rustc_interface::passes::analysis
           106: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
           107: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
           108: rustc_data_structures::stack::ensure_sufficient_stack
           109: rustc_query_system::query::plumbing::force_query_with_job
           110: rustc_query_system::query::plumbing::get_query_impl
           111: rustc_interface::passes::QueryContext::enter
           112: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
           113: rustc_span::with_source_map
           114: rustc_interface::interface::create_compiler_and_run
           115: rustc_span::with_session_globals
           116: std::sys_common::backtrace::__rust_begin_short_backtrace
           117: core::ops::function::FnOnce::call_once{{vtable.shim}}
           118: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/alloc/src/boxed.rs:1521:9
                <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/alloc/src/boxed.rs:1521:9
                std::sys::unix::thread::Thread::new::thread_start
                       at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/sys/unix/thread.rs:71:17
           119: start_thread
           120: clone
          

thread 'rustc' panicked at 'no warnings or errors encountered even though `delayed_good_path_bugs` issued', compiler/rustc_errors/src/lib.rs:974:13
stack backtrace:
   0:     0x7f8cf5fe5ee0 - std::backtrace_rs::backtrace::libunwind::trace::h5e9d00f0cdf4f57e
                               at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7f8cf5fe5ee0 - std::backtrace_rs::backtrace::trace_unsynchronized::hd5302bd66215dab9
                               at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f8cf5fe5ee0 - std::sys_common::backtrace::_print_fmt::ha0237cd11a34e2bf
                               at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7f8cf5fe5ee0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h171d4c10df1a98ee
                               at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7f8cf6056d0c - core::fmt::write::h89e4288724daa3fa
                               at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/core/src/fmt/mod.rs:1096:17
   5:     0x7f8cf5fd8ff2 - std::io::Write::write_fmt::h6d40f996e84584d9
                               at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/io/mod.rs:1568:15
   6:     0x7f8cf5fe9d95 - std::sys_common::backtrace::_print::h0c0b93221682afc8
                               at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7f8cf5fe9d95 - std::sys_common::backtrace::print::h57a9f95204c2fdd6
                               at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7f8cf5fe9d95 - std::panicking::default_hook::{{closure}}::h4245258b50e37e69
                               at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/panicking.rs:208:50
   9:     0x7f8cf5fe98f3 - std::panicking::default_hook::h7b00dcc1d0944747
                               at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/panicking.rs:225:9
  10:     0x7f8cf684bf3b - rustc_driver::report_ice::hd11b2540f4ebea82
  11:     0x7f8cf5fea696 - std::panicking::rust_panic_with_hook::h71e6a073d87de1f5
                               at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/panicking.rs:595:17
  12:     0x7f8cf5fea1b7 - std::panicking::begin_panic_handler::{{closure}}::hd549436f6bb6dbb8
                               at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/panicking.rs:497:13
  13:     0x7f8cf5fe637c - std::sys_common::backtrace::__rust_end_short_backtrace::h4e5f4b72b04174c3
                               at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/sys_common/backtrace.rs:141:18
  14:     0x7f8cf5fea119 - rust_begin_unwind
                               at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/panicking.rs:493:5
  15:     0x7f8cf5fea0cb - std::panicking::begin_panic_fmt::h818c3c917eaeb432
                               at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/panicking.rs:435:5
  16:     0x7f8cf97aa023 - rustc_errors::HandlerInner::flush_delayed::h07c536cfe5282664
  17:     0x7f8cf97a8922 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h866df0da1f18e81d
  18:     0x7f8cf8b8b398 - core::ptr::drop_in_place<rustc_session::parse::ParseSess>::h7d54056c5d0955f3
  19:     0x7f8cf8b8eb6d - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::haeaa1478c51489e4
  20:     0x7f8cf8b8c37d - core::ptr::drop_in_place<rustc_interface::interface::Compiler>::hc8b67c944d56a5c9
  21:     0x7f8cf8b89c04 - rustc_span::with_source_map::hedfeccc0422f91c4
  22:     0x7f8cf8b90eca - rustc_interface::interface::create_compiler_and_run::h7abf0b53119fd7ea
  23:     0x7f8cf8b89d05 - rustc_span::with_session_globals::hb5dbfdbd3bd12723
  24:     0x7f8cf8b9136a - std::sys_common::backtrace::__rust_begin_short_backtrace::hfe0fde7e082e2baf
  25:     0x7f8cf8badc4a - core::ops::function::FnOnce::call_once{{vtable.shim}}::h83306388c44d16bf
  26:     0x7f8cf5ffac8a - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h61144a2be4ee36d8
                               at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/alloc/src/boxed.rs:1521:9
  27:     0x7f8cf5ffac8a - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hcf5d395fdd120c17
                               at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/alloc/src/boxed.rs:1521:9
  28:     0x7f8cf5ffac8a - std::sys::unix::thread::Thread::new::thread_start::hb5e40d3d934ebb7a
                               at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/sys/unix/thread.rs:71:17
  29:     0x7f8cf5f2e609 - start_thread
  30:     0x7f8cf5e42293 - clone
  31:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0 (2fd73fabe 2021-03-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden
