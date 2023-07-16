rust
$ RUST_BACKTRACE=1 ./go
!! master-installed (default)
!! Executing '/home/user/.cargo/bin/cargo' in pwd='/home/user/build/2nonpkgs/rust.stuff/memdb' with args(1): 'build'
   Compiling memdb v1.0.0 (/home/user/build/2nonpkgs/rust.stuff/memdb)
error[E0507]: cannot move out of `*key` which is behind a shared reference
  --> src/lib.rs:28:74
   |
28 |     format!("Attempted to delete inexisting key '{}'", String::from_utf8(*key).unwrap())
   |                                                                          ^^^^ move occurs because `*key` has type `std::vec::Vec<u8>`, which does not implement the `Copy` trait

error: internal compiler error: src/librustc_mir/borrow_check/mod.rs:1949: Accessing `(*_3)` with the kind `Write(Move)` shouldn't be possible
  --> src/lib.rs:28:74
   |
28 |     format!("Attempted to delete inexisting key '{}'", String::from_utf8(*key).unwrap())
   |                                                                          ^^^^

thread '<unnamed>' panicked at 'Box<Any>', src/librustc_errors/lib.rs:871:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:76
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:60
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1028
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:64
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:196
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:210
  10: rustc_driver::report_ice
             at src/librustc_driver/lib.rs:1187
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:477
  12: std::panicking::begin_panic
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panicking.rs:407
  13: rustc_errors::HandlerInner::span_bug
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/<::std::macros::panic macros>:3
  14: rustc_errors::Handler::span_bug
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_errors/lib.rs:641
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/util/bug.rs:35
  16: rustc::ty::context::tls::with_opt::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1949
  17: rustc::ty::context::tls::with_context_opt
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1899
  18: rustc::ty::context::tls::with_opt
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1949
  19: rustc::util::bug::opt_span_bug_fmt
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/util/bug.rs:32
  20: rustc::util::bug::span_bug_fmt
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/util/bug.rs:23
  21: rustc_mir::borrow_check::MirBorrowckCtxt::check_access_permissions
             at src/librustc_mir/borrow_check/mod.rs:1949
  22: rustc_mir::borrow_check::MirBorrowckCtxt::access_place
             at src/librustc_mir/borrow_check/mod.rs:929
  23: rustc_mir::borrow_check::MirBorrowckCtxt::consume_operand
             at src/librustc_mir/borrow_check/mod.rs:0
  24: rustc_mir::borrow_check::MirBorrowckCtxt::consume_rvalue
             at src/librustc_mir/borrow_check/mod.rs:0
  25: <rustc_mir::borrow_check::MirBorrowckCtxt as rustc_mir::dataflow::DataflowResultsConsumer>::visit_statement_entry
             at src/librustc_mir/borrow_check/mod.rs:500
  26: rustc_mir::dataflow::DataflowResultsConsumer::process_basic_block
             at src/librustc_mir/dataflow/mod.rs:344
  27: rustc_mir::dataflow::DataflowResultsConsumer::analyze_results
             at src/librustc_mir/dataflow/mod.rs:332
  28: rustc_mir::borrow_check::do_mir_borrowck
             at src/librustc_mir/borrow_check/mod.rs:279
  29: rustc_mir::borrow_check::mir_borrowck::{{closure}}
             at src/librustc_mir/borrow_check/mod.rs:95
  30: rustc::infer::InferCtxtBuilder::enter::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/infer/mod.rs:528
  31: rustc::ty::context::GlobalCtxt::enter_local::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1599
  32: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1821
  33: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
  34: rustc::ty::context::tls::set_tlv
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1728
  35: rustc::ty::context::tls::enter_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1820
  36: rustc::ty::context::GlobalCtxt::enter_local::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1598
  37: rustc::ty::context::tls::with_related_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1927
  38: rustc::ty::context::tls::with_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1910
  39: rustc::ty::context::tls::with_context_opt
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1899
  40: rustc::ty::context::tls::with_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1910
  41: rustc::ty::context::tls::with_related_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1923
  42: rustc::ty::context::GlobalCtxt::enter_local
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1590
  43: rustc::infer::InferCtxtBuilder::enter
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/infer/mod.rs:527
  44: rustc_mir::borrow_check::mir_borrowck
             at src/librustc_mir/borrow_check/mod.rs:92
  45: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:1002
  46: rustc::ty::query::__query_compute::mir_borrowck
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:953
  47: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:994
  48: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:277
  49: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1821
  50: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
  51: rustc::ty::context::tls::set_tlv
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1728
  52: rustc::ty::context::tls::enter_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1820
  53: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:276
  54: rustc::ty::context::tls::with_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1910
  55: rustc::ty::context::tls::with_context_opt
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1899
  56: rustc::ty::context::tls::with_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1910
  57: rustc::dep_graph::graph::DepGraph::with_task_impl
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:270
  58: rustc::dep_graph::graph::DepGraph::with_task
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:202
  59: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:561
  60: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:277
  61: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1821
  62: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
  63: rustc::ty::context::tls::set_tlv
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1728
  64: rustc::ty::context::tls::enter_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1820
  65: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:276
  66: rustc::ty::context::tls::with_related_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1927
  67: rustc::ty::context::tls::with_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1910
  68: rustc::ty::context::tls::with_context_opt
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1899
  69: rustc::ty::context::tls::with_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1910
  70: rustc::ty::context::tls::with_related_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1923
  71: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:265
  72: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:553
  73: rustc::ty::query::plumbing::with_diagnostics
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:210
  74: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:552
  75: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:430
  76: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:615
  77: rustc::ty::query::TyCtxtEnsure::mir_borrowck
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:1030
  78: rustc_interface::passes::analysis::{{closure}}::{{closure}}
             at src/librustc_interface/passes.rs:939
  79: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/mod.rs:2797
  80: <rustc_rayon::iter::for_each::ForEachConsumer<F> as rustc_rayon::iter::plumbing::Folder<T>>::consume_iter::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-0.2.0/src/iter/for_each.rs:55
  81: <core::slice::Iter<T> as core::iter::traits::iterator::Iterator>::fold
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libcore/slice/mod.rs:3211
  82: <rustc_rayon::iter::for_each::ForEachConsumer<F> as rustc_rayon::iter::plumbing::Folder<T>>::consume_iter
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-0.2.0/src/iter/for_each.rs:55
  83: rustc_rayon::iter::plumbing::bridge_producer_consumer::helper
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-0.2.0/src/iter/plumbing/mod.rs:441
  84: rustc_rayon::iter::plumbing::bridge_producer_consumer::helper::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-0.2.0/src/iter/plumbing/mod.rs:430
  85: rustc_rayon_core::join::join_context::{{closure}}::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/join/mod.rs:130
  86: rustc_rayon_core::job::StackJob<L,F,R>::run_inline
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/job.rs:102
  87: rustc_rayon_core::join::join_context::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/join/mod.rs:160
  88: rustc_rayon_core::registry::in_worker
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:798
  89: rustc_rayon_core::join::join_context
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/join/mod.rs:119
  90: rustc_rayon::iter::plumbing::bridge_producer_consumer::helper
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-0.2.0/src/iter/plumbing/mod.rs:419
  91: rustc_rayon::iter::plumbing::bridge_producer_consumer::helper::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-0.2.0/src/iter/plumbing/mod.rs:430
  92: rustc_rayon_core::join::join_context::{{closure}}::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/join/mod.rs:130
  93: <rustc_rayon_core::job::StackJob<L,F,R> as rustc_rayon_core::job::Job>::execute::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/job.rs:121
  94: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panic.rs:315
  95: std::panicking::try::do_call
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panicking.rs:292
  96: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
  97: std::panicking::try
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panicking.rs:271
  98: std::panic::catch_unwind
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panic.rs:394
  99: rustc_rayon_core::unwind::halt_unwinding
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/unwind.rs:19
 100: <rustc_rayon_core::job::StackJob<L,F,R> as rustc_rayon_core::job::Job>::execute
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/job.rs:121
 101: rustc_rayon_core::job::JobRef::execute
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/job.rs:62
 102: rustc_rayon_core::registry::WorkerThread::execute
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:657
 103: rustc_rayon_core::registry::WorkerThread::wait_until_cold
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:637
 104: rustc_rayon_core::registry::WorkerThread::wait_until
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:611
 105: rustc_rayon_core::join::join_context::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/join/mod.rs:175
 106: rustc_rayon_core::registry::in_worker
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:798
 107: rustc_rayon_core::join::join_context
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/join/mod.rs:119
 108: rustc_rayon::iter::plumbing::bridge_producer_consumer::helper
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-0.2.0/src/iter/plumbing/mod.rs:419
 109: rustc_rayon::iter::plumbing::bridge_producer_consumer::helper::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-0.2.0/src/iter/plumbing/mod.rs:421
 110: rustc_rayon_core::join::join_context::{{closure}}::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/join/mod.rs:137
 111: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panic.rs:315
 112: std::panicking::try::do_call
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panicking.rs:292
 113: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
 114: std::panicking::try
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panicking.rs:271
 115: std::panic::catch_unwind
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panic.rs:394
 116: rustc_rayon_core::unwind::halt_unwinding
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/unwind.rs:19
 117: rustc_rayon_core::join::join_context::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/join/mod.rs:137
 118: rustc_rayon_core::registry::in_worker
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:798
 119: rustc_rayon_core::join::join_context
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/join/mod.rs:119
 120: rustc_rayon::iter::plumbing::bridge_producer_consumer::helper
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-0.2.0/src/iter/plumbing/mod.rs:419
 121: rustc_rayon::iter::plumbing::bridge_producer_consumer::helper::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-0.2.0/src/iter/plumbing/mod.rs:430
 122: rustc_rayon_core::join::join_context::{{closure}}::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/join/mod.rs:130
 123: <rustc_rayon_core::job::StackJob<L,F,R> as rustc_rayon_core::job::Job>::execute::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/job.rs:121
 124: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panic.rs:315
 125: std::panicking::try::do_call
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panicking.rs:292
 126: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
 127: std::panicking::try
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panicking.rs:271
 128: std::panic::catch_unwind
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panic.rs:394
 129: rustc_rayon_core::unwind::halt_unwinding
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/unwind.rs:19
 130: <rustc_rayon_core::job::StackJob<L,F,R> as rustc_rayon_core::job::Job>::execute
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/job.rs:121
 131: rustc_rayon_core::job::JobRef::execute
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/job.rs:62
 132: rustc_rayon_core::registry::WorkerThread::execute
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:657
 133: rustc_rayon_core::registry::WorkerThread::wait_until_cold
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:637
 134: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_interface/util.rs:235
 135: scoped_tls::ScopedKey<T>::set
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 136: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_interface/util.rs:235
 137: rustc::ty::context::tls::with_thread_locals::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1809
 138: std::thread::local::LocalKey<T>::try_with
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/thread/local.rs:262
 139: std::thread::local::LocalKey<T>::with
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/thread/local.rs:239
 140: rustc::ty::context::tls::with_thread_locals::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1801
 141: std::thread::local::LocalKey<T>::try_with
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/thread/local.rs:262
 142: std::thread::local::LocalKey<T>::with
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/thread/local.rs:239
 143: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_interface/util.rs:234
 144: scoped_tls::ScopedKey<T>::set
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 145: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_interface/util.rs:230
 146: scoped_tls::ScopedKey<T>::set
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 147: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_interface/util.rs:229
 148: rustc_rayon_core::thread_pool::ThreadPool::scoped_pool::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/thread_pool/mod.rs:104
 149: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
 150: std::panicking::try
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panicking.rs:271
 151: std::panic::catch_unwind
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panic.rs:394
 152: rustc_rayon_core::unwind::halt_unwinding
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/unwind.rs:19
 153: rustc_rayon_core::registry::main_loop
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:747
 154: rustc_rayon_core::registry::Registry::new::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:145
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-dev (f3c8eba64 2019-09-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z treat-err-as-bug=5 -Z external-macro-backtrace -C debuginfo=2 -C incremental -C target-cpu=native --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
    Building [=======================================================> ] 63/64: memdb                                                                    

