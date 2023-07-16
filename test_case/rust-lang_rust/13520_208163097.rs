
#0  0x00007ffff2c71749 in llvm::X86TargetLowering::isZExtFree(llvm::SDValue, llvm::EVT) const ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#1  0x00007ffff2ee6980 in llvm::RegsForValue::getCopyToRegs(llvm::SDValue, llvm::SelectionDAG&, llvm::SDLoc, llvm::SDValue&, llvm::SDValue*, llvm::Value const*, llvm::ISD::NodeType) const ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#2  0x00007ffff2f02d85 in llvm::SelectionDAGBuilder::visitInlineAsm(llvm::ImmutableCallSite) ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#3  0x00007ffff2f1526d in llvm::SelectionDAGBuilder::visitCall(llvm::CallInst const&) ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#4  0x00007ffff2f2434a in llvm::SelectionDAGBuilder::visit(llvm::Instruction const&) ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#5  0x00007ffff2f308f1 in llvm::SelectionDAGISel::SelectBasicBlock(llvm::ilist_iterator<llvm::Instruction const>, llvm::ilist_iterator<llvm::Instruction const>, bool&) () from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#6  0x00007ffff2f37618 in llvm::SelectionDAGISel::SelectAllBasicBlocks(llvm::Function const&) ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#7  0x00007ffff2f3934a in llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#8  0x00007ffff2c69d14 in (anonymous namespace)::X86DAGToDAGISel::runOnMachineFunction(llvm::MachineFunction&) ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#9  0x00007ffff398d007 in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#10 0x00007ffff398d04b in llvm::FPPassManager::runOnModule(llvm::Module&) ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#11 0x00007ffff398ebcf in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#12 0x00007ffff2500d08 in LLVMRustWriteOutputFile (Target=0x7fffea70ab00, PMR=0x7fffeca1ac40, M=0x7fffecbd0c00, 
    path=0x7fffea455520 "/home/john/Code/git/os/fringe/target/i686-unknown-linux-gnu/debug/fpe-e1ad571a17a8b4d2.0.o", 
    FileType=llvm::TargetMachine::CGFT_ObjectFile)
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/rustllvm/PassWrapper.cpp:276
#13 0x00007ffff63fa1e4 in rustc_trans::back::write::write_output_file::hdb50287d746d6d52 ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_trans-18402db3.so
#14 0x00007ffff63fc59b in rustc_trans::back::write::optimize_and_codegen::_$u7b$$u7b$closure$u7d$$u7d$::h40c3c076fb90a91a ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_trans-18402db3.so
#15 0x00007ffff6404107 in rustc_trans::back::write::execute_work_item::h4fb548b629e1a597 ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_trans-18402db3.so
#16 0x00007ffff63fd6d2 in rustc_trans::back::write::run_passes::hb492d8bcd01a6d9e ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_trans-18402db3.so
#17 0x00007ffff7a7d5e8 in rustc_driver::driver::phase_5_run_llvm_passes::h3e9465e211407306 ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_driver-18402db3.so
#18 0x00007ffff7a40451 in rustc_driver::driver::compile_input::h650fe5b01cb8d74d ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_driver-18402db3.so
#19 0x00007ffff7a26435 in rustc_driver::run_compiler::h68d23e0e9b7b247d ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_driver-18402db3.so
#20 0x00007ffff7a23892 in std::sys_common::unwind::try::try_fn::h67fde221a73148bc ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_driver-18402db3.so
#21 0x00007ffff74ada1c in __rust_try () from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/libstd-18402db3.so
#22 0x00007ffff74ad9ae in std::sys_common::unwind::inner_try::h4e97625a08807651 ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/libstd-18402db3.so
#23 0x00007ffff7a240db in _$LT$F$u20$as$u20$std..boxed..FnBox$LT$A$GT$$GT$::call_box::hc8936fa120642c49 ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_driver-18402db3.so
#24 0x00007ffff74bbb45 in std::sys::thread::Thread::new::thread_start::h74af400293164137 ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/libstd-18402db3.so
#25 0x00007fffef10b714 in start_thread () from /nix/store/pv9sza1cf2kpawck7wbwdnhlip5h57lg-glibc-2.23/lib/libpthread.so.0
#26 0x00007ffff711cc5d in clone () from /nix/store/pv9sza1cf2kpawck7wbwdnhlip5h57lg-glibc-2.23/lib/libc.so.6#0  0x00007ffff2c71749 in llvm::X86TargetLowering::isZExtFree(llvm::SDValue, llvm::EVT) const ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#1  0x00007ffff2ee6980 in llvm::RegsForValue::getCopyToRegs(llvm::SDValue, llvm::SelectionDAG&, llvm::SDLoc, llvm::SDValue&, llvm::SDValue*, llvm::Value const*, llvm::ISD::NodeType) const ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#2  0x00007ffff2f02d85 in llvm::SelectionDAGBuilder::visitInlineAsm(llvm::ImmutableCallSite) ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#3  0x00007ffff2f1526d in llvm::SelectionDAGBuilder::visitCall(llvm::CallInst const&) ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#4  0x00007ffff2f2434a in llvm::SelectionDAGBuilder::visit(llvm::Instruction const&) ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#5  0x00007ffff2f308f1 in llvm::SelectionDAGISel::SelectBasicBlock(llvm::ilist_iterator<llvm::Instruction const>, llvm::ilist_iterator<llvm::Instruction const>, bool&) () from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#6  0x00007ffff2f37618 in llvm::SelectionDAGISel::SelectAllBasicBlocks(llvm::Function const&) ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#7  0x00007ffff2f3934a in llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#8  0x00007ffff2c69d14 in (anonymous namespace)::X86DAGToDAGISel::runOnMachineFunction(llvm::MachineFunction&) ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#9  0x00007ffff398d007 in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#10 0x00007ffff398d04b in llvm::FPPassManager::runOnModule(llvm::Module&) ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#11 0x00007ffff398ebcf in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_llvm-18402db3.so
#12 0x00007ffff2500d08 in LLVMRustWriteOutputFile (Target=0x7fffea70ab00, PMR=0x7fffeca1ac40, M=0x7fffecbd0c00, 
    path=0x7fffea455520 "/home/john/Code/git/os/fringe/target/i686-unknown-linux-gnu/debug/fpe-e1ad571a17a8b4d2.0.o", 
    FileType=llvm::TargetMachine::CGFT_ObjectFile)
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/rustllvm/PassWrapper.cpp:276
#13 0x00007ffff63fa1e4 in rustc_trans::back::write::write_output_file::hdb50287d746d6d52 ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_trans-18402db3.so
#14 0x00007ffff63fc59b in rustc_trans::back::write::optimize_and_codegen::_$u7b$$u7b$closure$u7d$$u7d$::h40c3c076fb90a91a ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_trans-18402db3.so
#15 0x00007ffff6404107 in rustc_trans::back::write::execute_work_item::h4fb548b629e1a597 ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_trans-18402db3.so
#16 0x00007ffff63fd6d2 in rustc_trans::back::write::run_passes::hb492d8bcd01a6d9e ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_trans-18402db3.so
#17 0x00007ffff7a7d5e8 in rustc_driver::driver::phase_5_run_llvm_passes::h3e9465e211407306 ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_driver-18402db3.so
#18 0x00007ffff7a40451 in rustc_driver::driver::compile_input::h650fe5b01cb8d74d ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_driver-18402db3.so
#19 0x00007ffff7a26435 in rustc_driver::run_compiler::h68d23e0e9b7b247d ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_driver-18402db3.so
#20 0x00007ffff7a23892 in std::sys_common::unwind::try::try_fn::h67fde221a73148bc ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_driver-18402db3.so
#21 0x00007ffff74ada1c in __rust_try () from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/libstd-18402db3.so
#22 0x00007ffff74ad9ae in std::sys_common::unwind::inner_try::h4e97625a08807651 ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/libstd-18402db3.so
#23 0x00007ffff7a240db in _$LT$F$u20$as$u20$std..boxed..FnBox$LT$A$GT$$GT$::call_box::hc8936fa120642c49 ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/librustc_driver-18402db3.so
#24 0x00007ffff74bbb45 in std::sys::thread::Thread::new::thread_start::h74af400293164137 ()
   from /nix/store/npk25f8bbr4n666i2kik2w593nbz6vwx-rustc-nightly-2016-04-09/lib/libstd-18402db3.so
#25 0x00007fffef10b714 in start_thread () from /nix/store/pv9sza1cf2kpawck7wbwdnhlip5h57lg-glibc-2.23/lib/libpthread.so.0
#26 0x00007ffff711cc5d in clone () from /nix/store/pv9sza1cf2kpawck7wbwdnhlip5h57lg-glibc-2.23/lib/libc.so.6
