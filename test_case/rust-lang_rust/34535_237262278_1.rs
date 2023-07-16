
$ pstree -p 16871
rustc(16871)───rustc(16872)───{rustc}(16873)
$ sudo gdb -p 16871 --batch -ex bt
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
0x00007f808256be8c in __libc_waitpid (pid=16872, stat_loc=0x7ffc614a9d7c, options=0) at ../sysdeps/unix/sysv/linux/waitpid.c:31
31      ../sysdeps/unix/sysv/linux/waitpid.c: No such file or directory.
#0  0x00007f808256be8c in __libc_waitpid (pid=16872, stat_loc=0x7ffc614a9d7c, options=0) at ../sysdeps/unix/sysv/linux/waitpid.c:31
#1  0x0000555a9c9fff7b in std::process::Child::wait::h3efe8d2fdb58555e ()
#2  0x0000555a9c5fcf36 in rustup_init::proxy_mode::direct_proxy::h37bbd13facb3f408 ()
#3  0x0000555a9c5b9747 in rustup_init::run_multirust::h70cbf6663515e18b ()
#4  0x0000555a9c5b6ee2 in rustup_init::main::h81d74e5ca7cf36c2 ()
#5  0x0000555a9ca0b485 in std::sys_common::unwind::try::try_fn::h04c0c89e4add6cc4 ()
#6  0x0000555a9ca02c6c in __rust_try ()
#7  0x0000555a9ca0aecc in std::rt::lang_start::h61f4934e780b4dfc ()
#8  0x00007f8081fa2f45 in __libc_start_main (main=0x555a9c5baf10 <main>, argc=45, argv=0x7ffc614afa08, init=<optimised out>, fini=<optimised out>, rtld_fini=<optimised out>, stack_end=0x7ffc614af9f8) at libc-start.c:287
#9  0x0000555a9c5b6dd5 in _start ()
$ sudo gdb -p 16872 --batch -ex bt
[New LWP 16873]
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
0x00007fd6576df65b in pthread_join (threadid=140558540470016, thread_return=0x0) at pthread_join.c:92
92      pthread_join.c: No such file or directory.
#0  0x00007fd6576df65b in pthread_join (threadid=140558540470016, thread_return=0x0) at pthread_join.c:92
#1  0x00007fd66026f5cb in std::sys::thread::Thread::join::h22a49811a1b525c8 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libstd-c8005792.so
#2  0x00007fd66080a09a in rustc_driver::run::hebe6a2a70ce6ae1e () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-c8005792.so
#3  0x00007fd6608186ed in rustc_driver::main::h2290eb64ff3fc98a () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-c8005792.so
#4  0x00007fd660271609 in std::panicking::try::call::h5df3ac2979db3c90 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libstd-c8005792.so
#5  0x00007fd660281187 in __rust_maybe_catch_panic () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libstd-c8005792.so
#6  0x00007fd66027075d in std::rt::lang_start::hfe9ab243c60ffb9b () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libstd-c8005792.so
#7  0x00007fd65fdd1f45 in __libc_start_main (main=0x559ff04908d0 <main>, argc=47, argv=0x7ffeb624b7f8, init=<optimised out>, fini=<optimised out>, rtld_fini=<optimised out>, stack_end=0x7ffeb624b7e8) at libc-start.c:287
#8  0x0000559ff0490779 in _start ()
$ sudo gdb -p 16873 --batch -ex bt                                                                                                                
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
0x00007fd65fe9b72d in write () at ../sysdeps/unix/syscall-template.S:81
81      ../sysdeps/unix/syscall-template.S: No such file or directory.
#0  0x00007fd65fe9b72d in write () at ../sysdeps/unix/syscall-template.S:81
#1  0x00007fd660247401 in _$LT$std..io..stdio..Stderr$u20$as$u20$std..io..Write$GT$::write::hd0e839e43939c007 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-u
nknown-linux-gnu/lib/libstd-c8005792.so
#2  0x00007fd65628a73d in _$LT$term..terminfo..TerminfoTerminal$LT$T$GT$$u20$as$u20$std..io..Write$GT$::write::hdd258bedc9595a9c () from /home/aidanhs/.multirust/toolchai
ns/nightly-x86_64-unknown-linux-gnu/lib/libterm-c8005792.so
#3  0x00007fd657fec1e3 in _$LT$std..io..Write..write_fmt..Adaptor$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..fmt..Write$GT$::write_str::h312bc07629475bdf () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_errors-c8005792.so
#4  0x00007fd6602e8e08 in core::fmt::write::h955c581c22098b7a () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libstd-c8005792.so
#5  0x00007fd657fe9d63 in std::io::Write::write_fmt::hfee6c576dc30ba81 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_errors-c8
005792.so
#6  0x00007fd657ff7924 in rustc_errors::emitter::emit_to_destination::h38d36f33b3cf5586 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/l
ibrustc_errors-c8005792.so
#7  0x00007fd657ff6445 in rustc_errors::emitter::EmitterWriter::emit_message_old_school::hf000ccfd00396981 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unkn
own-linux-gnu/lib/librustc_errors-c8005792.so
#8  0x00007fd657fee7ea in _$LT$rustc_errors..emitter..EmitterWriter$u20$as$u20$rustc_errors..emitter..Emitter$GT$::emit::hb6bf465c9fd02120 () from /home/aidanhs/.multirus
t/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_errors-c8005792.so
#9  0x00007fd657ffc330 in rustc_errors::DiagnosticBuilder::emit::h92f9c1795d5aff70 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librus
tc_errors-c8005792.so
#10 0x00007fd65c9871ed in rustc::infer::InferCtxt::report_mismatched_types::h0b609f1ce41bfd4e () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu
/lib/librustc-c8005792.so
#11 0x00007fd65e617b9c in rustc_typeck::check::demand::_$LT$impl$u20$rustc_typeck..check..FnCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::demand_eqtype_with_ori
gin::h9ab18fcb8f617943 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_typeck-c8005792.so
#12 0x00007fd65e5fb487 in rustc_typeck::check::_match::_$LT$impl$u20$rustc_typeck..check..FnCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::check_pat_tuple_struct
::h314035dae70bea90 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_typeck-c8005792.so
#13 0x00007fd65e5f887f in rustc_typeck::check::_match::_$LT$impl$u20$rustc_typeck..check..FnCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::check_pat::h7b18286e9f
b261f3 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_typeck-c8005792.so
#14 0x00007fd65e5fb654 in rustc_typeck::check::_match::_$LT$impl$u20$rustc_typeck..check..FnCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::check_pat_tuple_struct
::h314035dae70bea90 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_typeck-c8005792.so
#15 0x00007fd65e5f887f in rustc_typeck::check::_match::_$LT$impl$u20$rustc_typeck..check..FnCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::check_pat::h7b18286e9f
b261f3 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_typeck-c8005792.so
#16 0x00007fd65e5f9793 in rustc_typeck::check::_match::_$LT$impl$u20$rustc_typeck..check..FnCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::check_pat::h7b18286e9f
b261f3 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_typeck-c8005792.so
#17 0x00007fd65e5fb654 in rustc_typeck::check::_match::_$LT$impl$u20$rustc_typeck..check..FnCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::check_pat_tuple_struct
::h314035dae70bea90 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_typeck-c8005792.so
#18 0x00007fd65e5f887f in rustc_typeck::check::_match::_$LT$impl$u20$rustc_typeck..check..FnCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::check_pat::h7b18286e9f
b261f3 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_typeck-c8005792.so
#19 0x00007fd65e5fa763 in rustc_typeck::check::_match::_$LT$impl$u20$rustc_typeck..check..FnCtxt$LT$$u27$a$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$GT$::check_match::h541c2744
e41fdabc () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_typeck-c8005792.so
#20 0x00007fd65e6825ec in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::hf9af8b059be90a47 () from /home/aidanhs/.multirust/toolchains/nightly-
x86_64-unknown-linux-gnu/lib/librustc_typeck-c8005792.so
#21 0x00007fd65e68777e in rustc_typeck::check::FnCtxt::check_decl_initializer::hba36f65b1576a6af () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-
gnu/lib/librustc_typeck-c8005792.so
#22 0x00007fd65e68782e in rustc_typeck::check::FnCtxt::check_decl_local::h2a0e3673e09cd8bc () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/li
b/librustc_typeck-c8005792.so
#23 0x00007fd65e687a89 in rustc_typeck::check::FnCtxt::check_stmt::hd7eaaa481f8ccc18 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libr
ustc_typeck-c8005792.so
#24 0x00007fd65e687e84 in rustc_typeck::check::FnCtxt::check_block_with_expected::h55da64af5446ca47 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-lin
ux-gnu/lib/librustc_typeck-c8005792.so
#25 0x00007fd65e678c3b in rustc_typeck::check::FnCtxt::check_then_else::h1653533aed17af75 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib
/librustc_typeck-c8005792.so
#26 0x00007fd65e682644 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::hf9af8b059be90a47 () from /home/aidanhs/.multirust/toolchains/nightly-
x86_64-unknown-linux-gnu/lib/librustc_typeck-c8005792.so
#27 0x00007fd65e678cb6 in rustc_typeck::check::FnCtxt::check_then_else::h1653533aed17af75 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib
/librustc_typeck-c8005792.so
#28 0x00007fd65e682644 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::hf9af8b059be90a47 () from /home/aidanhs/.multirust/toolchains/nightly-
x86_64-unknown-linux-gnu/lib/librustc_typeck-c8005792.so
#29 0x00007fd65e688072 in rustc_typeck::check::FnCtxt::check_block_with_expected::h55da64af5446ca47 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-lin
ux-gnu/lib/librustc_typeck-c8005792.so
#30 0x00007fd65e660dc7 in rustc_typeck::check::check_fn::h6c476956211c7381 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_typec
k-c8005792.so
#31 0x00007fd65e65f567 in rustc_typeck::check::check_bare_fn::h952d7c729d98a4e9 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_
typeck-c8005792.so
#32 0x00007fd65e66577f in rustc_typeck::check::check_item_body::h26c5d995a8cf2591 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librust
c_typeck-c8005792.so
#33 0x00007fd65e65e07e in rustc_typeck::check::check_item_bodies::h879e0600ff9f7c52 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libru
stc_typeck-c8005792.so
#34 0x00007fd65e6d5486 in rustc_typeck::check_crate::hcd055bbbc96bb85b () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_typeck-c8
005792.so
#35 0x00007fd6608257ec in rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::h7f2cec505064b4bf () from /home/aidanhs/.multirust/toolchains/n
ightly-x86_64-unknown-linux-gnu/lib/librustc_driver-c8005792.so
#36 0x00007fd660783e0c in rustc::ty::context::TyCtxt::create_and_enter::hab8fd8369376d0f6 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib
/librustc_driver-c8005792.so
#37 0x00007fd6607e682d in rustc_driver::driver::compile_input::hb4cc34cf85dc1edf () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc
_driver-c8005792.so
#38 0x00007fd66080c178 in rustc_driver::run_compiler::h50f95674bd902ab5 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-c
8005792.so
#39 0x00007fd66075785e in std::panicking::try::call::h4577500a5284c6ff () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-c8
005792.so
#40 0x00007fd660281187 in __rust_maybe_catch_panic () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libstd-c8005792.so
#41 0x00007fd660772570 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h24f3eb0b42327962 () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unkno
wn-linux-gnu/lib/librustc_driver-c8005792.so
#42 0x00007fd66026f478 in std::sys::thread::Thread::new::thread_start::h8f3bd45211e9f5ea () from /home/aidanhs/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/
libstd-c8005792.so
#43 0x00007fd6576de184 in start_thread (arg=0x7fd655dff700) at pthread_create.c:312
#44 0x00007fd65feaa37d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111
