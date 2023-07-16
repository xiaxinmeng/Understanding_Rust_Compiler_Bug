
#0  0x00007ffff1cec8e0 in LLVMTypeOf () from /home/logic/.multirust/toolchains/nightly-2016-11-06-x86_64-unknown-linux-gnu/bin/../lib/librustc_llvm-6eb85298.so
#1  0x00007ffff635336f in rustc_trans::common::val_ty () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/common.rs:708
#2  rustc_trans::base::from_immediate () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/base.rs:635
#3  0x00007ffff63c53d2 in rustc_trans::mir::MirContext::store_operand_direct ()
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/mir/operand.rs:269
#4  0x00007ffff63c5888 in rustc_trans::mir::operand::{{impl}}::store_operand::{{closure}} ()
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/mir/operand.rs:250
#5  rustc_trans::common::BlockAndBuilder::with_block<closure,()> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/common.rs:565
#6  rustc_trans::mir::MirContext::store_operand () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/mir/operand.rs:250
#7  rustc_trans::mir::MirContext::trans_rvalue () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/mir/rvalue.rs:181
#8  0x00007ffff63b4be1 in rustc_trans::mir::MirContext::trans_statement ()
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/mir/statement.rs:36
#9  rustc_trans::mir::MirContext::trans_block () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/mir/block.rs:104
#10 0x00007ffff63b3217 in rustc_trans::mir::trans_mir () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/mir/mod.rs:302
#11 0x00007ffff635715b in rustc_trans::base::trans_closure () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/base.rs:1040
#12 0x00007ffff63d1ece in rustc_trans::base::trans_instance () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/base.rs:1060
#13 rustc_trans::trans_item::TransItem::define () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/trans_item.rs:85
#14 0x00007ffff635a44c in rustc_trans::base::trans_crate::{{closure}} () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/base.rs:1636
#15 rustc::dep_graph::graph::DepGraph::with_task<closure,()> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/dep_graph/graph.rs:77
#16 rustc_trans::base::trans_crate () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/base.rs:1634
#17 0x00007ffff7b3e8d2 in rustc_driver::driver::phase_4_translate_to_llvm::{{closure}} ()
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/driver.rs:1025
#18 rustc::util::common::time<rustc_trans::CrateTranslation,closure> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/util/common.rs:38
#19 rustc_driver::driver::phase_4_translate_to_llvm () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/driver.rs:1023
#20 0x00007ffff7b75ba1 in rustc_driver::driver::compile_input::{{closure}} ()
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/driver.rs:205
#21 0x00007ffff7b6809f in rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}<closure,core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/driver.rs:979
#22 0x00007ffff7b32cb1 in rustc::ty::context::tls::enter::{{closure}}<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:966
#23 std::thread::local::LocalKey<core::cell::Cell<core::option::Option<(*const rustc::ty::context::tls::ThreadLocalGlobalCtxt, *const rustc::ty::context::tls::ThreadLocalInterners)>>>::with<core::cell::Cell<core::option::Option<(*const rustc::ty::context::tls::ThreadLocalGlobalCtxt, *const rustc::ty::context::tls::ThreadLocalInterners)>>,closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> ()
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/thread/local.rs:245
#24 rustc::ty::context::tls::enter<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> ()
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:963
#25 rustc::ty::context::tls::enter_global::{{closure}}<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:950
#26 std::thread::local::LocalKey<core::cell::Cell<fn(syntax_pos::Span, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>>>::with<core::cell::Cell<fn(syntax_pos::Span, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>>,closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/thread/local.rs:245
#27 rustc::ty::context::tls::enter_global<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:947
#28 rustc::ty::context::TyCtxt::create_and_enter<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:735
#29 rustc_driver::driver::phase_3_run_analysis_passes<closure,core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>> ()
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/driver.rs:854
#30 0x00007ffff7b25505 in rustc_driver::driver::compile_input () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/driver.rs:172
#31 0x00007ffff7b4f6a1 in rustc_driver::run_compiler () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/lib.rs:222
#32 0x00007ffff7a8ebd9 in rustc_driver::main::{{closure}} () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/lib.rs:1138
#33 rustc_driver::run::{{closure}}<closure> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/lib.rs:138
#34 rustc_driver::monitor::{{closure}}<closure> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/lib.rs:1072
#35 std::panic::{{impl}}::call_once<(),closure> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panic.rs:295
#36 std::panicking::try::do_call<std::panic::AssertUnwindSafe<closure>,()> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:356
#37 0x00007ffff77bc8ab in panic_unwind::__rust_maybe_catch_panic () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libpanic_unwind/lib.rs:97
#38 0x00007ffff7aad569 in std::panicking::try<(),std::panic::AssertUnwindSafe<closure>> ()
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:332
#39 std::panic::catch_unwind<std::panic::AssertUnwindSafe<closure>,()> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panic.rs:351
#40 std::thread::{{impl}}::spawn::{{closure}}<closure,()> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/thread/mod.rs:287
#41 alloc::boxed::{{impl}}::call_box<(),closure> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/liballoc/boxed.rs:595
#42 0x00007ffff77b04c5 in alloc::boxed::{{impl}}::call_once<(),()> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/liballoc/boxed.rs:605
#43 std::sys_common::thread::start_thread () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/sys_common/thread.rs:21
#44 std::sys::imp::thread::{{impl}}::new::thread_start () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/sys/unix/thread.rs:84
#45 0x00007fffefa6f70a in start_thread () from /lib/x86_64-linux-gnu/libpthread.so.0
#46 0x00007ffff746f0af in clone () from /lib/x86_64-linux-gnu/libc.so.6
