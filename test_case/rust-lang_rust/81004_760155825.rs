rust
#0  0x00007f9c4cce2add in rustc_data_structures::obligation_forest::ObligationForest<O>::register_obligation_at () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#1  0x00007f9c4cce16b4 in rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#2  0x00007f9c4cc32af6 in <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#3  0x00007f9c4cc328bb in <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_all_or_error () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#4  0x00007f9c4b37dde4 in rustc_typeck::check::FnCtxt::select_all_obligations_or_error () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#5  0x00007f9c4b5fbb64 in rustc_infer::infer::InferCtxtBuilder::enter () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#6  0x00007f9c4b504952 in rustc_typeck::check::wfcheck::check_associated_item () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#7  0x00007f9c4b3672d7 in rustc_typeck::check::check_impl_item_well_formed () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#8  0x00007f9c4b4c1a9e in rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::check_impl_item_well_formed>::compute () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#9  0x00007f9c4b4c63b1 in rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#10 0x00007f9c4b540ab7 in rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#11 0x00007f9c4b554b93 in rustc_data_structures::stack::ensure_sufficient_stack () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#12 0x00007f9c4b41a7e0 in rustc_query_system::query::plumbing::get_query_impl () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#13 0x00007f9c4b46c71e in rustc_query_system::query::plumbing::ensure_query_impl () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#14 0x00007f9c4b66faf7 in rustc_data_structures::sync::par_for_each_in () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#15 0x00007f9c4b3b27db in rustc_hir::hir::Crate::par_visit_all_item_likes () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#16 0x00007f9c4b4a0fb8 in rustc_session::session::Session::track_errors () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#17 0x00007f9c4b5a5915 in rustc_typeck::check_crate () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#18 0x00007f9c48b0c83f in rustc_interface::passes::analysis () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#19 0x00007f9c487d0c2b in rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#20 0x00007f9c487339b8 in rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#21 0x00007f9c487b4747 in rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#22 0x00007f9c487c50d3 in rustc_data_structures::stack::ensure_sufficient_stack () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#23 0x00007f9c4875be33 in rustc_query_system::query::plumbing::get_query_impl () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#24 0x00007f9c487d1ffa in rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#25 0x00007f9c48765f27 in rustc_span::with_source_map () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#26 0x00007f9c487d3513 in rustc_interface::interface::create_compiler_and_run () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#27 0x00007f9c487b19fa in scoped_tls::ScopedKey<T>::set () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#28 0x00007f9c487c6957 in std::sys_common::backtrace::__rust_begin_short_backtrace () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#29 0x00007f9c4874fdae in core::ops::function::FnOnce::call_once{{vtable-shim}} () from /home/lzutao/.rustup/toolchains/1.47-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#30 0x00007f9c47b5df5a in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/alloc/src/boxed.rs:1042
#31 <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/alloc/src/boxed.rs:1042
#32 std::sys::unix::thread::Thread::new::thread_start () at library/std/src/sys/unix/thread.rs:87
#33 0x00007f9c474954a4 in start_thread () from /lib/x86_64-linux-gnu/libpthread.so.0
#34 0x00007f9c471d7d0f in clone () from /lib/x86_64-linux-gnu/libc.so.6
