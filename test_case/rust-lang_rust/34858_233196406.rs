
#0  llvm::DenseMapBase<llvm::DenseMap<llvm::MDNode*, std::vector<llvm::TypedTrackingMDRef<llvm::MDNode>, std::allocator<llvm::TypedTrackingMDRef<llvm::MDNode> > >, llvm::DenseMapInfo<llvm::MDNode*>, llvm::detail::DenseMapPair<llvm::MDNode*, std::vector<llvm::TypedTrackingMDRef<llvm::MDNode>, std::allocator<llvm::TypedTrackingMDRef<llvm::MDNode> > > > >, llvm::MDNode*, std::vector<llvm::TypedTrackingMDRef<llvm::MDNode>, std::allocator<llvm::TypedTrackingMDRef<llvm::MDNode> > >, llvm::DenseMapInfo<llvm::MDNode*>, llvm::detail::DenseMapPair<llvm::MDNode*, std::vector<llvm::TypedTrackingMDRef<llvm::MDNode>, std::allocator<llvm::TypedTrackingMDRef<llvm::MDNode> > > > >::destroyAll (this=0x7f9b82be4080)
    at //home//dan//installed//rust//src//llvm//include/llvm/ADT/DenseMap.h:270
#1  llvm::DenseMap<llvm::MDNode*, std::vector<llvm::TypedTrackingMDRef<llvm::MDNode>, std::allocator<llvm::TypedTrackingMDRef<llvm::MDNode> > >, llvm::DenseMapInfo<llvm::MDNode*>, llvm::detail::DenseMapPair<llvm::MDNode*, std::vector<llvm::TypedTrackingMDRef<llvm::MDNode>, std::allocator<llvm::TypedTrackingMDRef<llvm::MDNode> > > > >::~DenseMap (this=0x7f9b82be4080, __in_chrg=<optimised out>) at //home//dan//installed//rust//src//llvm//include/llvm/ADT/DenseMap.h:574
#2  llvm::DIBuilder::~DIBuilder (this=0x7f9b82be3f00, __in_chrg=<optimised out>) at //home//dan//installed//rust//src//llvm//include/llvm/IR/DIBuilder.h:35
#3  LLVMDIBuilderDispose (Builder=0x7f9b82be3f00) at /home/dan/installed/rust/src/rustllvm/RustWrapper.cpp:288
#4  0x00007f9b8e002689 in rustc_trans::debuginfo::finalize (cx=0x7f9b835f1238) at /home/dan/installed/rust/src/librustc_trans/debuginfo/mod.rs:183
#5  0x00007f9b8df6e592 in rustc_trans::base::trans_crate (tcx=..., mir_map=0x7f9b835f22e8, analysis=...) at /home/dan/installed/rust/src/librustc_trans/base.rs:2554
#6  0x00007f9b8fed67bf in rustc_driver::driver::phase_4_translate_to_llvm::{{closure}} () at /home/dan/installed/rust/src/librustc_driver/driver.rs:1011
#7  0x00007f9b8fc93c6e in {{inlined-root}}::time<rustc_trans::CrateTranslation,closure> (do_it=false, what=..., f=...) at /home/dan/installed/rust/src/librustc/util/common.rs:38
#8  0x00007f9b8fea6b0c in rustc_driver::driver::phase_4_translate_to_llvm (tcx=..., mir_map=..., analysis=...) at /home/dan/installed/rust/src/librustc_driver/driver.rs:1009
#9  0x00007f9b8ff079ff in rustc_driver::driver::compile_input::{{closure}} (tcx=..., mir_map=..., analysis=..., result=...) at /home/dan/installed/rust/src/librustc_driver/driver.rs:203
#10 0x00007f9b8fed00e6 in rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}<closure,core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>> (tcx=...)
    at /home/dan/installed/rust/src/librustc_driver/driver.rs:972
#11 0x00007f9b8fecaf2b in rustc::ty::context::tls::enter::_$u7b$$u7b$closure$u7d$$u7d$::h9d08b024ac32fd21 () at /home/dan/installed/rust/src/librustc_driver/pretty.rs:285
#12 0x00007f9b8fbf4600 in {{inlined-root}}::with<core::cell::Cell<core::option::Option<(*const rustc::ty::context::tls::ThreadLocalGlobalCtxt, *const rustc::ty::context::tls::ThreadLocalInterners)>>,closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> (self=0x7f9b8a739510 <rustc::ty::context::tls::TLS_TCX::h4b589da3ad4497aa>, f=...)
    at /home/dan/installed/rust/src/libstd/thread/local.rs:245
#13 0x00007f9b8fc557b1 in {{inlined-root}}::enter<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> (gcx=0x7f9b835f5628, 
    interners=0x7f9b835f5628, f=...) at /home/dan/installed/rust/src/librustc/ty/context.rs:902
#14 0x00007f9b8fecce69 in rustc::ty::context::tls::enter_global::_$u7b$$u7b$closure$u7d$$u7d$::h8575bf8dd37062fa () at /home/dan/installed/rust/src/librustc_driver/pretty.rs:285
#15 0x00007f9b8fbf8b26 in {{inlined-root}}::with<core::cell::Cell<fn(syntax_pos::Span, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>>,closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> (self=0x7f9b85b8dc00 <syntax_pos::SPAN_DEBUG::h5629cb242cbcd20f>, f=...) at /home/dan/installed/rust/src/libstd/thread/local.rs:245
#16 0x00007f9b8fc5515c in {{inlined-root}}::enter_global<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> (gcx=..., f=...)
    at /home/dan/installed/rust/src/librustc/ty/context.rs:886
#17 0x00007f9b8fc5bb2e in {{inlined-root}}::create_and_enter<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> (s=0x7f9b835fb040, 
    arenas=0x7f9b835f9800, def_map=..., named_region_map=..., map=..., freevars=..., maybe_unused_trait_imports=..., region_maps=..., lang_items=..., stability=..., crate_name=..., f=...)
    at /home/dan/installed/rust/src/librustc/ty/context.rs:685
#18 0x00007f9b8fea67e5 in rustc_driver::driver::phase_3_run_analysis_passes<closure,core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>> (sess=0x7f9b835fb040, hir_map=..., analysis=..., 
    resolutions=..., arenas=0x7f9b835f9800, name=..., f=...) at /home/dan/installed/rust/src/librustc_driver/driver.rs:854
#19 0x00007f9b8fe9e5f0 in rustc_driver::driver::compile_input (sess=0x7f9b835fb040, cstore=0x7f9b82a1e210, cfg=..., input=0x7f9b835fbfa0, outdir=0x7f9b835fc190, output=0x7f9b835fc170, addl_plugins=..., control=0x7f9b835faae0)
    at /home/dan/installed/rust/src/librustc_driver/driver.rs:169
#20 0x00007f9b8feb3ba7 in rustc_driver::run_compiler_with_file_loader<syntax::codemap::RealFileLoader> (args=..., callbacks=..., loader=0x1d1d1d1d1d1d1d1d) at /home/dan/installed/rust/src/librustc_driver/lib.rs:220
#21 0x00007f9b8feb2b19 in rustc_driver::run_compiler (args=..., callbacks=...) at /home/dan/installed/rust/src/librustc_driver/lib.rs:157
#22 0x00007f9b8ff0c056 in rustc_driver::run::{{closure}} () at /home/dan/installed/rust/src/librustc_driver/lib.rs:135
#23 0x00007f9b8ff06e22 in rustc_driver::monitor::{{closure}}<closure> () at /home/dan/installed/rust/src/librustc_driver/lib.rs:1043
#24 0x00007f9b8fe7380a in {{inlined-root}}::call_once<(),closure> (self=..., _args=0) at /home/dan/installed/rust/src/libstd/panic.rs:256
#25 0x00007f9b8fec7234 in std::panicking::try::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h4767e083ebe96610 () at /home/dan/installed/rust/src/librustc_driver/pretty.rs:285
#26 0x00007f9b8fbd44b5 in {{inlined-root}}::call<closure> (f=0x7f9b835fe778) at /home/dan/installed/rust/src/libstd/panicking.rs:272
#27 0x00007f9b8f54a458 in __rust_try () from /home/dan/installed/rust/x86_64-unknown-linux-gnu/stage1/lib/libstd-f53fb285.so
#28 0x00007f9b8f54a3b5 in panic_unwind::__rust_maybe_catch_panic (f=0x7f9b8fbd44a0 <{{inlined-root}}::call<closure>>, data=0x7f9b835fe778 "\260\350_\203\233\177", data_ptr=0x7f9b835fe760, vtable_ptr=0x7f9b835fe758)
    at /home/dan/installed/rust/src/libpanic_unwind/lib.rs:91
#29 0x00007f9b8fec719e in std::panicking::try::_$u7b$$u7b$closure$u7d$$u7d$::h5edd11d34a445b2b () at /home/dan/installed/rust/src/librustc_driver/pretty.rs:285
#30 0x00007f9b8fbf76b0 in {{inlined-root}}::with<core::cell::Cell<usize>,closure,core::result::Result<(), Box<Any>>> (self=0x7f9b8f9029e0 <std::panicking::PANIC_COUNT::h99b79c8949805cd2>, f=...)
    at /home/dan/installed/rust/src/libstd/thread/local.rs:245
#31 0x00007f9b8fbd43d8 in {{inlined-root}}::try<(),std::panic::AssertUnwindSafe<closure>> (f=...) at /home/dan/installed/rust/src/libstd/panicking.rs:235
#32 0x00007f9b8fbd3a21 in {{inlined-root}}::catch_unwind<std::panic::AssertUnwindSafe<closure>,()> (f=...) at /home/dan/installed/rust/src/libstd/panic.rs:312
#33 0x00007f9b8fec6fa6 in std::thread::Builder::spawn::_$u7b$$u7b$closure$u7d$$u7d$::hf8ba3712412ac8f6 () at /home/dan/installed/rust/src/librustc_driver/pretty.rs:285
#34 0x00007f9b8fc3a15d in {{inlined-root}}::call_box<(),closure> (self=0x7f9b83677f20, args=0) at /home/dan/installed/rust/src/liballoc/boxed.rs:587
#35 0x00007f9b8f46541c in {{inlined-root}}::call_once<(),()> (self=..., args=0) at /home/dan/installed/rust/src/liballoc/boxed.rs:597
#36 0x00007f9b8f4ee76f in std::sys_common::thread::start_thread (main=0x7f9b836a4050) at /home/dan/installed/rust/src/libstd/sys/common/thread.rs:23
#37 0x00007f9b8f5185d5 in std::sys::thread::{{impl}}::new::thread_start (main=0x7f9b836a4050) at /home/dan/installed/rust/src/libstd/sys/unix/thread.rs:73
#38 0x00007f9b852336aa in start_thread (arg=0x7f9b835ff700) at pthread_create.c:333
#39 0x00007f9b8f0a513d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:109
