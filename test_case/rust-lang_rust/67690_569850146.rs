
#0  0x00007ff429b08329 in rustc_errors::styled_buffer::StyledBuffer::putc ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#1  0x00007ff429b21fb7 in <rustc_errors::emitter::EmitterWriter as rustc_errors::emitter::Emitter>::emit_diagnostic ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#2  0x00007ff429b372a9 in rustc_errors::HandlerInner::emit_diagnostic ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#3  0x00007ff429b0662b in rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
    ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#4  0x00007ff427eb00ec in <rustc_lint::builtin::TypeAliasBounds as rustc::lint::LateLintPass>::check_item ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#5  0x00007ff427e40ca6 in <rustc_lint::BuiltinCombinedModuleLateLintPass as rustc::lint::LateLintPass>::check_item ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#6  0x00007ff427e6652e in rustc::hir::intravisit::Visitor::visit_nested_item ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#7  0x00007ff427e6850d in rustc::lint::context::late_lint_mod ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#8  0x00007ff427df5c8a in rustc::ty::query::__query_compute::lint_mod ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#9  0x00007ff427e1263b in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::lint_mod>::compute ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#10 0x00007ff427dfa04d in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#11 0x00007ff427e25174 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin---Type <return> to continue, or q <return> to quit---
/../lib/librustc_driver-63848c3330cc991c.so
#12 0x00007ff427e12c25 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#13 0x00007ff427d57876 in rustc::lint::context::check_crate::{{closure}}::{{closure}} ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#14 0x00007ff427d582f6 in rustc::util::common::time ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#15 0x00007ff427d58956 in rustc::util::common::time ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#16 0x00007ff4277aa44a in __rust_maybe_catch_panic ()
    at src/libpanic_unwind/lib.rs:78
#17 0x00007ff427de3f73 in <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#18 0x00007ff4277aa44a in __rust_maybe_catch_panic ()
    at src/libpanic_unwind/lib.rs:78
#19 0x00007ff427d5e4e2 in rustc_interface::passes::analysis::{{closure}} ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#20 0x00007ff427d5df45 in rustc_interface::passes::analysis ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#21 0x00007ff427ce8481 in rustc::ty::query::__query_compute::analysis ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#22 0x00007ff427ceac7d in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#23 0x00007ff427c633c7 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#24 0x00007ff427ceda3a in rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}} ()
---Type <return> to continue, or q <return> to quit---
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#25 0x00007ff427daf92a in rustc_interface::passes::create_global_ctxt::{{closure}}
    ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#26 0x00007ff427ced78e in rustc_interface::passes::BoxedGlobalCtxt::enter ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#27 0x00007ff427cbd7c7 in rustc_interface::interface::run_compiler_in_existing_thread_pool ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#28 0x00007ff427c9e112 in std::thread::local::LocalKey<T>::with ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#29 0x00007ff427c9bbce in scoped_tls::ScopedKey<T>::set ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#30 0x00007ff427cca264 in syntax::with_globals ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#31 0x00007ff427ce28d0 in std::sys_common::backtrace::__rust_begin_short_backtrace
    ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#32 0x00007ff4277aa44a in __rust_maybe_catch_panic ()
    at src/libpanic_unwind/lib.rs:78
#33 0x00007ff427ce3809 in core::ops::function::FnOnce::call_once{{vtable-shim}} ()
   from /home/joshua/.local/lib/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-63848c3330cc991c.so
#34 0x00007ff42777ae2f in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once ()
    at /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/liballoc/boxed.rs:942
#35 0x00007ff4277a8e70 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once ()
    at /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/liballoc/boxed.rs:942
#36 std::sys_common::thread::start_thread () at src/libstd/sys_common/thread.rs:13
#37 std::sys::unix::thread::Thread::new::thread_start ()
    at src/libstd/sys/unix/thread.rs:79
#38 0x00007ff4275156db in start_thread (arg=0x7ff425bff700) at pthread_create.c:463
---Type <return> to continue, or q <return> to quit---
#39 0x00007ff426e3288f in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
