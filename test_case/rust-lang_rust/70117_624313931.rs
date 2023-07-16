console
Thread 2 "rustc" received signal SIGBUS, Bus error.
[Switching to Thread 0x7fffef1ff700 (LWP 1351394)]
0x00007ffff15391bd in llvm::AttributeList::addAttributes(llvm::LLVMContext&, unsigned int, llvm::AttrBuilder const&) const ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-9-rust-1.43.0-stable.so

(gdb) bt
#0  0x00007ffff15391bd in llvm::AttributeList::addAttributes(llvm::LLVMContext&, unsigned int, llvm::AttrBuilder const&) const ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-9-rust-1.43.0-stable.so
#1  0x00007ffff1616226 in llvm::Function::addAttributes(unsigned int, llvm::AttrBuilder const&) ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-9-rust-1.43.0-stable.so
#2  0x00007ffff4ef2c90 in LLVMRustAddFunctionAttribute ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7e244282122ae6e4.so
#3  0x00007ffff4ee1813 in rustc_codegen_llvm::declare::declare_raw_fn ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7e244282122ae6e4.so
#4  0x00007ffff4dbfff2 in <rustc_codegen_llvm::context::CodegenCx as rustc_codegen_ssa::traits::misc::MiscMethods>::eh_personality ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7e244282122ae6e4.so
#5  0x00007ffff4e2ee84 in rustc_codegen_ssa::mir::codegen_mir ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7e244282122ae6e4.so
#6  0x00007ffff4df0bac in <rustc::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::define ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7e244282122ae6e4.so
#7  0x00007ffff4d3657a in rustc_codegen_llvm::base::compile_codegen_unit::module_codegen ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7e244282122ae6e4.so
#8  0x00007ffff4e1ad81 in rustc::dep_graph::graph::DepGraph::with_task ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7e244282122ae6e4.so
#9  0x00007ffff4d36144 in rustc_codegen_llvm::base::compile_codegen_unit ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7e244282122ae6e4.so
#10 0x00007ffff4e4fc89 in rustc_codegen_ssa::base::codegen_crate ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7e244282122ae6e4.so
#11 0x00007ffff4e02895 in <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7e244282122ae6e4.so
#12 0x00007ffff4cb3436 in rustc_interface::passes::start_codegen ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7e244282122ae6e4.so
#13 0x00007ffff4cc6a93 in rustc::ty::context::tls::enter_global ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7e244282122ae6e4.so
#14 0x00007ffff4bd8846 in rustc_interface::queries::Queries::ongoing_codegen ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7e244282122ae6e4.so
#15 0x00007ffff4a875e6 in rustc_interface::interface::run_compiler_in_existing_thread_pool ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7e244282122ae6e4.so
#16 0x00007ffff4a3aacc in std::sys_common::backtrace::__rust_begin_short_backtrace ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7e244282122ae6e4.so
#17 0x00007ffff43eb627 in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:86
#18 0x00007ffff4a89eb6 in core::ops::function::FnOnce::call_once{{vtable-shim}} ()
   from /home/harald/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7e244282122ae6e4.so
#19 0x00007ffff43b65bf in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once ()
    at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550/src/liballoc/boxed.rs:1017
#20 0x00007ffff43e9f2d in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once ()
    at /rustc/4fb7144ed159f94491249e86d5bbd033b5d60550/src/liballoc/boxed.rs:1017
#21 std::sys_common::thread::start_thread () at src/libstd/sys_common/thread.rs:13
#22 std::sys::unix::thread::Thread::new::thread_start () at src/libstd/sys/unix/thread.rs:80
#23 0x00007ffff4333432 in start_thread () from /lib64/libpthread.so.0
#24 0x00007ffff424f9d3 in clone () from /lib64/libc.so.6
(gdb) 

