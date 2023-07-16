
error: internal compiler error: compiler/rustc_borrowck/src/universal_regions.rs:782:36: cannot convert `RePlaceholder(Placeholder { universe: U2, name: BrAnon(1) })` to a region vid

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1146:9
stack backtrace:
   0:     0x7ff6c79af4dc - std::backtrace_rs::backtrace::libunwind::trace::h2ab374bc2a3b7023
                               at /rustc/308dffd25cb55bbb4a1fbee9822cf82c6a5d012d/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7ff6c79af4dc - std::backtrace_rs::backtrace::trace_unsynchronized::h128cb5178b04dc46
                               at /rustc/308dffd25cb55bbb4a1fbee9822cf82c6a5d012d/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7ff6c79af4dc - std::sys_common::backtrace::_print_fmt::h5344f9eefca2041f
                               at /rustc/308dffd25cb55bbb4a1fbee9822cf82c6a5d012d/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7ff6c79af4dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h213003bc5c7acf04
                               at /rustc/308dffd25cb55bbb4a1fbee9822cf82c6a5d012d/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7ff6c7a0d75c - core::fmt::write::h78bf85fc3e93663f
                               at /rustc/308dffd25cb55bbb4a1fbee9822cf82c6a5d012d/library/core/src/fmt/mod.rs:1126:17
   5:     0x7ff6c79a03d5 - std::io::Write::write_fmt::he619515c888f21a5
                               at /rustc/308dffd25cb55bbb4a1fbee9822cf82c6a5d012d/library/std/src/io/mod.rs:1667:15
   6:     0x7ff6c79b2a40 - std::sys_common::backtrace::_print::hf706674f77848203
                               at /rustc/308dffd25cb55bbb4a1fbee9822cf82c6a5d012d/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7ff6c79b2a40 - std::sys_common::backtrace::print::hf0b6c7a88804ec56
                               at /rustc/308dffd25cb55bbb4a1fbee9822cf82c6a5d012d/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7ff6c79b2a40 - std::panicking::default_hook::{{closure}}::h2dde766cd83a333a
                               at /rustc/308dffd25cb55bbb4a1fbee9822cf82c6a5d012d/library/std/src/panicking.rs:210:50
   9:     0x7ff6c79b25f7 - std::panicking::default_hook::h501e3b2e134eb149
                               at /rustc/308dffd25cb55bbb4a1fbee9822cf82c6a5d012d/library/std/src/panicking.rs:227:9
  10:     0x7ff6c8195241 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::h8ff5de92123cb8fd
  11:     0x7ff6c79b3259 - std::panicking::rust_panic_with_hook::hc09e869c4cf00885
                               at /rustc/308dffd25cb55bbb4a1fbee9822cf82c6a5d012d/library/std/src/panicking.rs:628:17
  12:     0x7ff6c922228b - std::panicking::begin_panic::{{closure}}::h5c7a29a9f3c50d8d
  13:     0x7ff6c92221c6 - std::sys_common::backtrace::__rust_end_short_backtrace::h11b7892876009c1e
  14:     0x7ff6c922225f - std::panicking::begin_panic::hd63e7fe0ba9182da
  15:     0x7ff6c92321dd - std::panic::panic_any::h3c06e175d31a9167
  16:     0x7ff6c9234740 - rustc_errors::HandlerInner::bug::he6d06053695a98ed
  17:     0x7ff6c9233f60 - rustc_errors::Handler::bug::h61abf9991ffffcbc
  18:     0x7ff6c90b0fde - rustc_middle::ty::context::tls::with_opt::hd0b25ef1fa372784
  19:     0x7ff6c90b14b0 - rustc_middle::util::bug::opt_span_bug_fmt::he982db01d383152c
  20:     0x7ff6c90b1426 - rustc_middle::util::bug::bug_fmt::h3252ae0338f3fde6
  21:     0x7ff6c9777ab7 - rustc_borrowck::type_check::free_region_relations::UniversalRegionRelationsBuilder::add_implied_bounds::h6fc51cad360e4c45
  22:     0x7ff6c9774d2d - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once::he159d5dccad8f6c8
  23:     0x7ff6c9800e5e - <core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next::h58a65807831cb8e5
  24:     0x7ff6c980ddbf - <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter::h278c3d2eb57eb6fe
  25:     0x7ff6c9776e76 - rustc_borrowck::type_check::free_region_relations::create::h1e099acafe4805c1
  26:     0x7ff6c97e0b3c - rustc_borrowck::type_check::type_check::h3c4dfe3f7cb7800b
  27:     0x7ff6c97ac5dc - rustc_borrowck::nll::compute_regions::haa6cecc831d0b29a
  28:     0x7ff6c97bb62c - rustc_borrowck::do_mir_borrowck::h700e8947cfdadc30
  29:     0x7ff6c9793083 - rustc_infer::infer::InferCtxtBuilder::enter::h9b172b82676b5208
  30:     0x7ff6c97baa44 - rustc_borrowck::mir_borrowck::h43f04ab543d2f43f
  31:     0x7ff6c97b5d91 - core::ops::function::FnOnce::call_once::h8103cea540657a78
  32:     0x7ff6ca351731 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task::h289a71605f390793
  33:     0x7ff6ca2f9423 - rustc_data_structures::stack::ensure_sufficient_stack::h4b6e02134b2da06d
  34:     0x7ff6c98b2cbd - rustc_query_system::query::plumbing::try_execute_query::hb362ed745d75a2d5
  35:     0x7ff6c991c269 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_borrowck::hf27d489416d05e03
  36:     0x7ff6c934f1d8 - rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners::h227c8cd8436c8ccc
  37:     0x7ff6c9d4ecaa - rustc_interface::passes::analysis::hb918b02982537851
  38:     0x7ff6ca355b7f - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task::h347feb22cb0d1ecf
  39:     0x7ff6ca2f9559 - rustc_data_structures::stack::ensure_sufficient_stack::h4b6ffa4a2dfebaef
  40:     0x7ff6ca23d49b - rustc_query_system::query::plumbing::try_execute_query::h784269645558384c
  41:     0x7ff6ca3139f2 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis::ha8ab0b7748e9a5d1
  42:     0x7ff6c9d45b19 - rustc_interface::passes::QueryContext::enter::h805836b6c10e023c
  43:     0x7ff6c9d2caf4 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hded9061889acb733
  44:     0x7ff6c9d19e15 - rustc_span::with_source_map::hce3a664e96079636
  45:     0x7ff6c9d2c41c - scoped_tls::ScopedKey<T>::set::h033dd5b45ac9d9cd
  46:     0x7ff6c9d1ae4d - std::sys_common::backtrace::__rust_begin_short_backtrace::hc72d766e7b5b424b
  47:     0x7ff6c9d186b5 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hdbb85447b47f9f9d
  48:     0x7ff6c79bfdf3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h59eef3b9c8a82350
                               at /rustc/308dffd25cb55bbb4a1fbee9822cf82c6a5d012d/library/alloc/src/boxed.rs:1638:9
  49:     0x7ff6c79bfdf3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hb5bbe017c347469c
                               at /rustc/308dffd25cb55bbb4a1fbee9822cf82c6a5d012d/library/alloc/src/boxed.rs:1638:9
  50:     0x7ff6c79bfdf3 - std::sys::unix::thread::Thread::new::thread_start::h62931528f61e35f5
                               at /rustc/308dffd25cb55bbb4a1fbee9822cf82c6a5d012d/library/std/src/sys/unix/thread.rs:106:17
  51:     0x7ff6c78d5259 - start_thread
  52:     0x7ff6c77ea5e3 - __GI___clone
  53:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (308dffd25 2021-09-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C embed-bitcode=no -C debuginfo=2 -C debug-assertions=on -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] borrow-checking `main::{closure#2}`
#1 [analysis] running analysis passes on this crate
end of query stack
