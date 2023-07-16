
#0  __GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:50
#1  0x00007ffff71d9875 in __GI_abort () at abort.c:79
#2  0x00007ffff71d974f in __assert_fail_base (fmt=0x7ffff733b2f8 "%s%s%s:%u: %s%sAssertion `%s' failed.\n%n",
    assertion=0x7fffec290930 "isa<X>(Val) && \"cast<Ty>() argument of incompatible type!\"", file=0x7fffec28dfd0 "/home/void/rust/src/llvm/include/llvm/Support/Casting.h", line=255,
    function=0x7fffed905de0 <_ZZN4llvm4castINS_8FunctionENS_5ValueEEENS_10cast_rettyIT_PT0_E8ret_typeES6_E19__PRETTY_FUNCTION__> "typename llvm::cast_retty<X, Y*>::ret_type llvm::cast(Y*) [with X = llvm::Function; Y = llvm::Value; typename llvm::cast_retty<X, Y*>::ret_type = llvm::Function*]") at assert.c:92
#3  0x00007ffff71e7412 in __GI___assert_fail (assertion=assertion@entry=0x7fffec290930 "isa<X>(Val) && \"cast<Ty>() argument of incompatible type!\"",
    file=file@entry=0x7fffec28dfd0 "/home/void/rust/src/llvm/include/llvm/Support/Casting.h", line=line@entry=255,
    function=function@entry=0x7fffed905de0 <_ZZN4llvm4castINS_8FunctionENS_5ValueEEENS_10cast_rettyIT_PT0_E8ret_typeES6_E19__PRETTY_FUNCTION__> "typename llvm::cast_retty<X, Y*>::ret_type llvm::cast(Y*) [with X = llvm::Function; Y = llvm::Value; typename llvm::cast_retty<X, Y*>::ret_type = llvm::Function*]") at assert.c:101
#4  0x00007fffebe9422b in llvm::cast<llvm::Function, llvm::Value> (Val=<optimized out>) at /home/void/rust/src/llvm/include/llvm/IR/Value.h:804
#5  0x00007fffebe943a8 in llvm::unwrap<llvm::Function> (P=<optimized out>) at /home/void/rust/src/llvm/include/llvm/IR/Value.h:845
#6  LLVMSetFunctionCallConv (Fn=<optimized out>, CC=0) at /home/void/rust/src/llvm/lib/IR/Core.cpp:2084
#7  0x00007fffea19d91b in rustc_codegen_llvm::llvm::SetFunctionCallConv (fn_=0x5555559ca8b8, cc=<optimized out>) at librustc_codegen_llvm/llvm/mod.rs:119
#8  rustc_codegen_llvm::declare::declare_raw_fn (cx=0x7ffffffeef40, name=..., callconv=rustc_codegen_llvm::llvm::ffi::CCallConv, ty=<optimized out>) at librustc_codegen_llvm/declare.rs:67
#9  0x00007fffea188d13 in rustc_codegen_llvm::declare::declare_cfn (cx=<optimized out>, name=..., fn_type=<optimized out>) at librustc_codegen_llvm/declare.rs:119
#10 rustc_codegen_llvm::base::maybe_create_entry_wrapper::create_entry_fn (cx=0x7ffffffeef40, sp=..., rust_main=0x555555991ed8, rust_main_def_id=..., use_start_lang_item=false)
    at librustc_codegen_llvm/base.rs:574
#11 0x00007fffea18d471 in rustc_codegen_llvm::base::maybe_create_entry_wrapper (cx=<optimized out>) at librustc_codegen_llvm/base.rs:543
#12 rustc_codegen_llvm::base::compile_codegen_unit::module_codegen (tcx=..., cgu_name=...) at librustc_codegen_llvm/base.rs:1228
#13 0x00007fffea1608bb in rustc::dep_graph::graph::DepGraph::with_task_impl (self=0x7fffffff1038, key=..., cx=..., arg=..., no_tcx=<optimized out>, task=0x0,
    create_task=0x7fffea15fba0 <core::ops::function::FnOnce::call_once>, finish_task_and_alloc_depnode=0x7fffea15fd10 <core::ops::function::FnOnce::call_once>)
    at /home/void/rust/src/librustc/dep_graph/graph.rs:342
#14 0x00007fffea18afe0 in rustc::dep_graph::graph::DepGraph::with_task (self=0x7ffffffee8c0, cx=..., arg=..., task=<optimized out>, key=...)
    at /home/void/rust/src/librustc/dep_graph/graph.rs:208
#15 rustc_codegen_llvm::base::compile_codegen_unit (tcx=..., cgu_name=...) at librustc_codegen_llvm/base.rs:1188
#16 rustc_codegen_llvm::base::codegen_crate (tcx=..., rx=...) at librustc_codegen_llvm/base.rs:902
#17 0x00007fffea1c7840 in <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate (self=<optimized out>, tcx=..., rx=...)
    at librustc_codegen_llvm/lib.rs:214
#18 0x00007ffff7a0462e in rustc_driver::driver::phase_4_codegen::{{closure}} () at librustc_driver/driver.rs:1368
#19 rustc::util::common::time_ext (do_it=<optimized out>, sess=..., what=..., f=...) at /home/void/rust/src/librustc/util/common.rs:163
#20 0x00007ffff79fa344 in rustc::util::common::time (sess=0x7fffffff8ec0, what=..., f=...) at /home/void/rust/src/librustc/util/common.rs:157
#21 rustc_driver::driver::phase_4_codegen (codegen_backend=..., tcx=..., rx=...) at librustc_driver/driver.rs:1368
#22 0x00007ffff7ae2d19 in rustc_driver::driver::compile_input::{{closure}} (tcx=..., analysis=..., rx=..., result=...) at librustc_driver/driver.rs:328
#23 0x00007ffff7ae291c in rustc_driver::driver::phase_3_run_analysis_passes::{{closure}} (tcx=...) at librustc_driver/driver.rs:1351
#24 rustc::ty::context::tls::enter_global::{{closure}}::{{closure}} () at /home/void/rust/src/librustc/ty/context.rs:2054
#25 rustc::ty::context::tls::enter_context::{{closure}} () at /home/void/rust/src/librustc/ty/context.rs:2022
#26 rustc::ty::context::tls::set_tlv (value=<optimized out>, f=...) at /home/void/rust/src/librustc/ty/context.rs:1961
#27 0x00007ffff7ab2817 in rustc::ty::context::tls::enter_context (context=0x7fffffff0ed0, f=...) at /home/void/rust/src/librustc/ty/context.rs:2021
#28 rustc::ty::context::tls::enter_global::{{closure}} () at /home/void/rust/src/librustc/ty/context.rs:2053
#29 rustc::ty::context::tls::with_thread_locals::{{closure}}::{{closure}} (current=0x7ffff7fbe778) at /home/void/rust/src/librustc/ty/context.rs:2011
#30 <std::thread::local::LocalKey<T>>::try_with (self=<optimized out>, f=...) at /home/void/rust/src/libstd/thread/local.rs:294
#31 0x00007ffff7ab2f25 in <std::thread::local::LocalKey<T>>::with (self=0x7ffffffee8c0, f=...) at /home/void/rust/src/libstd/thread/local.rs:248
#32 rustc::ty::context::tls::with_thread_locals::{{closure}} (span_dbg=0x7ffff7fbe768) at /home/void/rust/src/librustc/ty/context.rs:2003
#33 <std::thread::local::LocalKey<T>>::try_with (self=<optimized out>, f=...) at /home/void/rust/src/libstd/thread/local.rs:294
#34 0x00007ffff7a6c1fe in <std::thread::local::LocalKey<T>>::with (self=0x7ffffffee8c0, f=...) at /home/void/rust/src/libstd/thread/local.rs:248
#35 rustc::ty::context::tls::with_thread_locals (f=...) at /home/void/rust/src/librustc/ty/context.rs:1995
---Type <return> to continue, or q <return> to quit---
#36 rustc::ty::context::tls::enter_global (gcx=0x7fffffff7c00, f=...) at /home/void/rust/src/librustc/ty/context.rs:2033
#37 rustc::ty::context::TyCtxt::create_and_enter (s=<optimized out>, cstore=..., local_providers=..., extern_providers=..., arenas=0x7fffffff7c00, resolutions=..., hir=...,
    on_disk_query_result_cache=..., crate_name=..., tx=..., output_filenames=0x7fffffff85c0, f=...) at /home/void/rust/src/librustc/ty/context.rs:1249
#38 0x00007ffff7a5d5b3 in rustc_driver::driver::phase_3_run_analysis_passes (codegen_backend=..., control=<optimized out>, sess=0x7fffffff8ec0, cstore=0x7fffffff87c0, hir_map=...,
    analysis=..., resolutions=..., arenas=0x7fffffff7c00, name=..., output_filenames=0x7fffffff85c0, f=...) at librustc_driver/driver.rs:1259
#39 0x00007ffff79f8547 in rustc_driver::driver::compile_input (codegen_backend=..., sess=<optimized out>, cstore=<optimized out>, input_path=<optimized out>, input=<optimized out>,
    outdir=<optimized out>, output=<optimized out>, addl_plugins=..., control=0x7fffffff8970) at librustc_driver/driver.rs:287
#40 0x00007ffff7acfb4e in rustc_driver::run_compiler_with_pool (matches=..., sopts=..., cfg=..., callbacks=..., file_loader=..., emitter_dest=...) at librustc_driver/lib.rs:563
#41 0x00007ffff7a60143 in rustc_driver::run_compiler::{{closure}} (sopts=...) at librustc_driver/lib.rs:485
#42 rustc_driver::driver::spawn_thread_pool::{{closure}} () at librustc_driver/driver.rs:76
#43 <scoped_tls::ScopedKey<T>>::set (self=<optimized out>, t=<optimized out>, f=...) at /home/void/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
#44 0x00007ffff7aced1e in rustc_driver::driver::spawn_thread_pool (f=..., opts=...) at librustc_driver/driver.rs:75
#45 rustc_driver::run_compiler (args=..., callbacks=..., file_loader=..., emitter_dest=...) at librustc_driver/lib.rs:484
#46 0x00007ffff7a5fc8e in rustc_driver::main::{{closure}} () at librustc_driver/lib.rs:1746
#47 rustc_driver::run::{{closure}}::{{closure}} () at librustc_driver/lib.rs:190
#48 <scoped_tls::ScopedKey<T>>::set (self=<optimized out>, t=0x7fffffffe080, f=...) at /home/void/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
#49 0x00007ffff7a5fb6f in syntax::with_globals::{{closure}} () at /home/void/rust/src/libsyntax/lib.rs:106
#50 <scoped_tls::ScopedKey<T>>::set (self=<optimized out>, t=0x7fffffffe030, f=...) at /home/void/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
#51 0x00007ffff7a35c22 in syntax::with_globals (f=...) at /home/void/rust/src/libsyntax/lib.rs:105
#52 rustc_driver::run::{{closure}} () at librustc_driver/lib.rs:189
#53 rustc_driver::monitor::{{closure}} () at librustc_driver/lib.rs:1661
#54 <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once (self=..., _args=<optimized out>) at /home/void/rust/src/libstd/panic.rs:313
#55 std::panicking::try::do_call (data=<optimized out>) at /home/void/rust/src/libstd/panicking.rs:310
#56 0x00007ffff764c30a in __rust_maybe_catch_panic (f=0x0, data=0x2 <error: Cannot access memory at address 0x2>, data_ptr=0x7fffffffe1f0, vtable_ptr=0x7fffffffe1f8)
    at libpanic_unwind/lib.rs:102
#57 0x00007ffff7a35b82 in std::panicking::try (f=...) at /home/void/rust/src/libstd/panicking.rs:289
#58 0x00007ffff7ab94cd in std::panic::catch_unwind (f=...) at /home/void/rust/src/libstd/panic.rs:392
#59 rustc_driver::in_named_rustc_thread (name=..., f=...) at librustc_driver/lib.rs:1575
#60 0x00007ffff7ab9604 in rustc_driver::in_rustc_thread (f=...) at librustc_driver/lib.rs:1586
#61 rustc_driver::monitor (f=...) at librustc_driver/lib.rs:1660
#62 0x00007ffff7ab8c06 in rustc_driver::run (run_compiler=...) at librustc_driver/lib.rs:188
#63 0x00007ffff7ad999b in rustc_driver::main () at librustc_driver/lib.rs:1739
#64 0x0000555555554863 in std::rt::lang_start::{{closure}} () at /home/void/rust/src/libstd/rt.rs:74
#65 0x00007ffff7635dc3 in std::rt::lang_start_internal::{{closure}} () at libstd/rt.rs:59
#66 std::panicking::try::do_call (data=0x7fffffffe540 "\340\345\377\377\377\177") at libstd/panicking.rs:310
#67 0x00007ffff764c30a in __rust_maybe_catch_panic (f=0x0, data=0x2 <error: Cannot access memory at address 0x2>, data_ptr=0x7fffffffe530, vtable_ptr=0x7fffffffe538)
    at libpanic_unwind/lib.rs:102
#68 0x00007ffff7635c4c in std::panicking::try (f=...) at libstd/panicking.rs:289
#69 0x00007ffff763c1ad in std::panic::catch_unwind (f=...) at libstd/panic.rs:392
#70 std::rt::lang_start_internal (main=..., argc=2, argv=0x7fffffffe718) at libstd/rt.rs:58
#71 0x00005555555548ca in main ()
#72 0x00007ffff71db3db in __libc_start_main (main=0x5555555548a0 <main>, argc=2, argv=0x7fffffffe718, init=<optimized out>, fini=<optimized out>, rtld_fini=<optimized out>,
    stack_end=0x7fffffffe708) at ../csu/libc-start.c:308
#73 0x000055555555479a in _start () at ../sysdeps/x86_64/start.S:120
