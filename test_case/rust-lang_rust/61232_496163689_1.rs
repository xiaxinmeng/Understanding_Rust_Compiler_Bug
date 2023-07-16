
% gdb ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc
GNU gdb (GDB) Fedora 8.2-3.fc29
Copyright (C) 2018 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-redhat-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc...done.
warning: Missing auto-load script at offset 0 in section .debug_gdb_scripts
of file /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/bin/rustc.
Use `info auto-load python-scripts [REGEXP]' to list them.
(gdb) r weird.rs
Starting program: /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/objdir-dbgopt/build/x86_64-unknown-linux-gnu/stage1/bin/rustc \
weird.rs
warning: Loadable section ".note.gnu.property" outside of ELF segments
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib64/libthread_db.so.1".
[New Thread 0x7ffff3b9b700 (LWP 23539)]
[New Thread 0x7fffe60c3700 (LWP 23540)]
[New Thread 0x7fffe5ec2700 (LWP 23541)]
[New Thread 0x7fffe5cc1700 (LWP 23542)]
[New Thread 0x7fffe5ac0700 (LWP 23543)]
[Thread 0x7fffe5cc1700 (LWP 23542) exited]
malloc(): mismatching next->prev_size (unsorted)

Thread 2 "rustc" received signal SIGABRT, Aborted.
[Switching to Thread 0x7ffff3b9b700 (LWP 23539)]
__GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:50
50        return ret;
Missing separate debuginfos, use: dnf debuginfo-install libgcc-8.2.1-5.fc29.x86_64 libstdc++-8.2.1-5.fc29.x86_64
(gdb) bt
#0  __GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:50
#1  0x00007ffff7b31895 in __GI_abort () at abort.c:79
#2  0x00007ffff7b8a927 in __libc_message (action=action@entry=do_abort, fmt=fmt@entry=0x7ffff7c98299 "%s\n")
    at ../sysdeps/posix/libc_fatal.c:181
#3  0x00007ffff7b9125c in malloc_printerr (str=str@entry=0x7ffff7c9a3d8 "malloc(): mismatching next->prev_size (unsorted)")
    at malloc.c:5382
#4  0x00007ffff7b9453c in _int_malloc (av=av@entry=0x7fffec000020, bytes=bytes@entry=26) at malloc.c:3737
#5  0x00007ffff7b95c8a in __GI___libc_malloc (bytes=26) at malloc.c:3057
#6  0x00007fffe8e27242 in alloc::alloc::alloc (layout=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/liballoc/alloc.rs:81
#7  <alloc::alloc::Global as core::alloc::Alloc>::alloc (self=<optimized out>, layout=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/liballoc/alloc.rs:169
#8  alloc::raw_vec::RawVec<T,A>::allocate_in (cap=26, zeroed=<optimized out>, a=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/liballoc/raw_vec.rs:95
#9  alloc::raw_vec::RawVec<T>::with_capacity (cap=26)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/liballoc/raw_vec.rs:139
#10 alloc::vec::Vec<T>::with_capacity (capacity=26) at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/liballoc/vec.rs:355
#11 alloc::slice::hack::to_vec (s=...) at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/liballoc/slice.rs:159
#12 alloc::slice::<impl [T]>::to_vec (self=...) at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/liballoc/slice.rs:380
#13 alloc::slice::<impl alloc::borrow::ToOwned for [T]>::to_owned (self=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/liballoc/slice.rs:648
#14 alloc::str::<impl alloc::borrow::ToOwned for str>::to_owned (self=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/liballoc/str.rs:206
#15 <alloc::string::String as core::convert::From<&str>>::from (s=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/liballoc/string.rs:2188
#16 <str as alloc::string::ToString>::to_string (self=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/liballoc/string.rs:2148
#17 rustc_codegen_llvm::consts::check_and_apply_linkage (cx=0x7ffff3b8b090, attrs=0x7ffff3b8a060, ty=<optimized out>, sym=...,
    span=...) at src/librustc_codegen_llvm/consts.rs:133
#18 0x00007fffe8c9fe58 in rustc_codegen_llvm::consts::<impl rustc_codegen_llvm::context::CodegenCx>::get_static (
    self=0x7ffff3b8b090, def_id=...) at src/librustc_codegen_llvm/consts.rs:233
#19 0x00007fffe8bee55a in <rustc_codegen_llvm::builder::Builder as rustc_codegen_ssa::traits::statics::StaticBuilderMethods>::get_static (self=<optimized out>, def_id=...) at src/librustc_codegen_llvm/builder.rs:1247
#20 rustc_codegen_ssa::mir::place::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_place (self=<optimized out>,
    bx=0x7ffff3b8a3c8, place=<optimized out>)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc_codegen_ssa/mir/place.rs:454
#21 0x00007fffe8bf06de in rustc_codegen_ssa::mir::rvalue::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_rvalue_operand
    (self=0x7ffff3b8aa30, bx=..., rvalue=<optimized out>)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc_codegen_ssa/mir/rvalue.rs:359
#22 0x00007fffe8bef465 in rustc_codegen_ssa::mir::rvalue::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_rvalue (
    self=0x7ffff3b8aa30, bx=..., dest=..., rvalue=0x0)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc_codegen_ssa/mir/rvalue.rs:141
#23 0x00007fffe8be460d in rustc_codegen_ssa::mir::statement::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_statement (
    self=0x7ffff3b8aa30, bx=..., statement=0x7fffec50b668)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc_codegen_ssa/mir/statement.rs:23
#24 rustc_codegen_ssa::mir::block::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_block (self=0x0, bb=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc_codegen_ssa/mir/block.rs:806
#25 rustc_codegen_ssa::mir::codegen_mir (cx=<optimized out>, llfn=<optimized out>, mir=<optimized out>, instance=..., sig=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc_codegen_ssa/mir/mod.rs:346
#26 0x00007fffe8d7e6d3 in rustc_codegen_ssa::base::codegen_instance (cx=0x7ffff3b8b090, instance=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc_codegen_ssa/base.rs:434
#27 0x00007fffe8e121d6 in rustc_codegen_ssa::mono_item::MonoItemExt::define (self=0x7ffff3b8b540, cx=0x7ffff3b8b090)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc_codegen_ssa/mono_item.rs:32
#28 0x00007fffe8c1d274 in rustc_codegen_llvm::base::compile_codegen_unit::module_codegen (tcx=..., cgu_name=...)
    at src/librustc_codegen_llvm/base.rs:146
#29 0x00007fffe8c9c4bc in rustc::dep_graph::graph::DepGraph::with_task_impl (self=<optimized out>, arg=..., no_tcx=false,
    task=0x7fffe8c1d090 <rustc_codegen_llvm::base::compile_codegen_unit::module_codegen>, key=..., cx=...,
    create_task=<optimized out>, finish_task_and_alloc_depnode=<optimized out>, hash_result=<optimized out>)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc/dep_graph/graph.rs:334
#30 rustc::dep_graph::graph::DepGraph::with_task (self=<optimized out>, key=..., cx=..., arg=...,
    task=0x7fffe8c1d090 <rustc_codegen_llvm::base::compile_codegen_unit::module_codegen>, hash_result=<optimized out>)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc/dep_graph/graph.rs:202
#31 0x00007fffe8c1ce9c in rustc_codegen_llvm::base::compile_codegen_unit (tcx=..., cgu_name=...)
    at src/librustc_codegen_llvm/base.rs:113
#32 0x00007fffe8d7b221 in <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::ExtraBackendMethods>::compile_codegen_unit (self=<optimized out>, tcx=..., cgu_name=...) at src/librustc_codegen_llvm/lib.rs:134
#33 rustc_codegen_ssa::base::codegen_crate (backend=..., tcx=..., metadata=..., need_metadata_module=<optimized out>, rx=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc_codegen_ssa/base.rs:669
#34 0x00007fffe8de9f8a in <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate (self=<optimized out>, tcx=..., metadata=..., need_metadata_module=<optimized out>, rx=...)
    at src/librustc_codegen_llvm/lib.rs:297
#35 0x00007ffff79a313d in rustc_interface::passes::start_codegen::{{closure}} () at src/librustc_interface/passes.rs:1086
#36 rustc::util::common::time_ext (sess=<optimized out>, what=..., do_it=<optimized out>, f=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc/util/common.rs:151
#37 rustc::util::common::time (sess=0x7fffec07f460, what=..., f=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc/util/common.rs:145
#38 0x00007ffff78e30ab in rustc_interface::passes::start_codegen (codegen_backend=..., tcx=..., rx=..., outputs=0x7ffff3b93c68)
    at src/librustc_interface/passes.rs:1085
#39 0x00007ffff78bf337 in rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen::{{closure}}::{{\
closure}} (tcx=...) at src/librustc_interface/queries.rs:255
#40 rustc_interface::passes::BoxedGlobalCtxt::enter::{{closure}}::{{closure}} (tcx=...) at src/librustc_interface/passes.rs:805
#41 rustc::ty::context::tls::enter_global::{{closure}} ()
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc/ty/context.rs:2007
#42 rustc::ty::context::tls::enter_context::{{closure}} ()
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc/ty/context.rs:1974
#43 rustc::ty::context::tls::set_tlv (value=140737154986968, f=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc/ty/context.rs:1907
#44 rustc::ty::context::tls::enter_context (context=0x7fffec2133d8, f=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc/ty/context.rs:1973
#45 rustc::ty::context::tls::enter_global (gcx=<optimized out>, f=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc/ty/context.rs:2006
#46 0x00007ffff78e38e1 in rustc_interface::passes::BoxedGlobalCtxt::enter::{{closure}} (gcx=0x0)
    at src/librustc_interface/passes.rs:805
#47 rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}} (args=...)
    at <::rustc_data_structures::box_region::declare_box_region_type macros>:17
#48 0x00007ffff79b1af5 in rustc_interface::passes::create_global_ctxt::{{closure}} () at src/librustc_interface/passes.rs:871#48 0x00007ffff79b1af5 in rustc_interface::passes::create_global_ctxt::{{closure}} () at src/librustc_interface/passes.rs:871
#49 0x00007ffff78e22a6 in alloc::boxed::<impl core::ops::generator::Generator for core::pin::Pin<alloc::boxed::Box<G>>>::resume
    (self=...) at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/liballoc/boxed.rs:974
#50 rustc_data_structures::box_region::PinnedGenerator<I,A,R>::access (self=0x7ffff3b93d28, closure=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc_data_structures/box_region.rs:52
#51 rustc_interface::passes::BoxedGlobalCtxt::access (self=0x7ffff3b93d28, f=...)
    at <::rustc_data_structures::box_region::declare_box_region_type macros>:19
#52 rustc_interface::passes::BoxedGlobalCtxt::enter (self=0x7ffff3b93d28, f=...) at src/librustc_interface/passes.rs:805
#53 0x00007ffff78c57df in rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen::{{closure}}
    () at src/librustc_interface/queries.rs:249
#54 rustc_interface::queries::Query<T>::compute (self=0x7ffff3b93d38, f=...) at src/librustc_interface/queries.rs:40
#55 0x00007ffff7a41324 in rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen (
    self=<optimized out>) at src/librustc_interface/queries.rs:246
#56 0x00007ffff7eacf2a in rustc_driver::run_compiler::{{closure}} (compiler=0x2) at src/librustc_driver/lib.rs:352
#57 rustc_interface::interface::run_compiler_in_existing_thread_pool (config=..., f=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc_interface/interface.rs:123
#58 0x00007ffff7f509d6 in rustc_interface::interface::run_compiler::{{closure}} ()
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc_interface/interface.rs:142
#59 rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}} ()
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc_interface/util.rs:188
#60 rustc::ty::context::tls::with_thread_locals::{{closure}}::{{closure}} (current=0x7ffff3b9b570)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc/ty/context.rs:1962
#61 std::thread::local::LocalKey<T>::try_with (f=..., self=<optimized out>)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/libstd/thread/local.rs:299
#62 std::thread::local::LocalKey<T>::with (self=<optimized out>, f=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/libstd/thread/local.rs:245
#63 rustc::ty::context::tls::with_thread_locals::{{closure}} (span_dbg=0x7ffff3b9a2f8)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc/ty/context.rs:1954
#64 std::thread::local::LocalKey<T>::try_with (self=<optimized out>, f=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/libstd/thread/local.rs:299
#65 std::thread::local::LocalKey<T>::with (self=<optimized out>, f=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/libstd/thread/local.rs:245
#66 0x00007ffff7ef5495 in rustc::ty::context::tls::with_thread_locals (f=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc/ty/context.rs:1946
#67 rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}} ()
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc_interface/util.rs:188
#68 scoped_tls::ScopedKey<T>::set (self=0x7ffff6716dd0 <rustc::ty::context::tls::GCX_PTR>, t=<optimized out>, f=...)
    at /home/pnkfelix/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
#69 rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}} ()
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc_interface/util.rs:184
#70 scoped_tls::ScopedKey<T>::set (self=<optimized out>, t=<optimized out>, f=...)
    at /home/pnkfelix/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
#71 syntax::with_globals::{{closure}} () at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/libsyntax/lib.rs:104
#72 scoped_tls::ScopedKey<T>::set (self=<optimized out>, t=<optimized out>, f=...)
    at /home/pnkfelix/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
#73 0x00007ffff7f06784 in syntax::with_globals (edition=<optimized out>, f=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/libsyntax/lib.rs:103
#74 0x00007ffff7e64778 in rustc_interface::util::spawn_thread_pool::{{closure}} ()
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc_interface/util.rs:183
#75 rustc_interface::util::scoped_thread::{{closure}} ()
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/librustc_interface/util.rs:160
#76 std::sys_common::backtrace::__rust_begin_short_backtrace (f=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/libstd/sys_common/backtrace.rs:77
#77 0x00007ffff7d91f9a in __rust_maybe_catch_panic (f=0x0, data=0x2 <error: Cannot access memory at address 0x2>,
    data_ptr=0x7ffff3b999e0, vtable_ptr=0x7ffff3b999e8) at src/libpanic_unwind/lib.rs:85
#78 0x00007ffff7e6598a in std::panicking::try (f=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/libstd/panicking.rs:272
#79 0x00007ffff7ea406e in std::panic::catch_unwind (f=...)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/libstd/panic.rs:388
#80 std::thread::Builder::spawn_unchecked::{{closure}} ()
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/libstd/thread/mod.rs:469
#81 core::ops::function::FnOnce::call_once{{vtable-shim}} ()
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/libcore/ops/function.rs:231
#82 0x00007ffff7d8bc7f in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once (self=..., args=<optimized out>)
    at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/liballoc/boxed.rs:744
#83 0x00007ffff7d8d7c0 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once (self=<optimized out>,
    args=<optimized out>) at /home/pnkfelix/Dev/Mozilla/issue59548/rust-59548/src/liballoc/boxed.rs:744
#84 std::sys_common::thread::start_thread (main=0x55555555b440 "\340\305UUUU") at src/libstd/sys_common/thread.rs:13
#85 0x00007ffff7d86456 in std::sys::unix::thread::Thread::new::thread_start (main=0x2) at src/libstd/sys/unix/thread.rs:79
#86 0x00007ffff508b58e in start_thread (arg=<optimized out>) at pthread_create.c:486
#87 0x00007ffff7c0c6a3 in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
