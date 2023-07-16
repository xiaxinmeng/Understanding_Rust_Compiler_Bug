rust
/tmp/tr 
$ time /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc --edition=2018 -C debuginfo=2 /tmp/tr/src/main.rs -v && RUST_LOG=trace RUST_BACKTRACE=1 ./main 
stack backtrace:
   0: <rustc::mir::interpret::error::EvalErrorKind<'tcx, O>>::description::h93cabbaeb77cf8e9 (0x7ff4add1b014)
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/mir/interpret/error.rs:430
   1: rustc_codegen_ssa::mir::block::<impl rustc_codegen_ssa::mir::FunctionCx<'a, 'tcx, Bx>>::codegen_terminator::hf5867a49a7c0092c (0x7ff4ade85850)
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_ssa/mir/block.rs:396
   2: rustc_codegen_ssa::mir::block::<impl rustc_codegen_ssa::mir::FunctionCx<'a, 'tcx, Bx>>::codegen_block::hd71f5a610ab19665 (0x7ff4ade821fe)
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_ssa/mir/block.rs:38
      rustc_codegen_ssa::mir::codegen_mir::h97380ecbeb32f317
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_ssa/mir/mod.rs:343
   3: rustc_codegen_ssa::base::codegen_instance::h08fe7402fba94e2a (0x7ff4add0c5d2)
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_ssa/base.rs:436
   4: rustc_codegen_ssa::mono_item::MonoItemExt::define::hc1b44ea0cbf6e3a8 (0x7ff4adda9047)
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_ssa/mono_item.rs:43
   5: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen::he0cfdc914df0c847 (0x7ff4addbe1bd)
             at src/librustc_codegen_llvm/base.rs:178
   6: rustc::dep_graph::graph::DepGraph::with_task_impl::h8ca210522f66e976 (0x7ff4ade33d9c)
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:295
      rustc::dep_graph::graph::DepGraph::with_task::h9677e9916ca4c2fb
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:181
   7: rustc_codegen_llvm::base::compile_codegen_unit::h2c505b8e6ff00db0 (0x7ff4addbddfa)
             at src/librustc_codegen_llvm/base.rs:145
   8: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::ExtraBackendMethods>::compile_codegen_unit::h92d011270b11d0bf (0x7ff4adc16af1)
             at src/librustc_codegen_llvm/lib.rs:144
   9: rustc_codegen_ssa::base::codegen_crate::hdac3c97bf6d7fbca (0x7ff4add0a243)
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_ssa/base.rs:687
  10: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate::h888e69a0f96218ba (0x7ff4adc170d0)
             at src/librustc_codegen_llvm/lib.rs:304
  11: rustc_driver::driver::phase_4_codegen::{{closure}}::h227d1332dd6978c2 (0x7ff4bccf2cdc)
             at src/librustc_driver/driver.rs:1349
      rustc::util::common::time_ext::h194714b7b1369f7c
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/util/common.rs:150
      rustc::util::common::time::ha11a3c04898eac7a
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/util/common.rs:144
  12: rustc_driver::driver::phase_4_codegen::h6bcca110e885caa9 (0x7ff4bcce18e0)
             at src/librustc_driver/driver.rs:1349
  13: rustc_driver::driver::compile_input::{{closure}}::h231c43f5b8e6c3e9 (0x7ff4bcdf8c50)
             at src/librustc_driver/driver.rs:316
  14: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h8f36925bf5962022 (0x7ff4bcdf2b0d)
             at src/librustc_driver/driver.rs:1332
      rustc::ty::context::tls::enter_global::{{closure}}::{{closure}}::hef4ec43c7065bde2
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1963
      rustc::ty::context::tls::enter_context::{{closure}}::he652dca2629d0c4f
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1929
      rustc_rayon_core::tlv::with::h1749e7b1cef75566
             at /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.1.1/src/tlv.rs:19
  15: rustc::ty::context::tls::set_tlv::hc87f5ce672c3fe3c (0x7ff4bcc8a35f)
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1838
      rustc::ty::context::tls::enter_context::h540c161a5c470d51
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1928
      rustc::ty::context::tls::enter_global::{{closure}}::h79bd1d338d355e69
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1962
      rustc::ty::context::tls::with_thread_locals::{{closure}}::{{closure}}::h36513f108ba283a4
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1917
      <std::thread::local::LocalKey<T>>::try_with::hb5b48892a35874da
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/thread/local.rs:299
      <std::thread::local::LocalKey<T>>::with::hb27a948451c920d5
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/thread/local.rs:245
      rustc::ty::context::tls::with_thread_locals::{{closure}}::hb1b4ea8acb560389
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1909
      <std::thread::local::LocalKey<T>>::try_with::hfee4f67c590f43fa
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/thread/local.rs:299
      <std::thread::local::LocalKey<T>>::with::hf7d2ceb7d643b7ad
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/thread/local.rs:245
  16: rustc::ty::context::tls::with_thread_locals::hc28c29965271ab67 (0x7ff4bcd721e4)
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1901
      rustc::ty::context::tls::enter_global::h72db5c2568d3abbb
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1940
  17: rustc::ty::context::TyCtxt::create_and_enter::h26fa6283efd9a6b7 (0x7ff4bcd8c37d)
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1260
  18: rustc_driver::driver::phase_3_run_analysis_passes::he0bcbc46efa21744 (0x7ff4bcd03052)
             at src/librustc_driver/driver.rs:1229
  19: rustc_driver::driver::compile_input::h4d4398317b2c6d58 (0x7ff4bccdfb97)
             at src/librustc_driver/driver.rs:275
  20: rustc_driver::run_compiler_with_pool::he0ab58fb4a355771 (0x7ff4bce01fac)
             at src/librustc_driver/lib.rs:527
  21: rustc_driver::run_compiler::{{closure}}::h1e4c126fef6bd8dc (0x7ff4bcd9fa05)
             at src/librustc_driver/lib.rs:449
      rustc_driver::driver::spawn_thread_pool::{{closure}}::{{closure}}::hd8a1f072d8753d2c
             at src/librustc_driver/driver.rs:86
      rustc_rayon_core::thread_pool::ThreadPool::install::{{closure}}::h7a43dce80642a2b4
             at /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.1.1/src/thread_pool/mod.rs:158
      rustc_rayon_core::registry::Registry::in_worker_cold::{{closure}}::he1599453f909cbdd
             at /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.1.1/src/registry.rs:359
      <rustc_rayon_core::job::StackJob<L, F, R> as rustc_rayon_core::job::Job>::execute::{{closure}}::hc7a75fba1110d7b4
             at /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.1.1/src/job.rs:121
      <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h0fe22c82f5272699
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panic.rs:309
  22: std::panicking::try::do_call::hb69d76ef89a9d93d (0x7ff4bcd74e53)
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panicking.rs:297
  23: __rust_maybe_catch_panic (0x7ff4bcbb7ff0)
             at src/libpanic_unwind/lib.rs:92
  24: std::panicking::try::hfe22bc5b1e02df82 (0x7ff4bcd74aae)
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panicking.rs:276
  25: std::panic::catch_unwind::hab370608fba46ba6 (0x7ff4bcda25f8)
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panic.rs:388
  26: rustc_rayon_core::unwind::halt_unwinding::hee06b0c95c109e81 (0x7ff4bcc9a478)
             at /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.1.1/src/unwind.rs:18
  27: <rustc_rayon_core::job::StackJob<L, F, R> as rustc_rayon_core::job::Job>::execute::h693c3cfaa60523e8 (0x7ff4bcd1d4f0)
             at /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.1.1/src/job.rs:121
  28: rustc_rayon_core::job::JobRef::execute::ha52f551ecea678cd (0x7ff4b8702df3)
             at /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.1.1/src/job.rs:62
      rustc_rayon_core::registry::WorkerThread::execute::hc89f8cf7c1d41e9b
             at /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.1.1/src/registry.rs:609
      rustc_rayon_core::registry::WorkerThread::wait_until_cold::h5177fb21147a739c
             at /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.1.1/src/registry.rs:589
  29: rustc_rayon_core::registry::WorkerThread::wait_until::h12fac8c0f1ed4bb8 (0x7ff4b87039db)
             at /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.1.1/src/registry.rs:565
      rustc_rayon_core::registry::main_loop::{{closure}}::hb6a51d03c20496ef
             at /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.1.1/src/registry.rs:701
  30: rustc_driver::driver::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::h41edfded97a513b4 (0x7ff4bce0b6df)
             at src/librustc_driver/driver.rs:100
      <scoped_tls::ScopedKey<T>>::set::hb491eb13ee4c3b95
             at /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
  31: rustc_driver::driver::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::he70cf8f3dbad9ef5 (0x7ff4bcc8629b)
             at src/librustc_driver/driver.rs:99
      rustc::ty::context::tls::with_thread_locals::{{closure}}::{{closure}}::h6074966d5788cc51
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1917
      <std::thread::local::LocalKey<T>>::try_with::h077abb1178ec3502
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/thread/local.rs:299
      <std::thread::local::LocalKey<T>>::with::he2ee3cda01983d7a
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/thread/local.rs:245
      rustc::ty::context::tls::with_thread_locals::{{closure}}::h01bfe6c1c47008ff
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1909
      <std::thread::local::LocalKey<T>>::try_with::h4fcbc8d2e276cb37
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/thread/local.rs:299
      <std::thread::local::LocalKey<T>>::with::h07c04aa6c28bd983
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/thread/local.rs:245
  32: rustc_driver::driver::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::h6a1d65875583578a (0x7ff4bce0b776)
             at src/librustc_driver/driver.rs:98
      <scoped_tls::ScopedKey<T>>::set::h385c43b72356bc8f
             at /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
      rustc_driver::driver::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::h597bf30d030c4095
             at src/librustc_driver/driver.rs:97
      <scoped_tls::ScopedKey<T>>::set::hc0e454a26a369f5e
             at /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
  33: rustc_driver::driver::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::he26ea67043a46132 (0x7ff4bce1976d)
             at src/librustc_driver/driver.rs:96
      rustc_rayon_core::thread_pool::ThreadPool::scoped_pool::{{closure}}::ha394f83af0bf888f
             at /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.1.1/src/thread_pool/mod.rs:103
  34: __rust_maybe_catch_panic (0x7ff4bcbb7ff0)
             at src/libpanic_unwind/lib.rs:92
  35: std::panicking::try::h0324edf9a21513db (0x7ff4b86fcf5f)
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panicking.rs:276
      std::panic::catch_unwind::hac46a414d53b9e0d
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panic.rs:388
      rustc_rayon_core::unwind::halt_unwinding::h2b6105fb03509a92
             at /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.1.1/src/unwind.rs:18
      rustc_rayon_core::registry::main_loop::h1fda886cbb064140
             at /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.1.1/src/registry.rs:705
      rustc_rayon_core::registry::Registry::new::{{closure}}::hdc57e38d2eea6137
             at /home/xftroxgpx/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.1.1/src/registry.rs:139
      std::sys_common::backtrace::__rust_begin_short_backtrace::h3ee2e7302fbd8cab
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/sys_common/backtrace.rs:135
  36: std::thread::Builder::spawn_unchecked::{{closure}}::{{closure}}::h4d46a00a4516e335 (0x7ff4b86fddfb)
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/thread/mod.rs:469
      <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h88071d6ae379fc37
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panic.rs:309
      std::panicking::try::do_call::hd9ecda59b94bddcc
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panicking.rs:297
  37: __rust_maybe_catch_panic (0x7ff4bcbb7ff0)
             at src/libpanic_unwind/lib.rs:92
  38: std::panicking::try::h3d46a5104c396178 (0x7ff4b870035f)
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panicking.rs:276
      std::panic::catch_unwind::hc10ac859fe269334
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panic.rs:388
      std::thread::Builder::spawn_unchecked::{{closure}}::hed00129e1c5572f2
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/thread/mod.rs:468
      <F as alloc::boxed::FnBox<A>>::call_box::h3d507a5e4bf27391
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/liballoc/boxed.rs:734
  39: std::sys_common::thread::start_thread::h7c24e4f48ce975ba (0x7ff4bcba8d06)
             at src/libstd/sys_common/thread.rs:14
  40: std::sys::unix::thread::Thread::new::thread_start::h966b57701b78ecf1 (0x7ff4bcb9c3f5)
             at src/libstd/sys/unix/thread.rs:81
  41: start_thread (0x7ff4b862c73e)
             at /usr/src/debug/glibc/nptl/pthread_create.c:486
  42: __GI___clone (0x7ff4bca33bc2)
  43: <unknown> (0x0)

real	0m7.569s
user	0m1.512s
sys	0m2.920s
0 / 0
thread 'main' panicked at 'wuuuuuuattttttt??!attempt to multiply with overflow', /tmp/tr/src/main.rs:12:15
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:70
             at src/libstd/sys_common/backtrace.rs:58
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:215
   4: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:478
   5: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:385
   6: rust_begin_unwind
             at src/libstd/panicking.rs:312
   7: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
   8: core::panicking::panic
             at src/libcore/panicking.rs:49
   9: main::main
             at ./src/main.rs:12
  10: std::rt::lang_start::{{closure}}
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/rt.rs:64
  11: std::panicking::try::do_call
             at src/libstd/rt.rs:49
             at src/libstd/panicking.rs:297
  12: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:92
  13: std::panicking::try
             at src/libstd/panicking.rs:276
  14: std::panic::catch_unwind
             at src/libstd/panic.rs:388
  15: std::rt::lang_start_internal
             at src/libstd/rt.rs:48
  16: std::rt::lang_start
             at /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/rt.rs:64
  17: main
  18: __libc_start_main
             at ../csu/libc-start.c:308
  19: _start
             at ../sysdeps/x86_64/start.S:120

