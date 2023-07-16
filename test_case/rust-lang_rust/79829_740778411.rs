
error: expected one of `(`, `)`, `,`, `::`, or `=`, found `==`
 --> cfg-overflow.rs:3:31
  |
3 |         #[cfg(not(target_arch == "x86"))] {}
  |                               ^^ expected one of `(`, `)`, `,`, `::`, or `=`


Thread 2 "rustc" received signal SIGABRT, Aborted.
[Switching to Thread 0x7fffed9ff700 (LWP 2132847)]
__GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:50
50	../sysdeps/unix/sysv/linux/raise.c: No such file or directory.
(gdb) where
#0  __GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:50
#1  0x00007ffff2fe6859 in __GI_abort () at abort.c:79
#2  0x00007ffff32c8e77 in std::sys::unix::abort_internal ()
    at library/std/src/sys/unix/mod.rs:237
#3  0x00007ffff32b2026 in std::process::abort () at library/std/src/process.rs:1784
#4  0x00007ffff5c37914 in rustc_ast::mut_visit::visit_clobber::{{closure}} ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#5  0x00007ffff5c3eab8 in rustc_ast::mut_visit::noop_visit_expr ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#6  0x00007ffff5bf8792 in <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#7  0x00007ffff5c41a92 in rustc_ast::mut_visit::noop_visit_local ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#8  0x00007ffff5c4a8a4 in rustc_ast::mut_visit::noop_flat_map_stmt_kind ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#9  0x00007ffff5bd2b36 in <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::flat_map_stmt ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#10 0x00007ffff5b94d02 in <alloc::vec::Vec<T> as rustc_data_structures::map_in_place::MapInPlace<T>>::flat_map_in_place ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#11 0x00007ffff5c474bf in rustc_ast::mut_visit::noop_visit_item_kind ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#12 0x00007ffff5c43227 in rustc_ast::mut_visit::noop_flat_map_item ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#13 0x00007ffff5bd3c7b in <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::flat_map_item ()
   from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin--Type <RET> for more, q to quit, c to continue without paging--c
/../lib/librustc_driver-74194b4af230f3a4.so
#14 0x00007ffff5b9fe4a in <alloc::vec::Vec<T> as rustc_data_structures::map_in_place::MapInPlace<T>>::flat_map_in_place () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#15 0x00007ffff5c474f4 in rustc_ast::mut_visit::noop_visit_item_kind () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#16 0x00007ffff5c43227 in rustc_ast::mut_visit::noop_flat_map_item () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#17 0x00007ffff5bd3c7b in <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::flat_map_item () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#18 0x00007ffff5c556f1 in <smallvec::SmallVec<A> as rustc_data_structures::map_in_place::MapInPlace<T>>::flat_map_in_place () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#19 0x00007ffff5bcd6c4 in rustc_expand::expand::MacroExpander::collect_invocations () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#20 0x00007ffff5bc8e4f in rustc_expand::expand::MacroExpander::fully_expand_fragment () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#21 0x00007ffff5bc862e in rustc_expand::expand::MacroExpander::expand_crate () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#22 0x00007ffff3d8de58 in rustc_session::utils::<impl rustc_session::session::Session>::time () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#23 0x00007ffff3dcc088 in rustc_interface::passes::configure_and_expand_inner () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#24 0x00007ffff3d84657 in rustc_interface::passes::configure_and_expand::{{closure}} () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#25 0x00007ffff3d753df in rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#26 0x00007ffff3dcb2c8 in rustc_interface::passes::configure_and_expand () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#27 0x00007ffff3e26a9f in rustc_interface::queries::Queries::expansion () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#28 0x00007ffff3b17e61 in rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#29 0x00007ffff3bab5f7 in rustc_span::with_source_map () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#30 0x00007ffff3b1968b in rustc_interface::interface::create_compiler_and_run () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#31 0x00007ffff3bc5b60 in scoped_tls::ScopedKey<T>::set () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#32 0x00007ffff3bcc346 in std::sys_common::backtrace::__rust_begin_short_backtrace () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#33 0x00007ffff3b2107a in core::ops::function::FnOnce::call_once{{vtable-shim}} () from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74194b4af230f3a4.so
#34 0x00007ffff32c865a in <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once () at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/alloc/src/boxed.rs:1328
#35 <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once () at /rustc/0f6f2d681b39c5f95459cd09cb936b6ceb27cd82/library/alloc/src/boxed.rs:1328
#36 std::sys::unix::thread::Thread::new::thread_start () at library/std/src/sys/unix/thread.rs:71
#37 0x00007ffff31cd609 in start_thread (arg=<optimized out>) at pthread_create.c:477
#38 0x00007ffff30e3293 in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
