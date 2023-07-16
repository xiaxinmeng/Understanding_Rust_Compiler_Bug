
error: internal compiler error: trimmed_def_paths constructed
  |
  = note: delayed at    0: std::backtrace::Backtrace::create
             1: rustc_errors::Handler::delay_good_path_bug
             2: rustc_middle::ty::print::pretty::trimmed_def_paths
             3: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             4: rustc_data_structures::stack::ensure_sufficient_stack
             5: rustc_query_system::query::plumbing::try_execute_query
             6: rustc_query_system::query::plumbing::get_query
             7: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path
             8: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path
             9: rustc_middle::ty::print::pretty::PrettyPrinter::pretty_print_type
            10: rustc_middle::ty::print::pretty::<impl core::fmt::Display for &rustc_middle::ty::TyS>::fmt
            11: core::fmt::write
            12: alloc::fmt::format
            13: rustc_mir_build::thir::pattern::const_to_pat::ConstToPat::recur
            14: <core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold
            15: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
            16: core::iter::adapters::process_results
            17: rustc_mir_build::thir::pattern::const_to_pat::ConstToPat::recur
            18: rustc_mir_build::thir::pattern::const_to_pat::ConstToPat::recur
            19: rustc_mir_build::thir::pattern::const_to_pat::ConstToPat::to_pat
            20: rustc_infer::infer::InferCtxtBuilder::enter
            21: rustc_mir_build::thir::pattern::const_to_pat::<impl rustc_mir_build::thir::pattern::PatCtxt>::const_to_pat
            22: rustc_mir_build::thir::pattern::PatCtxt::lower_path
            23: rustc_mir_build::thir::pattern::PatCtxt::lower_pattern
            24: rustc_mir_build::thir::pattern::check_match::MatchVisitor::lower_pattern
            25: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
            26: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
            27: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
            28: rustc_hir::intravisit::walk_expr
            29: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
            30: rustc_hir::intravisit::walk_expr
            31: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
            32: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
            33: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
            34: rustc_mir_build::thir::pattern::check_match::check_match
            35: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
            36: rustc_data_structures::stack::ensure_sufficient_stack
            37: rustc_query_system::query::plumbing::try_execute_query
            38: rustc_query_system::query::plumbing::get_query
            39: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners
            40: rustc_session::utils::<impl rustc_session::session::Session>::time
            41: rustc_interface::passes::analysis
            42: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
            43: rustc_data_structures::stack::ensure_sufficient_stack
            44: rustc_query_system::query::plumbing::try_execute_query
            45: rustc_query_system::query::plumbing::get_query
            46: rustc_interface::passes::QueryContext::enter
            47: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
            48: rustc_span::with_source_map
            49: scoped_tls::ScopedKey<T>::set
            50: std::sys_common::backtrace::__rust_begin_short_backtrace
            51: core::ops::function::FnOnce::call_once{{vtable.shim}}
            52: std::sys::unix::thread::Thread::new::thread_start
            53: _pthread_jit_write_protect_np
          

thread 'rustc' panicked at 'no warnings or errors encountered even though `delayed_good_path_bugs` issued', compiler/rustc_errors/src/lib.rs:1165:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (9dd4ce80f 2021-09-17) running on aarch64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
