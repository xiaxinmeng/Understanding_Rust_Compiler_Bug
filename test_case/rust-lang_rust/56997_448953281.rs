
Program terminated with signal SIGSEGV, Segmentation fault.
#0  0x000003adb70c66e4 in llvm::LLVMContextImpl::~LLVMContextImpl() ()
   from /nix/rust/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x000003adb7033f0c in LLVMContextDispose () from /nix/rust/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#2  0x000003adb4f796de in _$LT$rustc_codegen_ssa..ModuleCodegen$LT$M$GT$$GT$::into_compiled_module::ha86a62962a10a8e7 () from /nix/rust/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#3  0x000003adb4f9f860 in rustc_codegen_llvm::back::write::codegen::hf4a55f7d344cdda8 () from /nix/rust/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#4  0x000003adb4f5851d in rustc_codegen_ssa::back::write::execute_work_item::h4273347a619e859a () from /nix/rust/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#5  0x000003adb4f25aef in std::sys_common::backtrace::__rust_begin_short_backtrace::h049bba0b46693ce5 () from /nix/rust/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#6  0x000003adb4f4c926 in std::panicking::try::do_call::hfb439c39c00c2596 () from /nix/rust/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#7  0x000003adbf07562a in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:102
#8  0x000003adb4f37898 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h231c0846c490104a () from /nix/rust/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#9  0x000003adbf068ace in _$LT$alloc..boxed..Box$LT$$LP$dyn$u20$alloc..boxed..FnBox$LT$A$C$$u20$Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$RP$$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::h13f2ab96e9896fa6 () at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/boxed.rs:683
#10 std::sys_common::thread::start_thread::h89a04f637e628acf () at src/libstd/sys_common/thread.rs:24
#11 std::sys::unix::thread::Thread::new::thread_start::h29e9549935b1ee06 () at src/libstd/sys/unix/thread.rs:91
#12 0x000003adb9542494 in start_thread (arg=0x3adb05ff700) at pthread_create.c:333
#13 0x000003adbed3bacf in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:97
