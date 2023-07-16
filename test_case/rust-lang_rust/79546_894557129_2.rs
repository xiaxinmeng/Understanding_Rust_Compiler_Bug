
error: internal compiler error: trimmed_def_paths constructed
  |
  = note: delayed at    0: rustc_errors::Handler::delay_good_path_bug
             1: rustc_middle::ty::print::pretty::trimmed_def_paths
             2: rustc_data_structures::stack::ensure_sufficient_stack
             3: rustc_query_system::query::plumbing::get_query_impl
             4: rustc_query_system::query::plumbing::get_query
             5: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path
             6: rustc_middle::ty::print::pretty::<impl rustc_middle::ty::context::TyCtxt>::def_path_str_with_substs
             7: rustc_session::session::Session::consider_optimizing
             8: rustc_middle::ty::context::TyCtxt::consider_optimizing
             9: rustc_middle::ty::ReprOptions::new
            10: rustc_typeck::collect::adt_def
            11: rustc_query_system::query::plumbing::get_query_impl
            12: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::adt_def
            13: rustc_typeck::collect::type_of::type_of
            14: rustc_query_system::query::plumbing::get_query_impl
            15: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::type_of
            16: rustc_typeck::collect::convert_item
            17: <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_item
            18: rustc_middle::hir::map::Map::visit_item_likes_in_module
            19: rustc_typeck::collect::collect_mod_item_types
            20: rustc_query_system::query::plumbing::get_query_impl
            21: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_mod_item_types
            22: rustc_session::session::Session::track_errors
            23: rustc_typeck::check_crate
            24: rustc_interface::passes::analysis
            25: rustc_query_system::query::plumbing::get_query_impl
            26: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
            27: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
            28: rustc_span::with_source_map
            29: rustc_interface::interface::create_compiler_and_run
            30: scoped_tls::ScopedKey<T>::set
            31: std::sys_common::backtrace::__rust_begin_short_backtrace
            32: core::ops::function::FnOnce::call_once{{vtable.shim}}
            33: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /rustc/a6ece56152d8eb11e049e9fcce147b2859e12c92/library/alloc/src/boxed.rs:1572:9
            34: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /rustc/a6ece56152d8eb11e049e9fcce147b2859e12c92/library/alloc/src/boxed.rs:1572:9
            35: std::sys::unix::thread::Thread::new::thread_start
                       at /rustc/a6ece56152d8eb11e049e9fcce147b2859e12c92/library/std/src/sys/unix/thread.rs:91:17
            36: start_thread
            37: __clone
          

thread 'rustc' panicked at 'no warnings or errors encountered even though `delayed_good_path_bugs` issued', compiler/rustc_errors/src/lib.rs:1050:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (a6ece5615 2021-08-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z fuel=test=0

query stack during panic:
end of query stack
