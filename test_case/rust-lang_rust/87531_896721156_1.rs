
error: internal compiler error: compiler/rustc_trait_selection/src/traits/codegen.rs:78:17: Encountered error `OutputTypeParameterMismatch(Binder(<[closure@src/main.rs:33:38: 33:44] as std::ops::Fn<(<WorldFetch as Fetch<'_>>::Item,)>>, [Region(BrAnon(0))]), Binder(<[closure@src/main.rs:33:38: 33:44] as std::ops::Fn<((),)>>, []), Sorts(ExpectedFound { expected: (), found: <WorldFetch as Fetch<'_>>::Item }))` selecting `Binder(<[closure@src/main.rs:33:38: 33:44] as std::ops::Fn<((),)>>, [])` during codegen

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1046:9
stack backtrace:
   0:     0x7fc0c52a9d80 - std::backtrace_rs::backtrace::libunwind::trace::h820df6e5a36600f2
                               at /rustc/ae90dcf0207c57c3034f00b07048d63f8b2363c8/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7fc0c52a9d80 - std::backtrace_rs::backtrace::trace_unsynchronized::h08422e102b839b36
                               at /rustc/ae90dcf0207c57c3034f00b07048d63f8b2363c8/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fc0c52a9d80 - std::sys_common::backtrace::_print_fmt::h57fa141da5c1d78f
                               at /rustc/ae90dcf0207c57c3034f00b07048d63f8b2363c8/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7fc0c52a9d80 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he771973a791a4e00
                               at /rustc/ae90dcf0207c57c3034f00b07048d63f8b2363c8/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7fc0c5317f4c - core::fmt::write::h9a6d9c74526a6c1b
                               at /rustc/ae90dcf0207c57c3034f00b07048d63f8b2363c8/library/core/src/fmt/mod.rs:1117:17
   5:     0x7fc0c529b685 - std::io::Write::write_fmt::h9d3df603db3c3ca4
                               at /rustc/ae90dcf0207c57c3034f00b07048d63f8b2363c8/library/std/src/io/mod.rs:1667:15
   6:     0x7fc0c52ad33b - std::sys_common::backtrace::_print::ha158db1ea88125c4
                               at /rustc/ae90dcf0207c57c3034f00b07048d63f8b2363c8/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7fc0c52ad33b - std::sys_common::backtrace::print::h342dde07e263fe00
                               at /rustc/ae90dcf0207c57c3034f00b07048d63f8b2363c8/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7fc0c52ad33b - std::panicking::default_hook::{{closure}}::h642e95a669718f6e
                               at /rustc/ae90dcf0207c57c3034f00b07048d63f8b2363c8/library/std/src/panicking.rs:210:50
   9:     0x7fc0c52aceb0 - std::panicking::default_hook::hc5055134d92c4284
                               at /rustc/ae90dcf0207c57c3034f00b07048d63f8b2363c8/library/std/src/panicking.rs:227:9
  10:     0x7fc0c5a8d731 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::h304e91c4dd9ccf1e
  11:     0x7fc0c52adb69 - std::panicking::rust_panic_with_hook::h6f45f3f96085621d
                               at /rustc/ae90dcf0207c57c3034f00b07048d63f8b2363c8/library/std/src/panicking.rs:628:17
  12:     0x7fc0c6a4dd5b - std::panicking::begin_panic::{{closure}}::ha486b1482929756f
  13:     0x7fc0c6a4dce6 - std::sys_common::backtrace::__rust_end_short_backtrace::h3f9d04b762f19ede
  14:     0x7fc0c6a4edff - std::panicking::begin_panic::h08c493e41a1b6bd8
  15:     0x7fc0c6a6327d - std::panic::panic_any::hcbbea2fdace6d4dc
  16:     0x7fc0c6a65cda - rustc_errors::HandlerInner::bug::hf98d0be0ebba3966
  17:     0x7fc0c6a65790 - rustc_errors::Handler::bug::h04eb0f7808456322
  18:     0x7fc0c69c233c - rustc_middle::ty::context::tls::with_opt::hde5a02b3e340c168
  19:     0x7fc0c69c2540 - rustc_middle::util::bug::opt_span_bug_fmt::h9a28451e10b9fa93
  20:     0x7fc0c69c24b6 - rustc_middle::util::bug::bug_fmt::h23a56cbc4ca6dee5
  21:     0x7fc0c735cfaa - rustc_infer::infer::InferCtxtBuilder::enter::h2c875bb32a972d15
  22:     0x7fc0c73b5ef7 - rustc_trait_selection::traits::codegen::codegen_fulfill_obligation::h64001017f09350a7
  23:     0x7fc0c7ab8900 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h15dacd4ce886ebae
  24:     0x7fc0c7b6a225 - rustc_data_structures::stack::ensure_sufficient_stack::hd63598e46c5cd730
  25:     0x7fc0c70eacba - rustc_query_system::query::plumbing::get_query_impl::h5771d252c00e96d7
  26:     0x7fc0c71627f2 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::codegen_fulfill_obligation::ha77cd79506ae5449
  27:     0x7fc0c6d77333 - rustc_ty_utils::instance::inner_resolve_instance::h64355d6972112dfe
  28:     0x7fc0c6d76186 - rustc_ty_utils::instance::resolve_instance::h99586070619f4be5
  29:     0x7fc0c7ac4b3e - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h5a353927bfbbe82e
  30:     0x7fc0c7b5f464 - rustc_data_structures::stack::ensure_sufficient_stack::h12f961eef1da38a0
  31:     0x7fc0c710eadb - rustc_query_system::query::plumbing::get_query_impl::hefa4374c8aca5d4c
  32:     0x7fc0c7163f45 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::resolve_instance::h37b1745f8a1fae47
  33:     0x7fc0c74aaa74 - rustc_middle::ty::instance::Instance::resolve_opt_const_arg::ha9dd19d17d2fa702
  34:     0x7fc0c74a9fe9 - rustc_middle::ty::instance::Instance::resolve::h3c499c49724ee011
  35:     0x7fc0c70d1fd0 - <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc_middle::mir::visit::Visitor>::visit_terminator::h992a7e10a89b7b17
  36:     0x7fc0c70d4cfc - rustc_mir::monomorphize::collector::collect_neighbours::h5a3f80fec8cd8ab1
  37:     0x7fc0c70cf30d - rustc_mir::monomorphize::collector::collect_items_rec::h44c2beac1ba7820d
  38:     0x7fc0c70cf45a - rustc_mir::monomorphize::collector::collect_items_rec::h44c2beac1ba7820d
  39:     0x7fc0c70ce413 - rustc_mir::monomorphize::collector::collect_crate_mono_items::hd9a6bcfd88bea354
  40:     0x7fc0c6fa65b7 - rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items::h1a9a53fc99b46dc9
  41:     0x7fc0c63da80a - rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::h1996e29bec96e763
  42:     0x7fc0c63acbcd - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h6c4f76e4f6e71592
  43:     0x7fc0c644f414 - rustc_data_structures::stack::ensure_sufficient_stack::hc1ea62e05a73757d
  44:     0x7fc0c6208996 - rustc_query_system::query::plumbing::force_query_with_job::h5e7331dd38f3e059
  45:     0x7fc0c79dec2d - rustc_query_system::query::plumbing::get_query_impl::h15204a6dd2a7e64a
  46:     0x7fc0c7b22ac6 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items::h7c997d6b7c345890
  47:     0x7fc0c75deca8 - rustc_codegen_ssa::base::codegen_crate::he5549afa8d35de5a
  48:     0x7fc0c75fa8ba - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::h249f326ee2c13441
  49:     0x7fc0c757ac15 - rustc_interface::passes::QueryContext::enter::hd50f2d8d11038ed6
  50:     0x7fc0c7572a56 - rustc_interface::queries::Queries::ongoing_codegen::hca1e84ea243aab86
  51:     0x7fc0c754fe25 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hfc676051f08a93ae
  52:     0x7fc0c753f21c - rustc_span::with_source_map::h01f7ec78329392f8
  53:     0x7fc0c7550fda - rustc_interface::interface::create_compiler_and_run::h9515a6b09b966b58
  54:     0x7fc0c7543229 - scoped_tls::ScopedKey<T>::set::h8d6a24e36deb1132
  55:     0x7fc0c753f8ea - std::sys_common::backtrace::__rust_begin_short_backtrace::h652864f7c1a77965
  56:     0x7fc0c753eb25 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h9a66d3b50f91b159
  57:     0x7fc0c52ba683 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::he2a466339cf4c9af
                               at /rustc/ae90dcf0207c57c3034f00b07048d63f8b2363c8/library/alloc/src/boxed.rs:1636:9
  58:     0x7fc0c52ba683 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h1f2d848478877ebe
                               at /rustc/ae90dcf0207c57c3034f00b07048d63f8b2363c8/library/alloc/src/boxed.rs:1636:9
  59:     0x7fc0c52ba683 - std::sys::unix::thread::Thread::new::thread_start::h12773e412063ea14
                               at /rustc/ae90dcf0207c57c3034f00b07048d63f8b2363c8/library/std/src/sys/unix/thread.rs:91:17
  60:     0x7fc0c51e6450 - start_thread
  61:     0x7fc0c50f4e73 - clone
  62:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (ae90dcf02 2021-08-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [codegen_fulfill_obligation] checking if `std::ops::Fn` fulfills its obligations
#1 [resolve_instance] resolving instance `<[closure@src/main.rs:33:38: 33:44] as std::ops::Fn<((),)>>::call`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: could not compile `abc`
