
> gdb --args rustc --crate-name hl_fuzzer --edition=2018 fuzzer/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="amd64-linux"' --cfg 'feature="default"' -C metadata=258ea3d5d4c9e7e0 -C extra-filename=-258ea3d5d4c9e7e0 --out-dir .../target/debug/deps -C incremental=.../target/debug/incremental -L dependency=.../target/debug/deps --extern hlang=.../target/debug/deps/libhlang-7040598e8269a156.rmeta

# Recursion starts here, leads to stack overflow.
...
#74319 0x00007ffff5510bb9 in rustc_data_structures::graph::iterate::post_order_walk () from .../nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74320 0x00007ffff5510bb9 in rustc_data_structures::graph::iterate::post_order_walk () from .../nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74321 0x00007ffff5510bb9 in rustc_data_structures::graph::iterate::post_order_walk () from .../nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74325 0x00007ffff5510ced in rustc_data_structures::graph::iterate::reverse_post_order () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74326 0x00007ffff5276f8d in rustc_data_structures::graph::dominators::dominators () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74327 0x00007ffff559b40b in rustc_mir::borrow_check::do_mir_borrowck () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74328 0x00007ffff50285e6 in rustc_infer::infer::InferCtxtBuilder::enter () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74329 0x00007ffff5596f49 in rustc_mir::borrow_check::mir_borrowck () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74330 0x00007ffff5560e35 in core::ops::function::FnOnce::call_once () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74331 0x00007ffff3fa7d25 in rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_borrowck>::compute () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74332 0x00007ffff3f4cf81 in rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74333 0x00007ffff3f11e87 in rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74334 0x00007ffff3fa9c41 in rustc_data_structures::stack::ensure_sufficient_stack () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74335 0x00007ffff3ef0b5c in rustc_query_system::query::plumbing::get_query_impl () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74336 0x00007ffff3ef5ff9 in rustc_query_system::query::plumbing::ensure_query_impl () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74337 0x00007ffff3fa4f73 in rustc_interface::passes::analysis () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74338 0x00007ffff3cded7b in rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74339 0x00007ffff3ce0df8 in rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74340 0x00007ffff3d7d0e4 in rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74341 0x00007ffff3d783df in rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}} () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74342 0x00007ffff3d0a444 in rustc_query_system::query::plumbing::get_query_impl () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74343 0x00007ffff3ce112b in rustc_interface::passes::QueryContext::enter () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74344 0x00007ffff3d7a4a2 in rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74345 0x00007ffff3d412a2 in rustc_span::with_source_map () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74346 0x00007ffff3d7b9c2 in rustc_interface::interface::create_compiler_and_run () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74347 0x00007ffff3d63d5a in scoped_tls::ScopedKey<T>::set () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74348 0x00007ffff3d809b5 in std::sys_common::backtrace::__rust_begin_short_backtrace () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74349 0x00007ffff3cfe3ce in core::ops::function::FnOnce::call_once{{vtable-shim}} () from .../toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-49c72af66a9d25fd.so
#74350 0x00007ffff34a621a in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/ffa2e7ae8fbf9badc035740db949b9dae271c29f/library/alloc/src/boxed.rs:1042
#74351 <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/ffa2e7ae8fbf9badc035740db949b9dae271c29f/library/alloc/src/boxed.rs:1042
#74352 std::sys::unix::thread::Thread::new::thread_start () at library/std/src/sys/unix/thread.rs:89
#74353 0x00007ffff33d0609 in start_thread (arg=<optimized out>) at pthread_create.c:477
#74354 0x00007ffff32e4293 in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
`
