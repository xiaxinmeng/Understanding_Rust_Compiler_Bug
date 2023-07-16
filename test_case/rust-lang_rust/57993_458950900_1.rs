
#0  __GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:50
#1  0x00007ffff7660895 in __GI_abort () at abort.c:79
#2  0x00007ffff7660769 in __assert_fail_base (fmt=0x7ffff77c7e90 "%s%s%s:%u: %s%sAssertion `%s' failed.\n%n", assertion=0x7fffe9d8e74f <.str.77.llvm> "i < getNumArgOperands() && \"Out of bounds!\"", 
    file=0x7fffe9576793 <.str.80.llvm> "/checkout/src/llvm-project/llvm/include/llvm/IR/InstrTypes.h", line=1136, function=<optimized out>) at assert.c:92
#3  0x00007ffff766e9f6 in __GI___assert_fail (assertion=0x7fffe9d8e74f <.str.77.llvm> "i < getNumArgOperands() && \"Out of bounds!\"", 
    file=0x7fffe9576793 <.str.80.llvm> "/checkout/src/llvm-project/llvm/include/llvm/IR/InstrTypes.h", line=1136, 
    function=0x7fffe94c3524 <__PRETTY_FUNCTION__._ZNK4llvm8CallBase13getArgOperandEj.llvm.9461709957181598743> "llvm::Value *llvm::CallBase::getArgOperand(unsigned int) const") at assert.c:101
#4  0x00007fffeadc6bfc in llvm::UpgradeIntrinsicCall(llvm::CallInst*, llvm::Function*) () from /home/mateusz/.rustup/toolchains/alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#5  0x00007fffeadcc01a in llvm::UpgradeCallsToIntrinsic(llvm::Function*) () from /home/mateusz/.rustup/toolchains/alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#6  0x00007fffee0adfa9 in LLVMRustRunFunctionPassManager () from /home/mateusz/.rustup/toolchains/alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#7  0x00007fffee04b7a1 in rustc_codegen_llvm::back::write::optimize () from /home/mateusz/.rustup/toolchains/alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#8  0x00007fffee01d92d in rustc_codegen_ssa::back::write::execute_work_item () from /home/mateusz/.rustup/toolchains/alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#9  0x00007fffedf9d509 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
   from /home/mateusz/.rustup/toolchains/alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#10 0x00007fffedfd81c6 in std::panicking::try::do_call () from /home/mateusz/.rustup/toolchains/alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#11 0x00007ffff78daeca in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#12 0x00007fffedf79ba8 in <F as alloc::boxed::FnBox<A>>::call_box () from /home/mateusz/.rustup/toolchains/alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#13 0x00007ffff78d9cce in call_once<(),()> () at /rustc/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/src/liballoc/boxed.rs:744
#14 start_thread () at src/libstd/sys_common/thread.rs:14
#15 thread_start () at src/libstd/sys/unix/thread.rs:81
#16 0x00007ffff781c58e in start_thread (arg=<optimized out>) at pthread_create.c:486
#17 0x00007ffff773b6a3 in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
