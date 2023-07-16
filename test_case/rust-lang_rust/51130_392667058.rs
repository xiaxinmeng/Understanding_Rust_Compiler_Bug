
#gdb rustc 
GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
Copyright (C) 2016 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from rustc...done.
(gdb) r main.rs
Starting program: /home/op/.cargo/bin/rustc main.rs
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
process 15579 is executing new program: /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[New Thread 0x7fffe85ff700 (LWP 15584)]
[New Thread 0x7fffe7fff700 (LWP 15585)]
[New Thread 0x7fffe79ff700 (LWP 15586)]
[New Thread 0x7fffe77fe700 (LWP 15587)]
[New Thread 0x7fffe71ff700 (LWP 15588)]
[Thread 0x7fffe77fe700 (LWP 15587) exited]
[New Thread 0x7fffe77fe700 (LWP 15589)]
[Thread 0x7fffe79ff700 (LWP 15586) exited]
[New Thread 0x7fffe6bff700 (LWP 15590)]
[New Thread 0x7fffe79ff700 (LWP 15591)]
[Thread 0x7fffe77fe700 (LWP 15589) exited]
[Thread 0x7fffe6bff700 (LWP 15590) exited]
[Thread 0x7fffe71ff700 (LWP 15588) exited]
[New Thread 0x7fffe6bff700 (LWP 15592)]
[Thread 0x7fffe79ff700 (LWP 15591) exited]

Thread 10 "rustc" received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0x7fffe6bff700 (LWP 15592)]
0x00007fffed67f404 in llvm::X86TargetLowering::LowerAsmOperandForConstraint(llvm::SDValue, std::string&, std::vector<llvm::SDValue, std::allocator<llvm::SDValue> >&, llvm::SelectionDAG&) const () from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
(gdb) bt
#0  0x00007fffed67f404 in llvm::X86TargetLowering::LowerAsmOperandForConstraint(llvm::SDValue, std::string&, std::vector<llvm::SDValue, std::allocator<llvm::SDValue> >&, llvm::SelectionDAG&) const () from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x00007fffee19ea2f in llvm::SelectionDAGBuilder::visitInlineAsm(llvm::ImmutableCallSite) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#2  0x00007fffee182cb8 in llvm::SelectionDAGBuilder::visit(llvm::Instruction const&) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#3  0x00007fffee204d40 in llvm::SelectionDAGISel::SelectBasicBlock(llvm::ilist_iterator<llvm::ilist_detail::node_options<llvm::Instruction, false, false, void>, false, true>, llvm::ilist_iterator<llvm::ilist_detail::node_options<llvm::Instruction, false, false, void>, false, true>, bool&) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#4  0x00007fffee203a3e in llvm::SelectionDAGISel::SelectAllBasicBlocks(llvm::Function const&) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#5  0x00007fffee200f73 in llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#6  0x00007fffed5ded51 in (anonymous namespace)::X86DAGToDAGISel::runOnMachineFunction(llvm::MachineFunction&) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#7  0x00007fffee3dc7ca in llvm::MachineFunctionPass::runOnFunction(llvm::Function&) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#8  0x00007fffeebc235b in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#9  0x00007fffeebc25c3 in llvm::FPPassManager::runOnModule(llvm::Module&) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#10 0x00007fffeebc29c5 in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#11 0x00007fffed546e2a in LLVMRustWriteOutputFile ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#12 0x00007fffed431d7a in rustc_codegen_llvm::back::write::write_output_file::hf698d82f9c58fbc6 ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#13 0x00007fffed3eb767 in rustc_codegen_llvm::back::write::codegen::_$u7b$$u7b$closure$u7d$$u7d$::h747b8ab31c2b9bcd ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#14 0x00007fffed3e556a in rustc::util::common::time_ext::h235dac2bffa5e147 ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#15 0x00007fffed434065 in rustc_codegen_llvm::back::write::codegen::h6094f17d63c01077 ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#16 0x00007fffed423a42 in std::sys_common::backtrace::__rust_begin_short_backtrace::h133fe52b0e7d804a ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#17 0x00007fffed449548 in std::panicking::try::do_call::h79f44434ae03a417 ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#18 0x00007ffff768d94a in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:105
#19 0x00007fffed3e24d1 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h287fbe6d24a16a97 ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#20 0x00007ffff768107b in _$LT$alloc..boxed..Box$LT$alloc..boxed..FnBox$LT$A$C$$u20$Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::h4d3c38b88e082d2c () at /checkout/src/liballoc/boxed.rs:648
#21 std::sys_common::thread::start_thread::haf82b92357fb7f56 () at libstd/sys_common/thread.rs:24
#22 0x00007ffff764a7f6 in std::sys::unix::thread::Thread::new::thread_start::h184d9993e322d77a () at libstd/sys/unix/thread.rs:90
#23 0x00007ffff18e27fc in start_thread (arg=0x7fffe6bff700) at pthread_create.c:465
#24 0x00007ffff7325b0f in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
(gdb) 
