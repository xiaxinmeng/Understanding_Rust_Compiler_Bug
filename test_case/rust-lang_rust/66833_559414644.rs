
$ RUST_BACKTRACE=1 cargo build
   Compiling gram v0.0.0 (/Users/stephan_boyer/Desktop/Dropbox/projects/gram)
error: internal compiler error: src/librustc/dep_graph/graph.rs:722: try_mark_previous_green() - Forcing the DepNode should have set its color

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:812:9
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::panicking::default_hook::{{closure}}
   4: std::panicking::default_hook
   5: rustc_driver::report_ice
   6: std::panicking::rust_panic_with_hook
   7: std::panicking::begin_panic
   8: rustc_errors::HandlerInner::bug
   9: rustc_errors::Handler::bug
  10: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  11: rustc::ty::context::tls::with_opt::{{closure}}
  12: rustc::ty::context::tls::with_context_opt
  13: rustc::ty::context::tls::with_opt
  14: rustc::util::bug::opt_span_bug_fmt
  15: rustc::util::bug::bug_fmt
  16: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  17: rustc::dep_graph::graph::DepGraph::try_mark_green
  18: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  19: rustc::ty::AdtDef::sized_constraint_for_ty
  20: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  21: rustc::ty::AdtDef::sized_constraint_for_ty
  22: <core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next
  23: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  24: <T as rustc::ty::context::InternIteratorElement<T,R>>::intern_with
  25: rustc::ty::adt_sized_constraint
  26: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::adt_sized_constraint>::compute
  27: rustc::dep_graph::graph::DepGraph::with_task_impl
  28: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query
  29: rustc::ty::query::plumbing::force_from_dep_node
  30: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  31: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  32: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  33: rustc::dep_graph::graph::DepGraph::try_mark_green
  34: rustc::dep_graph::graph::DepGraph::try_mark_green_and_read
  35: <rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor as rustc::hir::itemlikevisit::ParItemLikeVisitor>::visit_item
  36: __rust_maybe_catch_panic
  37: rustc_data_structures::sync::par_for_each_in
  38: __rust_maybe_catch_panic
  39: rustc::hir::Crate::par_visit_all_item_likes
  40: rustc::util::common::time
  41: rustc_typeck::check_crate
  42: rustc_interface::passes::analysis
  43: rustc::ty::query::__query_compute::analysis
  44: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  45: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  46: rustc_interface::passes::create_global_ctxt::{{closure}}
  47: rustc_interface::interface::run_compiler_in_existing_thread_pool
  48: std::thread::local::LocalKey<T>::with
  49: scoped_tls::ScopedKey<T>::set
  50: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0 (4560ea788 2019-11-04) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', src/libcore/result.rs:1165:5
stack backtrace:
   0:        0x10c428165 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66f38d4abb2e41c1
   1:        0x10c45ee10 - core::fmt::write::h4f487e714088986d
   2:        0x10c41b7bb - std::io::Write::write_fmt::h7bd7b6fe8b47c5fb
   3:        0x10c42c4aa - std::panicking::default_hook::{{closure}}::heee79636c241547c
   4:        0x10c42c1b5 - std::panicking::default_hook::hfcbd07059d15441e
   5:        0x1099c1762 - rustc_driver::report_ice::h5ad941de17a16320
   6:        0x10c42cce2 - std::panicking::rust_panic_with_hook::h0c4b67125f55410a
   7:        0x10c42c74d - std::panicking::continue_panic_fmt::h0e74ab2b215a1401
   8:        0x10c42c649 - rust_begin_unwind
   9:        0x10c4583bf - core::panicking::panic_fmt::h09741a3213dba543
  10:        0x10c4585c9 - core::result::unwrap_failed::hf47c31c429b02014
  11:        0x10b50e58b - rustc_errors::Handler::force_print_diagnostic::h0bfe3e394dc103fa
  12:        0x10ae65615 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::try_print_query_stack::ha545746afda6bf84
  13:        0x1099c2392 - rustc_driver::report_ice::h5ad941de17a16320
  14:        0x10c42cce2 - std::panicking::rust_panic_with_hook::h0c4b67125f55410a
  15:        0x10b4f9e51 - std::panicking::begin_panic::h15f5ca0459a0afbf
  16:        0x10b50ed37 - rustc_errors::HandlerInner::bug::h3ae5a8494001dbc6
  17:        0x10b50dbdd - rustc_errors::Handler::bug::h926b6bc1e5e81222
  18:        0x10ae21dfb - rustc::util::bug::opt_span_bug_fmt::{{closure}}::hc9e1ba37629e505a
  19:        0x10ae1c306 - rustc::ty::context::tls::with_opt::{{closure}}::h0f2d2c67a51c6ea3
  20:        0x10ae1c294 - rustc::ty::context::tls::with_context_opt::hffb989149536908e
  21:        0x10ae1c2c2 - rustc::ty::context::tls::with_opt::h8dbbf9943655c843
  22:        0x10ae21d18 - rustc::util::bug::opt_span_bug_fmt::haea6cb90fd22dc2a
  23:        0x10ae21c6b - rustc::util::bug::bug_fmt::hd6a2040ffa44f924
  24:        0x10abf2182 - rustc::dep_graph::graph::DepGraph::try_mark_previous_green::h21449f810f6a016f
  25:        0x10abf19e2 - rustc::dep_graph::graph::DepGraph::try_mark_green::hfec97d0739a2eebb
  26:        0x10af2bfc5 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hcf7398218abb6591
  27:        0x10b07e6dd - rustc::ty::AdtDef::sized_constraint_for_ty::ha628c55bacf85818
  28:        0x10ad02fa3 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter::h14120fe53c10d1fd
  29:        0x10b07e73d - rustc::ty::AdtDef::sized_constraint_for_ty::ha628c55bacf85818
  30:        0x10ac7df5e - <core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next::h3c0623c7b5f31150
  31:        0x10ac82138 - <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter::h19b017ae0909c34e
  32:        0x10b073a30 - <T as rustc::ty::context::InternIteratorElement<T,R>>::intern_with::h6fb225ad59d51420
  33:        0x10b07f676 - rustc::ty::adt_sized_constraint::h7c462e65b7b989ac
  34:        0x10b02e2db - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::adt_sized_constraint>::compute::h806f847214726a1a
  35:        0x10abe7335 - rustc::dep_graph::graph::DepGraph::with_task_impl::he19b8e766de24210
  36:        0x10affaeb9 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query::hdd7b88141f33cd0d
  37:        0x10acd3db7 - rustc::ty::query::plumbing::force_from_dep_node::h9bac8709c2934cbd
  38:        0x10abf1c5a - rustc::dep_graph::graph::DepGraph::try_mark_previous_green::h21449f810f6a016f
  39:        0x10abf1bb6 - rustc::dep_graph::graph::DepGraph::try_mark_previous_green::h21449f810f6a016f
  40:        0x10abf1bb6 - rustc::dep_graph::graph::DepGraph::try_mark_previous_green::h21449f810f6a016f
  41:        0x10abf19e2 - rustc::dep_graph::graph::DepGraph::try_mark_green::hfec97d0739a2eebb
  42:        0x10abf1862 - rustc::dep_graph::graph::DepGraph::try_mark_green_and_read::he289ad57f05459e5
  43:        0x109ef31f0 - <rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor as rustc::hir::itemlikevisit::ParItemLikeVisitor>::visit_item::hca6fb8868cd499b2
  44:        0x10c43bf8f - __rust_maybe_catch_panic
  45:        0x109f6797d - rustc_data_structures::sync::par_for_each_in::h2e3ae9319adb57a7
  46:        0x10c43bf8f - __rust_maybe_catch_panic
  47:        0x109ee82b9 - rustc::hir::Crate::par_visit_all_item_likes::h1e30f806c135e4ec
  48:        0x109f0de12 - rustc::util::common::time::h22df5ddfaa5f956d
  49:        0x10a0b2a1c - rustc_typeck::check_crate::h4f0cc6c764300359
  50:        0x109a59e72 - rustc_interface::passes::analysis::h0bbd5fdf3a5c0c97
  51:        0x1099635fe - rustc::ty::query::__query_compute::analysis::h7396128e68e0db7f
  52:        0x10996f0a9 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h267adeeb0cdf18a0
  53:        0x10998f19c - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::hf2e56cdb05ffd121
  54:        0x109acc71b - rustc_interface::passes::create_global_ctxt::{{closure}}::h42bdcf19d1a42a9e
  55:        0x109991352 - rustc_interface::interface::run_compiler_in_existing_thread_pool::he7e390e79c6cfcaf
  56:        0x1099c3284 - std::thread::local::LocalKey<T>::with::h7cc2d0d20721bb64
  57:        0x1099c75c2 - scoped_tls::ScopedKey<T>::set::h1e5f389a2c4779b0
  58:        0x1099e0765 - syntax::with_globals::hccf9e749a1217bc9
  59:        0x10995fd1d - std::sys_common::backtrace::__rust_begin_short_backtrace::h09b527d5f5497ee9
  60:        0x10c43bf8f - __rust_maybe_catch_panic
  61:        0x109984f97 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h4448e4e8ceddd77c
  62:        0x10c40e00e - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hd1afe4d791e3679f
  63:        0x10c43ad9e - std::sys::unix::thread::Thread::new::thread_start::h785fcf8d81bd0f52
  64:     0x7fff65ec2d36 - _pthread_start

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0 (4560ea788 2019-11-04) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
thread panicked while processing panic. aborting.
error: could not compile `gram`.

Caused by:
  process didn't exit successfully: `rustc --edition=2018 --crate-name gram src/main.rs --color always --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=0a1de5ceed462ebd -C extra-filename=-0a1de5ceed462ebd --out-dir /Users/stephan_boyer/Desktop/Dropbox/projects/gram/target/debug/deps -C incremental=/Users/stephan_boyer/Desktop/Dropbox/projects/gram/target/debug/incremental -L dependency=/Users/stephan_boyer/Desktop/Dropbox/projects/gram/target/debug/deps --extern atty=/Users/stephan_boyer/Desktop/Dropbox/projects/gram/target/debug/deps/libatty-739cef6896daafd2.rlib --extern clap=/Users/stephan_boyer/Desktop/Dropbox/projects/gram/target/debug/deps/libclap-531ce504e9eef3c0.rlib --extern colored=/Users/stephan_boyer/Desktop/Dropbox/projects/gram/target/debug/deps/libcolored-381bee60be1a8230.rlib --extern pad=/Users/stephan_boyer/Desktop/Dropbox/projects/gram/target/debug/deps/libpad-4c62b3764aee07b4.rlib` (signal: 4, SIGILL: illegal instruction)
