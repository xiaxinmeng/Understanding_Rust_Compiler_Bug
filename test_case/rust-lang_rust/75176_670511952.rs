
 Documenting proc_macro v0.0.0 (/home/joshua/rustc/library/proc_macro)
error: cannot find a built-in macro with name `cfg`
    --> /home/joshua/rustc/library/core/src/macros/mod.rs:1183:5
     |
1183 | /     macro_rules! cfg {
1184 | |         ($($cfg:tt)*) => {
1185 | |             /* compiler built-in */
1186 | |         };
1187 | |     }
     | |_____^

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', src/librustc_errors/lib.rs:941:13
stack backtrace:
   0: std::backtrace_rs::backtrace::libunwind::trace
             at ./library/std/src/../../backtrace/src/backtrace/libunwind.rs:96
   1: std::backtrace_rs::backtrace::trace_unsynchronized
             at ./library/std/src/../../backtrace/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at ./library/std/src/sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at ./library/std/src/sys_common/backtrace.rs:58
   4: core::fmt::write
             at ./library/core/src/fmt/mod.rs:1117
   5: std::io::Write::write_fmt
             at ./library/std/src/io/mod.rs:1510
   6: std::sys_common::backtrace::_print
             at ./library/std/src/sys_common/backtrace.rs:61
   7: std::sys_common::backtrace::print
             at ./library/std/src/sys_common/backtrace.rs:48
   8: std::panicking::default_hook::{{closure}}
             at ./library/std/src/panicking.rs:198
   9: std::panicking::default_hook
             at ./library/std/src/panicking.rs:217
  10: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
             at ./library/alloc/src/boxed.rs:1044
  11: rustc_driver::report_ice
             at ./src/librustc_driver/lib.rs:1158
  12: std::panicking::rust_panic_with_hook
             at ./library/std/src/panicking.rs:530
  13: std::panicking::begin_panic
             at ./library/std/src/panicking.rs:456
  14: rustc_errors::HandlerInner::panic_if_treat_err_as_bug
             at ./src/librustc_errors/lib.rs:941
  15: rustc_errors::HandlerInner::bump_err_count
             at ./src/librustc_errors/lib.rs:927
  16: rustc_errors::HandlerInner::emit_diagnostic
             at ./src/librustc_errors/lib.rs:767
  17: rustc_errors::Handler::emit_diag_at_span
             at ./src/librustc_errors/lib.rs:706
  18: rustc_errors::Handler::span_err
             at ./src/librustc_errors/lib.rs:605
  19: rustc_session::session::Session::span_err
             at ./src/librustc_session/session.rs:381
  20: rustc_resolve::macros::<impl rustc_resolve::Resolver>::compile_macro
             at ./src/librustc_resolve/macros.rs:1085
  21: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver>::get_macro_by_def_id
             at ./src/librustc_resolve/build_reduced_graph.rs:167
  22: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver>::get_macro
             at ./src/librustc_resolve/build_reduced_graph.rs:155
  23: rustc_resolve::macros::<impl rustc_resolve::Resolver>::check_reserved_macro_name
             at ./src/librustc_resolve/macros.rs:1057
  24: rustc_resolve::imports::<impl rustc_resolve::Resolver>::try_define
             at ./src/librustc_resolve/imports.rs:494
  25: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver>::define
             at ./src/librustc_resolve/build_reduced_graph.rs:93
  26: rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor::build_reduced_graph_for_external_crate_res
  27: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver>::build_reduced_graph_external
             at ./src/librustc_resolve/build_reduced_graph.rs:190
  28: rustc_resolve::Resolver::resolutions
             at ./src/librustc_resolve/lib.rs:1487
  29: rustc_resolve::ModuleData::for_each_child
             at ./src/librustc_resolve/lib.rs:514
  30: rustc_resolve::diagnostics::<impl rustc_resolve::Resolver>::lookup_import_candidates_from_module
             at ./src/librustc_resolve/diagnostics.rs:738
  31: rustc_resolve::diagnostics::<impl rustc_resolve::Resolver>::lookup_import_candidates
             at ./src/librustc_resolve/diagnostics.rs:876
  32: rustc_resolve::Resolver::resolve_path_with_ribs
             at ./src/librustc_resolve/lib.rs:2331
  33: rustc_resolve::Resolver::resolve_path
             at ./src/librustc_resolve/lib.rs:2081
  34: rustc_resolve::Resolver::resolve_ast_path
             at ./src/librustc_resolve/lib.rs:3091
  35: rustc_resolve::Resolver::resolve_str_path_error
             at ./src/librustc_resolve/lib.rs:3080
  36: rustdoc::passes::collect_intra_doc_links::LinkCollector::macro_resolve::{{closure}}
             at ./src/librustdoc/passes/collect_intra_doc_links.rs:158
  37: rustc_interface::passes::BoxedResolver::access::{{closure}}
             at ./src/librustc_data_structures/box_region.rs:113
  38: rustc_interface::passes::configure_and_expand::{{closure}}
             at ./src/librustc_interface/passes.rs:135
  39: alloc::boxed::<impl core::ops::generator::Generator<R> for core::pin::Pin<alloc::boxed::Box<G>>>::resume
             at ./library/alloc/src/boxed.rs:1145
  40: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::access
             at ./src/librustc_data_structures/box_region.rs:55
  41: rustc_interface::passes::BoxedResolver::access
             at ./src/librustc_data_structures/box_region.rs:119
  42: rustdoc::core::DocContext::enter_resolver
             at ./src/librustdoc/core.rs:83
  43: rustdoc::passes::collect_intra_doc_links::LinkCollector::macro_resolve
             at ./src/librustdoc/passes/collect_intra_doc_links.rs:141
  44: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::fold::DocFolder>::fold_item
             at ./src/librustdoc/passes/collect_intra_doc_links.rs:699
  45: rustdoc::fold::DocFolder::fold_inner_recur::{{closure}}
             at ./src/librustdoc/fold.rs:61
  46: core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut
             at ./library/core/src/ops/function.rs:275
  47: core::iter::traits::iterator::Iterator::find_map::check::{{closure}}
             at ./library/core/src/iter/traits/iterator.rs:2238
  48: core::iter::traits::iterator::Iterator::try_fold
             at ./library/core/src/iter/traits/iterator.rs:1870
  49: core::iter::traits::iterator::Iterator::find_map
             at ./library/core/src/iter/traits/iterator.rs:2244
  50: <core::iter::adapters::FilterMap<I,F> as core::iter::traits::iterator::Iterator>::next
             at ./library/core/src/iter/adapters/mod.rs:1132
  51: alloc::vec::Vec<T>::extend_desugared
             at ./library/alloc/src/vec.rs:2254
  52: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::spec_extend
             at ./library/alloc/src/vec.rs:2147
  53: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
             at ./library/alloc/src/vec.rs:2142
  54: <alloc::vec::Vec<T> as core::iter::traits::collect::FromIterator<T>>::from_iter
             at ./library/alloc/src/vec.rs:2032
  55: core::iter::traits::iterator::Iterator::collect
             at ./library/core/src/iter/traits/iterator.rs:1652
  56: rustdoc::fold::DocFolder::fold_crate
             at ./src/librustdoc/fold.rs:98
  57: rustdoc::passes::collect_intra_doc_links::collect_intra_doc_links
             at ./src/librustdoc/passes/collect_intra_doc_links.rs:42
  58: rustdoc::core::run_core::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustdoc/core.rs:601
  59: rustc_middle::ty::context::tls::enter_global::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1743
  60: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1720
  61: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1704
  62: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1720
  63: rustc_middle::ty::context::tls::enter_global
             at ./src/librustc_middle/ty/context.rs:1743
  64: rustc_interface::passes::QueryContext::enter
             at ./src/librustc_interface/passes.rs:755
  65: rustdoc::core::run_core::{{closure}}::{{closure}}
             at ./src/librustdoc/core.rs:451
  66: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./src/librustc_interface/queries.rs:385
  67: rustdoc::core::run_core::{{closure}}
             at ./src/librustdoc/core.rs:413
  68: rustc_interface::interface::create_compiler_and_run::{{closure}}
             at ./src/librustc_interface/interface.rs:192
  69: rustc_span::with_source_map
             at ./src/librustc_span/lib.rs:736
  70: rustc_interface::interface::create_compiler_and_run
             at ./src/librustc_interface/interface.rs:186
  71: rustdoc::core::run_core
             at ./src/librustdoc/core.rs:412
  72: rustdoc::main_options::{{closure}}
             at ./src/librustdoc/lib.rs:507
  73: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at ./library/std/src/panic.rs:318
  74: std::panicking::try::do_call
             at ./library/std/src/panicking.rs:348
  75: std::panicking::try
             at ./library/std/src/panicking.rs:325
  76: std::panic::catch_unwind
             at ./library/std/src/panic.rs:394
  77: rustc_driver::catch_fatal_errors
             at ./src/librustc_driver/lib.rs:1123
  78: rustdoc::main_options
             at ./src/librustdoc/lib.rs:503
  79: rustdoc::main_args::{{closure}}
             at ./src/librustdoc/lib.rs:438
  80: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:151
  81: scoped_tls::ScopedKey<T>::set
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
  82: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:147
  83: scoped_tls::ScopedKey<T>::set
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
  84: rustc_ast::attr::with_session_globals::{{closure}}
             at ./src/librustc_ast/attr/mod.rs:49
  85: scoped_tls::ScopedKey<T>::set
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
  86: rustc_ast::attr::with_session_globals
             at ./src/librustc_ast/attr/mod.rs:48
  87: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}
             at ./src/librustc_interface/util.rs:146
  88: rustc_interface::util::scoped_thread::{{closure}}
             at ./src/librustc_interface/util.rs:121
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
