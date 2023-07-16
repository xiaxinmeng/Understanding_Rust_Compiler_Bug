
#3  0x00007f2d7175ec62 in __assert_fail () from /lib/x86_64-linux-gnu/libc.so.6
#4  0x00007f2d6baaaee6 in llvm::cast_retty<llvm::Constant, llvm::Value*>::ret_type llvm::dyn_cast<llvm::Constant, llvm::Value>(llvm::Value*) ()
   from /home/logic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_llvm-3aedc27cea629d08.so
#5  0x00007f2d6ca47371 in LLVMBuildInBoundsGEP () from /home/logic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_llvm-3aedc27cea629d08.so
#6  0x00007f2d712890fa in rustc_trans::builder::Builder::gepi ()
   from /home/logic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-8524c596fdbfd5aa.so
#7  0x00007f2d712ba6d2 in rustc_trans::meth::VirtualIndex::get_fn ()
   from /home/logic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-8524c596fdbfd5aa.so
#8  0x00007f2d712c4155 in rustc_trans::mir::block::<impl rustc_trans::mir::MirContext<'a, 'tcx>>::trans_block ()
   from /home/logic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-8524c596fdbfd5aa.so
#9  0x00007f2d712bf0d1 in rustc_trans::mir::trans_mir () from /home/logic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-8524c596fdbfd5aa.so
#10 0x00007f2d712e33dd in rustc_trans::trans_item::TransItem::define ()
   from /home/logic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-8524c596fdbfd5aa.so
#11 0x00007f2d7127f74e in rustc_trans::base::trans_crate::trans_def_task ()
   from /home/logic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-8524c596fdbfd5aa.so
#12 0x00007f2d7122b85b in rustc::dep_graph::graph::DepGraph::with_task ()
   from /home/logic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-8524c596fdbfd5aa.so
#13 0x00007f2d71279f7b in rustc_trans::base::trans_crate ()
   from /home/logic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-8524c596fdbfd5aa.so
#14 0x00007f2d71f7f97a in rustc_driver::driver::phase_4_translate_to_llvm ()
   from /home/logic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-0f4024af7b82b2df.so
#15 0x00007f2d71f3df49 in rustc_driver::driver::compile_input::{{closure}} ()
   from /home/logic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-0f4024af7b82b2df.so
#16 0x00007f2d71f6c8e6 in rustc_driver::driver::phase_3_run_analysis_passes::{{closure}} ()
   from /home/logic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-0f4024af7b82b2df.so
#17 0x00007f2d71eada73 in rustc::ty::context::TyCtxt::create_and_enter ()
   from /home/logic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-0f4024af7b82b2df.so
#18 0x00007f2d71f5a9bf in rustc_driver::driver::phase_3_run_analysis_passes ()
   from /home/logic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-0f4024af7b82b2df.so
#19 0x00007f2d71f33e80 in rustc_driver::driver::compile_input ()
   from /home/logic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-0f4024af7b82b2df.so
#20 0x00007f2d71f93d8c in rustc_driver::run_compiler () from /home/logic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-0f4024af7b82b2df.so
