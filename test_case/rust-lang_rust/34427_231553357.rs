 rust
Program received signal SIGILL, Illegal instruction.
[Switching to Thread 0xb2a282b0 (LWP 25309)]
collections::vec::{{impl}}::extend_desugared<core::option::Option<rustc_trans::mir::constant::Const>,core::iter::Map<core::ops::Range<usize>, closure>> (self=<optimized out>, iterator=...)
    at src/libcollections/vec.rs:1424
1424                    ptr::write(self.get_unchecked_mut(len), element);
(gdb) bt
#0  collections::vec::{{impl}}::extend_desugared<core::option::Option<rustc_trans::mir::constant::Const>,core::iter::Map<core::ops::Range<usize>, closure>> (self=<optimized out>, iterator=...) at src/libcollections/vec.rs:1424
#1  collections::vec::{{impl}}::from_iter<core::option::Option<rustc_trans::mir::constant::Const>,core::iter::Map<core::ops::Range<usize>, closure>> (iter=...) at src/libcollections/vec.rs:1325
#2  rustc_data_structures::indexed_vec::{{impl}}::from_iter<rustc::mir::repr::Local,core::option::Option<rustc_trans::mir::constant::Const>,core::iter::Map<core::ops::Range<usize>, closure>> (iter=...) at src/librustc_data_structures/indexed_vec.rs:180
#3  core::iter::iterator::Iterator::collect<core::iter::Map<core::ops::Range<usize>, closure>,rustc_data_structures::indexed_vec::IndexVec<rustc::mir::repr::Local, core::option::Option<rustc_trans::mir::constant::Const>>> (self=...) at src/libcore/iter/iterator.rs:1207
#4  rustc_trans::mir::constant::{{impl}}::new (ccx=<optimized out>, mir=<optimized out>, substs=<optimized out>, args=...)
    at src/librustc_trans/mir/constant.rs:221
#5  0xb6554cd4 in rustc_trans::mir::constant::{{impl}}::trans_constant (self=0xb2a15170, bcx=<optimized out>, constant=<optimized out>)
    at src/librustc_trans/mir/constant.rs:911
#6  0xb65541e0 in rustc_trans::mir::operand::{{impl}}::trans_operand (self=0x0, bcx=0xb2a13b60, operand=<optimized out>)
    at src/librustc_trans/mir/operand.rs:234
#7  0xb65644cc in rustc_trans::mir::rvalue::{{impl}}::trans_rvalue_operand (self=<optimized out>, bcx=..., rvalue=<optimized out>, 
    debug_loc=...) at src/librustc_trans/mir/rvalue.rs:476
#8  0xb654aa28 in rustc_trans::mir::statement::{{impl}}::trans_statement (statement=<optimized out>, self=<optimized out>, bcx=...)
    at src/librustc_trans/mir/statement.rs:36
#9  rustc_trans::mir::block::{{impl}}::trans_block (self=<optimized out>, bb=...) at src/librustc_trans/mir/block.rs:103
#10 0xb64354e8 in rustc_trans::mir::trans_mir (fcx=<optimized out>) at src/librustc_trans/mir/mod.rs:249
#11 0xb642df4c in rustc_trans::base::trans_closure (ccx=<optimized out>, decl=<optimized out>, body=<optimized out>, 
    llfndecl=<optimized out>, instance=..., inlined_id=271, sig=<optimized out>, abi=<optimized out>, closure_env=...)
    at src/librustc_trans/base.rs:1854
#12 0xb6456f78 in rustc_trans::base::trans_instance (ccx=<optimized out>, instance=...) at src/librustc_trans/base.rs:1953
#13 rustc_trans::trans_item::{{impl}}::define (self=<optimized out>, ccx=<optimized out>) at src/librustc_trans/trans_item.rs:89
#14 0xb6446660 in rustc_trans::base::trans_crate (tcx=..., mir_map=<optimized out>, analysis=...) at src/librustc_trans/base.rs:2638
#15 0xb6e0bc74 in rustc_driver::driver::phase_4_translate_to_llvm::{{closure}} () at src/librustc_driver/driver.rs:1055
#16 rustc::util::common::time<rustc_trans::CrateTranslation,closure> (what=..., f=..., do_it=<optimized out>)
    at src/librustc/util/common.rs:38
#17 rustc_driver::driver::phase_4_translate_to_llvm (tcx=..., mir_map=..., analysis=...) at src/librustc_driver/driver.rs:1053
#18 0xb6e0a1a4 in rustc_driver::driver::compile_input::{{closure}} (tcx=..., mir_map=..., analysis=..., result=...)
    at src/librustc_driver/driver.rs:204
#19 0xb6e167f4 in rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}<closure,core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>> (tcx=...) at src/librustc_driver/driver.rs:1016
#20 0xb6e1b8a4 in std::thread::local::{{impl}}::with<core::cell::Cell<core::option::Option<(*const rustc::ty::context::tls::ThreadLocalGlobalCtxt, *const rustc::ty::context::tls::ThreadLocalInterners)>>,closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> (f=..., self=<optimized out>) at src/libstd/thread/local.rs:245
#21 rustc::ty::context::tls::enter<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> (gcx=<optimized out>, interners=<optimized out>, f=...) at src/librustc/ty/context.rs:903
#22 std::thread::local::{{impl}}::with<core::cell::Cell<fn(syntax_pos::Span, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>>,closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> (f=..., self=<optimized out>) at src/libstd/thread/local.rs:245
#23 rustc::ty::context::tls::enter_global<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> (gcx=..., f=...) at src/librustc/ty/context.rs:887
#24 rustc::ty::context::{{impl}}::create_and_enter<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> (s=<optimized out>, arenas=<optimized out>, def_map=..., named_region_map=..., 
    map=..., freevars=..., maybe_unused_trait_imports=..., region_maps=..., lang_items=..., stability=..., crate_name=..., f=...)
    at src/librustc/ty/context.rs:685
#25 0xb6e0ff50 in rustc_driver::driver::phase_3_run_analysis_passes<closure,core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>> (sess=<optimized out>, hir_map=..., analysis=..., resolutions=..., arenas=<optimized out>, 
    name=..., f=...) at src/librustc_driver/driver.rs:898
#26 0xb6dd55e0 in rustc_driver::driver::compile_input (sess=<optimized out>, cstore=<optimized out>, cfg=..., input=0xb2a25ff8, 
    outdir=0xb2a26028, output=0xb2a26018, addl_plugins=..., control=0xb2a257e8) at src/librustc_driver/driver.rs:170
#27 0xb6dbf10c in rustc_driver::run_compiler_with_file_loader<syntax::codemap::RealFileLoader> (args=..., callbacks=..., 
    loader=<optimized out>) at src/librustc_driver/lib.rs:220
#28 rustc_driver::run_compiler (args=..., callbacks=...) at src/librustc_driver/lib.rs:157
#29 0xb6dcce68 in rustc_driver::run::{{closure}} () at src/librustc_driver/lib.rs:135
#30 rustc_driver::monitor::{{closure}}<closure> () at src/librustc_driver/lib.rs:1043
#31 std::panic::{{impl}}::call_once<(),closure> (self=...) at src/libstd/panic.rs:251
#32 std::panicking::try::call<closure> (f=0xb2a27c54) at src/libstd/panicking.rs:272
#33 0xb6b32238 in panic_unwind::__rust_maybe_catch_panic (f=0x40000000, data=0xaa482e18 "", data_ptr=0xb2a27c50, vtable_ptr=0xb2a27c4c)
    at src/libpanic_unwind/lib.rs:91
#34 0xb6dcd8a8 in std::thread::local::{{impl}}::with<core::cell::Cell<usize>,closure,core::result::Result<(), Box<Any>>> (
    self=<optimized out>, f=...) at src/libstd/thread/local.rs:245
#33 0xb6b32238 in panic_unwind::__rust_maybe_catch_panic (f=0x40000000, data=0xaa482e18 "", data_ptr=0xb2a27c50, vtable_ptr=0xb2a27c4c)
    at src/libpanic_unwind/lib.rs:91
#34 0xb6dcd8a8 in std::thread::local::{{impl}}::with<core::cell::Cell<usize>,closure,core::result::Result<(), Box<Any>>> (
    self=<optimized out>, f=...) at src/libstd/thread/local.rs:245
#35 std::panicking::try<(),std::panic::AssertUnwindSafe<closure>> (f=...) at src/libstd/panicking.rs:235
#36 std::panic::catch_unwind<std::panic::AssertUnwindSafe<closure>,()> (f=...) at src/libstd/panic.rs:307
#37 alloc::boxed::{{impl}}::call_box<(),closure> (self=0x7f56eb88, args=<optimized out>) at src/liballoc/boxed.rs:544
#38 0xb6b218e8 in alloc::boxed::{{impl}}::call_once<(),()> (self=...) at src/liballoc/boxed.rs:554
#39 std::sys_common::thread::start_thread (main=<optimized out>) at src/libstd/sys/common/thread.rs:23
#40 std::sys::thread::{{impl}}::new::thread_start (main=0x7f56ec48) at src/libstd/sys/unix/thread.rs:74
#41 0xb2cd5fa0 in start_thread (arg=0xb2a282b0) at pthread_create.c:314
#42 0xb69a8d9c in ?? () at ../ports/sysdeps/unix/sysv/linux/arm/nptl/../clone.S:92 from /lib/arm-linux-gnueabihf/libc.so.6
