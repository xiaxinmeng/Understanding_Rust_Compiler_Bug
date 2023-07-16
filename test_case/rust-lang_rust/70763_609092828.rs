
centril@centrilnas2:~/programming/rust/rust-gamma$ rustc +gamma-stage1 foo.rs
error: expected one of `(`, `)`, `,`, `::`, or `=`, found `==`
 --> foo.rs:1:33
  |
1 | const _: _ = [#[cfg(ident(ident == "arm"))] 0];
  |                                 ^^ expected one of `(`, `)`, `,`, `::`, or `=`

   0: rustc_ast::mut_visit::visit_clobber::{{closure}}
             at ./src/librustc_ast/mut_visit.rs:316
   1: core::result::Result<T,E>::unwrap_or_else
             at ./src/libcore/result.rs:851
      rustc_ast::mut_visit::visit_clobber
             at ./src/librustc_ast/mut_visit.rs:315
   2: <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::visit_expr
             at src/librustc_expand/expand.rs:1171
      rustc_ast::mut_visit::noop_visit_item_kind::{{closure}}
             at ./src/librustc_ast/mut_visit.rs:887
      rustc_ast::mut_visit::visit_opt
             at ./src/librustc_ast/mut_visit.rs:341
      rustc_ast::mut_visit::noop_visit_item_kind
             at ./src/librustc_ast/mut_visit.rs:887
   3: <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::visit_item_kind     
             at src/librustc_expand/expand.rs:1642
      rustc_ast::mut_visit::noop_flat_map_item
             at ./src/librustc_ast/mut_visit.rs:1017
   4: <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::flat_map_item       
             at src/librustc_expand/expand.rs:1521
   5: rustc_ast::mut_visit::noop_visit_mod::{{closure}}
             at ./src/librustc_ast/mut_visit.rs:976
      <alloc::vec::Vec<T> as rustc_ast::util::map_in_place::MapInPlace<T>>::flat_map_in_place
             at ./src/librustc_ast/util/map_in_place.rs:36
   6: rustc_ast::mut_visit::noop_visit_mod
             at ./src/librustc_ast/mut_visit.rs:976
      rustc_ast::mut_visit::MutVisitor::visit_mod
             at ./src/librustc_ast/mut_visit.rs:166
      rustc_ast::mut_visit::noop_visit_item_kind
             at ./src/librustc_ast/mut_visit.rs:894
   7: <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::visit_item_kind     
             at src/librustc_expand/expand.rs:1642
      rustc_ast::mut_visit::noop_flat_map_item
             at ./src/librustc_ast/mut_visit.rs:1017
   8: <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::flat_map_item       
             at src/librustc_expand/expand.rs:1521
   9: rustc_expand::expand::AstFragment::mut_visit_with::{{closure}}
             at src/librustc_expand/expand.rs:121
      <smallvec::SmallVec<A> as rustc_ast::util::map_in_place::MapInPlace<T>>::flat_map_in_place
             at ./src/librustc_ast/util/map_in_place.rs:82
  10: rustc_expand::expand::AstFragment::mut_visit_with
             at src/librustc_expand/expand.rs:121
      rustc_expand::expand::MacroExpander::collect_invocations
             at src/librustc_expand/expand.rs:577
  11: rustc_expand::expand::MacroExpander::fully_expand_fragment
             at src/librustc_expand/expand.rs:396
  12: rustc_expand::expand::MacroExpander::expand_crate
             at src/librustc_expand/expand.rs:363
  13: rustc_interface::passes::configure_and_expand_inner::{{closure}}::{{closure}}
             at src/librustc_interface/passes.rs:302
      rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./src/librustc_data_structures/profiling.rs:569
      rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./src/librustc_session/utils.rs:9
      rustc_interface::passes::configure_and_expand_inner::{{closure}}
             at src/librustc_interface/passes.rs:302
      rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./src/librustc_data_structures/profiling.rs:569
      rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./src/librustc_session/utils.rs:9
  14: rustc_interface::passes::configure_and_expand_inner
             at src/librustc_interface/passes.rs:256
  15: rustc_interface::passes::configure_and_expand::{{closure}}
             at src/librustc_interface/passes.rs:115
  16: alloc::boxed::<impl core::ops::generator::Generator<R> for core::pin::Pin<alloc::boxed::Box<G>>>::resume
             at ./src/liballoc/boxed.rs:1115
      rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new
             at ./src/librustc_data_structures/box_region.rs:34
  17: rustc_interface::passes::BoxedResolver::new
             at ./<::rustc_data_structures::box_region::declare_box_region_type macros>:12
      rustc_interface::passes::configure_and_expand
             at src/librustc_interface/passes.rs:112
  18: rustc_interface::queries::Queries::expansion::{{closure}}
             at src/librustc_interface/queries.rs:176
      rustc_interface::queries::Query<T>::compute
             at src/librustc_interface/queries.rs:34
      rustc_interface::queries::Queries::expansion
             at src/librustc_interface/queries.rs:172
  19: rustc_driver::run_compiler::{{closure}}::{{closure}}
             at src/librustc_driver/lib.rs:337
      rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./src/librustc_interface/queries.rs:385
      rustc_driver::run_compiler::{{closure}}
             at src/librustc_driver/lib.rs:286
      rustc_interface::interface::run_compiler_in_existing_thread_pool
             at ./src/librustc_interface/interface.rs:199
  20: rustc_interface::interface::run_compiler::{{closure}}
             at ./src/librustc_interface/interface.rs:213
      rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:154
      scoped_tls::ScopedKey<T>::set
             at /home/centril/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
      rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:150
      scoped_tls::ScopedKey<T>::set
             at /home/centril/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
      rustc_ast::attr::with_globals::{{closure}}
             at ./src/librustc_ast/attr/mod.rs:44
      scoped_tls::ScopedKey<T>::set
             at /home/centril/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
      rustc_ast::attr::with_globals
             at ./src/librustc_ast/attr/mod.rs:44
  21: rustc_interface::util::spawn_thread_pool::{{closure}}
             at ./src/librustc_interface/util.rs:149
      rustc_interface::util::scoped_thread::{{closure}}
             at ./src/librustc_interface/util.rs:124
      std::sys_common::backtrace::__rust_begin_short_backtrace
             at ./src/libstd/sys_common/backtrace.rs:130
  22: std::thread::Builder::spawn_unchecked::{{closure}}::{{closure}}
             at ./src/libstd/thread/mod.rs:475
      <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at ./src/libstd/panic.rs:318
      std::panicking::try::do_call
             at ./src/libstd/panicking.rs:331
  23: std::panicking::try::do_try
             at src/libstd/panicking.rs:298
  24: std::panicking::try
             at ./src/libstd/panicking.rs:274
      std::panic::catch_unwind
             at ./src/libstd/panic.rs:394
      std::thread::Builder::spawn_unchecked::{{closure}}
             at ./src/libstd/thread/mod.rs:474
      core::ops::function::FnOnce::call_once{{vtable.shim}}
             at ./src/libcore/ops/function.rs:232
  25: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
             at ./src/liballoc/boxed.rs:1008
  26: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
             at ./src/liballoc/boxed.rs:1008
      std::sys_common::thread::start_thread
             at src/libstd/sys_common/thread.rs:13
  27: std::sys::unix::thread::Thread::new::thread_start
             at src/libstd/sys/unix/thread.rs:80
  28: start_thread
             at /build/glibc-KRRWSm/glibc-2.29/nptl/pthread_create.c:486
  29: __clone

Aborted (core dumped)
