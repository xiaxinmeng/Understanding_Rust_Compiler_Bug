rust
#0  free (ptr=0x48c2df416aec23d6) at ../jemalloc/src/jemalloc.c:2393
#1  0x00007ffff3513bcc in <smallvec::SmallVec<A> as core::ops::drop::Drop>::drop () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#2  0x00007ffff35c1011 in <rustc_resolve::late::LateResolutionVisitor as rustc_ast::visit::Visitor>::visit_local () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#3  0x00007ffff35c0822 in <rustc_resolve::late::LateResolutionVisitor as rustc_ast::visit::Visitor>::visit_block () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#4  0x00007ffff35c27d0 in <rustc_resolve::late::LateResolutionVisitor as rustc_ast::visit::Visitor>::visit_fn () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#5  0x00007ffff354bf9c in rustc_ast::visit::walk_assoc_item () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#6  0x00007ffff35ce54e in rustc_resolve::late::LateResolutionVisitor::with_generic_param_rib () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#7  0x00007ffff35c4f39 in rustc_resolve::late::LateResolutionVisitor::resolve_item () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#8  0x00007ffff355896e in rustc_ast::visit::walk_item () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#9  0x00007ffff35c4473 in rustc_resolve::late::LateResolutionVisitor::resolve_item () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#10 0x00007ffff355896e in rustc_ast::visit::walk_item () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#11 0x00007ffff35c4473 in rustc_resolve::late::LateResolutionVisitor::resolve_item () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#12 0x00007ffff3549d42 in rustc_ast::visit::walk_crate () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#13 0x00007ffff358aac7 in rustc_resolve::Resolver::resolve_crate () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#14 0x00007ffff08f5c97 in rustc_interface::passes::configure_and_expand_inner () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#15 0x00007ffff08d26c9 in rustc_interface::passes::configure_and_expand::{{closure}} () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#16 0x00007ffff08acecf in rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#17 0x00007ffff08f4965 in rustc_interface::passes::configure_and_expand () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#18 0x00007ffff0915f73 in rustc_interface::queries::Queries::expansion () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#19 0x00007ffff05bf887 in rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#20 0x00007ffff0553f27 in rustc_span::with_source_map () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#21 0x00007ffff05c1513 in rustc_interface::interface::create_compiler_and_run () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#22 0x00007ffff059f9fa in scoped_tls::ScopedKey<T>::set () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#23 0x00007ffff05b4957 in std::sys_common::backtrace::__rust_begin_short_backtrace () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#24 0x00007ffff053ddae in core::ops::function::FnOnce::call_once{{vtable-shim}} () from /home/overminddl1/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-ff4ec557f69b94a7.so
#25 0x00007fffef94bf5a in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/alloc/src/boxed.rs:1042
#26 <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/alloc/src/boxed.rs:1042
#27 std::sys::unix::thread::Thread::new::thread_start () at library/std/src/sys/unix/thread.rs:87
#28 0x00007fffef83e669 in start_thread (arg=<optimized out>) at pthread_create.c:479
#29 0x00007fffef7642b3 in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
