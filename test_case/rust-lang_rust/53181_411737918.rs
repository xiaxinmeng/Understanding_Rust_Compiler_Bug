
(gdb) r hello.rs
Starting program: /home/glaubitz/stage2/bin/rustc hello.rs
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/sparc64-linux-gnu/libthread_db.so.1".

Program received signal SIGBUS, Bus error.
0xffff800100f37738 in rustc_resolve::Resolver::resolve_path () from /home/glaubitz/stage2/bin/../lib/../lib/librustc_resolve-ea53ebec52bf11a1.so
(gdb) bt
#0  0xffff800100f37738 in rustc_resolve::Resolver::resolve_path () from /home/glaubitz/stage2/bin/../lib/../lib/librustc_resolve-ea53ebec52bf11a1.so
#1  0xffff800100f2d2d4 in rustc_resolve::Resolver::smart_resolve_path_fragment () from /home/glaubitz/stage2/bin/../lib/../lib/librustc_resolve-ea53ebec52bf11a1.so
#2  0xffff800100f2ce70 in rustc_resolve::Resolver::smart_resolve_path_with_crate_lint () from /home/glaubitz/stage2/bin/../lib/../lib/librustc_resolve-ea53ebec52bf11a1.so
#3  0xffff800100f3b13c in rustc_resolve::Resolver::resolve_expr () from /home/glaubitz/stage2/bin/../lib/../lib/librustc_resolve-ea53ebec52bf11a1.so
#4  0xffff800100f3ac78 in rustc_resolve::Resolver::resolve_expr () from /home/glaubitz/stage2/bin/../lib/../lib/librustc_resolve-ea53ebec52bf11a1.so
#5  0xffff800100f176ec in <rustc_resolve::Resolver<'a, 'cl> as syntax::visit::Visitor<'tcx>>::visit_block () from /home/glaubitz/stage2/bin/../lib/../lib/librustc_resolve-ea53ebec52bf11a1.so
#6  0xffff800100f39cf0 in rustc_resolve::Resolver::resolve_labeled_block () from /home/glaubitz/stage2/bin/../lib/../lib/librustc_resolve-ea53ebec52bf11a1.so
#7  0xffff800100f3a564 in rustc_resolve::Resolver::resolve_expr () from /home/glaubitz/stage2/bin/../lib/../lib/librustc_resolve-ea53ebec52bf11a1.so
#8  0xffff800100f176ec in <rustc_resolve::Resolver<'a, 'cl> as syntax::visit::Visitor<'tcx>>::visit_block () from /home/glaubitz/stage2/bin/../lib/../lib/librustc_resolve-ea53ebec52bf11a1.so
#9  0xffff800100f19454 in <rustc_resolve::Resolver<'a, 'cl> as syntax::visit::Visitor<'tcx>>::visit_fn () from /home/glaubitz/stage2/bin/../lib/../lib/librustc_resolve-ea53ebec52bf11a1.so
#10 0xffff800100f5c900 in syntax::visit::walk_item () from /home/glaubitz/stage2/bin/../lib/../lib/librustc_resolve-ea53ebec52bf11a1.so
#11 0xffff800100f25084 in rustc_resolve::Resolver::resolve_item () from /home/glaubitz/stage2/bin/../lib/../lib/librustc_resolve-ea53ebec52bf11a1.so
#12 0xffff800100f20234 in rustc_resolve::Resolver::resolve_crate () from /home/glaubitz/stage2/bin/../lib/../lib/librustc_resolve-ea53ebec52bf11a1.so
#13 0xffff800100214138 in rustc::util::common::time () from /home/glaubitz/stage2/bin/../lib/librustc_driver-8c16f292143771bc.so
#14 0xffff800100200a68 in rustc_driver::driver::phase_2_configure_and_expand () from /home/glaubitz/stage2/bin/../lib/librustc_driver-8c16f292143771bc.so
#15 0xffff8001001fa128 in rustc_driver::driver::compile_input () from /home/glaubitz/stage2/bin/../lib/librustc_driver-8c16f292143771bc.so
#16 0xffff8001002cf8ec in rustc_driver::run_compiler_with_pool () from /home/glaubitz/stage2/bin/../lib/librustc_driver-8c16f292143771bc.so
#17 0xffff8001001e3c48 in <scoped_tls::ScopedKey<T>>::set () from /home/glaubitz/stage2/bin/../lib/librustc_driver-8c16f292143771bc.so
#18 0xffff800100284034 in syntax::with_globals () from /home/glaubitz/stage2/bin/../lib/librustc_driver-8c16f292143771bc.so
#19 0xffff8001001e1ca0 in <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once () from /home/glaubitz/stage2/bin/../lib/librustc_driver-8c16f292143771bc.so
#20 0xffff8001002b580c in std::panicking::try::do_call () from /home/glaubitz/stage2/bin/../lib/librustc_driver-8c16f292143771bc.so
#21 0xffff80010057c55c in __rust_maybe_catch_panic () from /home/glaubitz/stage2/bin/../lib/libstd-fb04bd5ded883618.so
#22 0xffff8001002cbe90 in rustc_driver::run () from /home/glaubitz/stage2/bin/../lib/librustc_driver-8c16f292143771bc.so
#23 0xffff8001002dc4cc in rustc_driver::main () from /home/glaubitz/stage2/bin/../lib/librustc_driver-8c16f292143771bc.so
#24 0x0000010000000d98 in rustc_binary::main ()
#25 0x0000010000000d60 in std::rt::lang_start::{{closure}} ()
#26 0xffff800100534810 in std::sys_common::backtrace::__rust_begin_short_backtrace () from /home/glaubitz/stage2/bin/../lib/libstd-fb04bd5ded883618.so
#27 0xffff800100531b4c in std::panicking::try::do_call () from /home/glaubitz/stage2/bin/../lib/libstd-fb04bd5ded883618.so
#28 0xffff80010057c55c in __rust_maybe_catch_panic () from /home/glaubitz/stage2/bin/../lib/libstd-fb04bd5ded883618.so
#29 0xffff800100524038 in std::rt::lang_start_internal () from /home/glaubitz/stage2/bin/../lib/libstd-fb04bd5ded883618.so
#30 0x0000010000000de0 in main ()
(gdb)
