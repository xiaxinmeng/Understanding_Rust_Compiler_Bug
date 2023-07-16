
#0  0x00007ffb2154535e in ntdll!RtlUnwindEx () from /c/Windows/SYSTEM32/ntdll.dll
#1  0x0000000067d07009 in _Unwind_Resume () from /home/we/rust/build/x86_64-pc-windows-gnu/stage1/bin/std-f942dabaf1bc2c38.dll
#2  0x0000000067cce0f9 in rust_eh_unwind_resume () at src\libpanic_unwind/gcc.rs:325
#3  0x00000000007fba64 in rustc_data_structures::profiling::VerboseTimingGuard::run () at C:\msys64\home\we\rust\src\librustc_data_structures/profiling.rs:567
#4  rustc_session::utils::<impl rustc_session::session::Session>::time () at C:\msys64\home\we\rust\src\librustc_session/utils.rs:9
#5  rustc_interface::passes::analysis::{{closure}}::{{closure}} () at src\librustc_interface/passes.rs:775
#6  core::ops::function::FnOnce::call_once () at C:\msys64\home\we\rust\src\libcore\ops/function.rs:232
#7  <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once () at C:\msys64\home\we\rust\src\libstd/panic.rs:318
#8  std::panicking::try::do_call () at C:\msys64\home\we\rust\src\libstd/panicking.rs:330
#9  std::panicking::try::do_try () at C:\msys64\home\we\rust\src\libstd/panicking.rs:298
#10 std::panicking::try () at C:\msys64\home\we\rust\src\libstd/panicking.rs:275
#11 std::panic::catch_unwind () at C:\msys64\home\we\rust\src\libstd/panic.rs:394
#12 0x00000000007b2ab1 in rustc_interface::passes::analysis::{{closure}} () at src\librustc_interface/passes.rs:770
#13 rustc_data_structures::profiling::VerboseTimingGuard::run () at C:\msys64\home\we\rust\src\librustc_data_structures/profiling.rs:569
#14 rustc_session::utils::<impl rustc_session::session::Session>::time () at C:\msys64\home\we\rust\src\librustc_session/utils.rs:9
#15 0x000000000083e549 in rustc_interface::passes::analysis () at src\librustc_interface/passes.rs:769
#16 0x00000000005ba426 in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute::{{closure}} () at C:\msys64\home\we\rust\src\librustc\ty\query/plumbing.rs:1008
#17 rustc::ty::query::__query_compute::analysis () at C:\msys64\home\we\rust\src\librustc\ty\query/plumbing.rs:960
#18 0x0000000000620aae in rustc::dep_graph::graph::DepGraph::with_task_impl () at C:\msys64\home\we\rust\src\librustc\dep_graph/graph.rs:341
#19 0x00000000005f8e09 in rustc::dep_graph::graph::DepGraph::with_eval_always_task () at C:\msys64\home\we\rust\src\librustc\dep_graph/graph.rs:388
#20 rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}} () at C:\msys64\home\we\rust\src\librustc\ty\query/plumbing.rs:662
#21 rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}} () at C:\msys64\home\we\rust\src\librustc\ty\query/plumbing.rs:324
#22 rustc::ty::context::tls::enter_context::{{closure}} () at C:\msys64\home\we\rust\src\librustc\ty/context.rs:1720
#23 rustc::ty::context::tls::set_tlv () at C:\msys64\home\we\rust\src\librustc\ty/context.rs:1704
#24 rustc::ty::context::tls::enter_context () at C:\msys64\home\we\rust\src\librustc\ty/context.rs:1720
#25 rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}} () at C:\msys64\home\we\rust\src\librustc\ty\query/plumbing.rs:324
#26 rustc::ty::context::tls::with_related_context::{{closure}} () at C:\msys64\home\we\rust\src\librustc\ty/context.rs:1808
#27 rustc::ty::context::tls::with_context::{{closure}} () at C:\msys64\home\we\rust\src\librustc\ty/context.rs:1792
#28 rustc::ty::context::tls::with_context_opt () at C:\msys64\home\we\rust\src\librustc\ty/context.rs:1781
#29 rustc::ty::context::tls::with_context () at C:\msys64\home\we\rust\src\librustc\ty/context.rs:1792
#30 rustc::ty::context::tls::with_related_context () at C:\msys64\home\we\rust\src\librustc\ty/context.rs:1805
#31 rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query () at C:\msys64\home\we\rust\src\librustc\ty\query/plumbing.rs:313
#32 rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}} () at C:\msys64\home\we\rust\src\librustc\ty\query/plumbing.rs:660
#33 rustc::ty::query::plumbing::with_diagnostics () at C:\msys64\home\we\rust\src\librustc\ty\query/plumbing.rs:248
#34 rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job () at C:\msys64\home\we\rust\src\librustc\ty\query/plumbing.rs:659
#35 rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::try_execute_query () at C:\msys64\home\we\rust\src\librustc\ty\query/plumbing.rs:493
#36 rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::{{closure}} () at C:\msys64\home\we\rust\src\librustc\ty\query/plumbing.rs:468
#37 <rustc::ty::query::caches::DefaultCache as rustc::ty::query::caches::QueryCache<K,V>>::lookup () at C:\msys64\home\we\rust\src\librustc\ty\query/caches.rs:86
#38 rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::try_get_cached () at C:\msys64\home\we\rust\src\librustc\ty\query/plumbing.rs:436
#39 rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query () at C:\msys64\home\we\rust\src\librustc\ty\query/plumbing.rs:462
#40 0x0000000000632c4c in rustc::ty::query::TyCtxtAt::analysis () at C:\msys64\home\we\rust\src\librustc\ty\query/plumbing.rs:1114
#41 rustc::ty::query::<impl rustc::ty::context::TyCtxt>::analysis () at C:\msys64\home\we\rust\src\librustc\ty\query/plumbing.rs:1077
#42 rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}} () at src\librustc_driver\lib.rs:392
#43 rustc_interface::passes::QueryContext::enter::{{closure}} () at C:\msys64\home\we\rust\src\librustc_interface/passes.rs:696
#44 rustc::ty::context::tls::enter_global::{{closure}} () at C:\msys64\home\we\rust\src\librustc\ty/context.rs:1743
#45 rustc::ty::context::tls::enter_context::{{closure}} () at C:\msys64\home\we\rust\src\librustc\ty/context.rs:1720
#46 rustc::ty::context::tls::set_tlv () at C:\msys64\home\we\rust\src\librustc\ty/context.rs:1704
#47 rustc::ty::context::tls::enter_context () at C:\msys64\home\we\rust\src\librustc\ty/context.rs:1720
#48 rustc::ty::context::tls::enter_global () at C:\msys64\home\we\rust\src\librustc\ty/context.rs:1743
#49 0x0000000000600d98 in rustc_interface::passes::QueryContext::enter () at C:\msys64\home\we\rust\src\librustc_interface/passes.rs:696
#50 rustc_driver::run_compiler::{{closure}}::{{closure}} () at src\librustc_driver\lib.rs:392
#51 rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter () at C:\msys64\home\we\rust\src\librustc_interface/queries.rs:339
#52 rustc_driver::run_compiler::{{closure}} () at src\librustc_driver\lib.rs:292
#53 rustc_interface::interface::run_compiler_in_existing_thread_pool () at C:\msys64\home\we\rust\src\librustc_interface/interface.rs:199
#54 0x00000000005b5e52 in rustc_interface::interface::run_compiler::{{closure}} () at C:\msys64\home\we\rust\src\librustc_interface/interface.rs:213
#55 rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}} () at C:\msys64\home\we\rust\src\librustc_interface/util.rs:155
#56 scoped_tls::ScopedKey<T>::set () at C:\Users\we\.cargo\registry\src\github.com-1ecc6299db9ec823\scoped-tls-1.0.0\src/lib.rs:137
#57 rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}} () at C:\msys64\home\we\rust\src\librustc_interface/util.rs:151
#58 scoped_tls::ScopedKey<T>::set () at C:\Users\we\.cargo\registry\src\github.com-1ecc6299db9ec823\scoped-tls-1.0.0\src/lib.rs:137
#59 rustc_ast::attr::with_globals::{{closure}} () at C:\msys64\home\we\rust\src\librustc_ast\attr/mod.rs:44
#60 scoped_tls::ScopedKey<T>::set () at C:\Users\we\.cargo\registry\src\github.com-1ecc6299db9ec823\scoped-tls-1.0.0\src/lib.rs:137
#61 rustc_ast::attr::with_globals () at C:\msys64\home\we\rust\src\librustc_ast\attr/mod.rs:44
#62 rustc_interface::util::spawn_thread_pool::{{closure}} () at C:\msys64\home\we\rust\src\librustc_interface/util.rs:150
#63 rustc_interface::util::scoped_thread::{{closure}} () at C:\msys64\home\we\rust\src\librustc_interface/util.rs:125
#64 std::sys_common::backtrace::__rust_begin_short_backtrace () at C:\msys64\home\we\rust\src\libstd\sys_common/backtrace.rs:130
#65 0x0000000000602e8a in std::thread::Builder::spawn_unchecked::{{closure}}::{{closure}} () at C:\msys64\home\we\rust\src\libstd\thread/mod.rs:475
#66 <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once () at C:\msys64\home\we\rust\src\libstd/panic.rs:318
#67 std::panicking::try::do_call () at C:\msys64\home\we\rust\src\libstd/panicking.rs:330
#68 std::panicking::try::do_try () at C:\msys64\home\we\rust\src\libstd/panicking.rs:298
#69 std::panicking::try () at C:\msys64\home\we\rust\src\libstd/panicking.rs:275
#70 std::panic::catch_unwind () at C:\msys64\home\we\rust\src\libstd/panic.rs:394
--Type <RET> for more, q to quit, c to continue without paging--
#71 std::thread::Builder::spawn_unchecked::{{closure}} () at C:\msys64\home\we\rust\src\libstd\thread/mod.rs:474
#72 core::ops::function::FnOnce::call_once{{vtable-shim}} () at C:\msys64\home\we\rust\src\libcore\ops/function.rs:232
#73 0x0000000067c93f16 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at C:\msys64\home\we\rust\src\liballoc/boxed.rs:1017
#74 0x0000000067cb80c9 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at C:\msys64\home\we\rust\src\liballoc/boxed.rs:1017
#75 std::sys_common::thread::start_thread () at src\libstd\sys_common/thread.rs:13
#76 0x0000000067ca50f9 in std::sys::windows::thread::Thread::new::thread_start () at src\libstd\sys\windows/thread.rs:51
#77 0x00007ffb1fb913d2 in KERNEL32!BaseThreadInitThunk () from /c/Windows/system32/KERNEL32.DLL
#78 0x00007ffb215054f4 in ntdll!RtlUserThreadStart () from /c/Windows/SYSTEM32/ntdll.dll
#79 0x0000000000000000 in ?? ()
