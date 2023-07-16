
Thread 25 "LTO compiler_bu" received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0x7fffe4ffe640 (LWP 27539)]
0x00007ffff1660a5a in llvm::MachineInstr::addRegisterDead(llvm::Register, llvm::TargetRegisterInfo const*, bool) ()
   from /home/jeremy/.rustup/toolchains/snowgoons/lib/librustc_driver-c81e44a66c4bb924.so
(gdb) bt
#0  0x00007ffff1660a5a in llvm::MachineInstr::addRegisterDead(llvm::Register, llvm::TargetRegisterInfo const*, bool) ()
   from /home/jeremy/.rustup/toolchains/snowgoons/lib/librustc_driver-c81e44a66c4bb924.so
#1  0x00007ffff15981d0 in llvm::LiveIntervals::computeDeadValues(llvm::LiveInterval&, llvm::SmallVectorImpl<llvm::MachineInstr*>*) [clone .localalias] () from /home/jeremy/.rustup/toolchains/snowgoons/lib/librustc_driver-c81e44a66c4bb924.so
#2  0x00007ffff159e3aa in llvm::LiveIntervals::shrinkToUses(llvm::LiveInterval*, llvm::SmallVectorImpl<llvm::MachineInstr*>*) ()
   from /home/jeremy/.rustup/toolchains/snowgoons/lib/librustc_driver-c81e44a66c4bb924.so
#3  0x00007ffff15bbd8a in llvm::LiveRangeEdit::eliminateDeadDefs(llvm::SmallVectorImpl<llvm::MachineInstr*>&, llvm::ArrayRef<llvm::Register>, llvm::AAResults*) () from /home/jeremy/.rustup/toolchains/snowgoons/lib/librustc_driver-c81e44a66c4bb924.so
#4  0x00007ffff1561b03 in (anonymous namespace)::HoistSpillHelper::hoistAllSpills() ()
   from /home/jeremy/.rustup/toolchains/snowgoons/lib/librustc_driver-c81e44a66c4bb924.so
#5  0x00007ffff17aec62 in llvm::RegAllocBase::postOptimization() [clone .localalias] ()
   from /home/jeremy/.rustup/toolchains/snowgoons/lib/librustc_driver-c81e44a66c4bb924.so
#6  0x00007ffff17cd02e in (anonymous namespace)::RAGreedy::runOnMachineFunction(llvm::MachineFunction&) ()
   from /home/jeremy/.rustup/toolchains/snowgoons/lib/librustc_driver-c81e44a66c4bb924.so
#7  0x00007ffff16526d4 in llvm::MachineFunctionPass::runOnFunction(llvm::Function&) [clone .localalias] ()
   from /home/jeremy/.rustup/toolchains/snowgoons/lib/librustc_driver-c81e44a66c4bb924.so
#8  0x00007ffff25c8d20 in llvm::FPPassManager::runOnFunction(llvm::Function&) [clone .localalias] ()
   from /home/jeremy/.rustup/toolchains/snowgoons/lib/librustc_driver-c81e44a66c4bb924.so
#9  0x00007ffff25c8e8c in llvm::FPPassManager::runOnModule(llvm::Module&) [clone .localalias] ()
   from /home/jeremy/.rustup/toolchains/snowgoons/lib/librustc_driver-c81e44a66c4bb924.so
#10 0x00007ffff25ca8b1 in llvm::legacy::PassManagerImpl::run(llvm::Module&) [clone .localalias] ()
   from /home/jeremy/.rustup/toolchains/snowgoons/lib/librustc_driver-c81e44a66c4bb924.so
#11 0x00007ffff025c5c8 in LLVMRustWriteOutputFile () from /home/jeremy/.rustup/toolchains/snowgoons/lib/librustc_driver-c81e44a66c4bb924.so
#12 0x00007ffff00b270d in _RNvNtNtCs2Hy2UJfC2YD_18rustc_codegen_llvm4back5write17write_output_file ()
    at compiler/rustc_codegen_llvm/src/back/write.rs:72
#13 0x00007ffff00b91c7 in _RNCNvNtNtCs2Hy2UJfC2YD_18rustc_codegen_llvm4back5write7codegens0_0B7_ ()
    at compiler/rustc_codegen_llvm/src/back/write.rs:914
#14 _RINvNvNtNtCs2Hy2UJfC2YD_18rustc_codegen_llvm4back5write7codegen12with_codegenNCB2_s0_0INtNtCsaEJlGfOjSyv_4core6result6ResultuNtNtCs5LBiNHwbBUs_10rustc_span11fatal_error10FatalErrorEEB8_ () at compiler/rustc_codegen_llvm/src/back/write.rs:768
#15 _RNvNtNtCs2Hy2UJfC2YD_18rustc_codegen_llvm4back5write7codegen () at compiler/rustc_codegen_llvm/src/back/write.rs:913
#16 0x00007ffff010f382 in _RNvXs1_Cs2Hy2UJfC2YD_18rustc_codegen_llvmNtB5_18LlvmCodegenBackendNtNtNtCscKPc0T4Mv2G_17rustc_codegen_ssa6traits5write19WriteBackendMethods7codegen () at compiler/rustc_codegen_llvm/src/lib.rs:215
#17 _RINvNtNtCscKPc0T4Mv2G_17rustc_codegen_ssa4back5write24finish_intra_module_workNtCs2Hy2UJfC2YD_18rustc_codegen_llvm18LlvmCodegenBackendEB1g_ () at /home/jeremy/apechem/rust-avr-nightly-builder/build/rust/compiler/rustc_codegen_ssa/src/back/write.rs:911
#18 0x00007ffff010e088 in _RINvNtNtCscKPc0T4Mv2G_17rustc_codegen_ssa4back5write21execute_lto_work_itemNtCs2Hy2UJfC2YD_18rustc_codegen_llvm18LlvmCodegenBackendEB1d_ () at /home/jeremy/apechem/rust-avr-nightly-builder/build/rust/compiler/rustc_codegen_ssa/src/back/write.rs:897
--Type <RET> for more, q to quit, c to continue without paging--
#19 _RINvNtNtCscKPc0T4Mv2G_17rustc_codegen_ssa4back5write17execute_work_itemNtCs2Hy2UJfC2YD_18rustc_codegen_llvm18LlvmCodegenBackendEB19_ ()
    at /home/jeremy/apechem/rust-avr-nightly-builder/build/rust/compiler/rustc_codegen_ssa/src/back/write.rs:749
#20 0x00007ffff01ab6c5 in _RNCINvNtNtCscKPc0T4Mv2G_17rustc_codegen_ssa4back5write10spawn_workNtCs2Hy2UJfC2YD_18rustc_codegen_llvm18LlvmCodegenBackendE0B14_ () at /home/jeremy/apechem/rust-avr-nightly-builder/build/rust/compiler/rustc_codegen_ssa/src/back/write.rs:1665
#21 _RNCINvXs0_Cs2Hy2UJfC2YD_18rustc_codegen_llvmNtB8_18LlvmCodegenBackendNtNtNtCscKPc0T4Mv2G_17rustc_codegen_ssa6traits7backend19ExtraBackendMethods18spawn_named_threadNCINvNtNtB1b_4back5write10spawn_workBG_E0uE0B8_ () at compiler/rustc_codegen_llvm/src/lib.rs:157
#22 _RINvNtNtCsiRCMJYjrQF4_3std10sys_common9backtrace28___rust_begin_short_backtraceNCINvXs0_Cs2Hy2UJfC2YD_18rustc_codegen_llvmNtB1o_18LlvmCodegenBackendNtNtNtCscKPc0T4Mv2G_17rustc_codegen_ssa6traits7backend19ExtraBackendMethods18spawn_named_threadNCINvNtNtB2s_4back5write10spawn_workB1W_E0uE0uEB1o_ () at /home/jeremy/apechem/rust-avr-nightly-builder/build/rust/library/std/src/sys_common/backtrace.rs:123
#23 0x00007ffff0228cd7 in _RNCNCINvMNtCsiRCMJYjrQF4_3std6threadNtB7_7Builder15spawn_uncheckedNCINvXs0_Cs2Hy2UJfC2YD_18rustc_codegen_llvmNtB1b_18LlvmCodegenBackendNtNtNtCscKPc0T4Mv2G_17rustc_codegen_ssa6traits7backend19ExtraBackendMethods18spawn_named_threadNCINvNtNtB2f_4back5write10spawn_workB1J_E0uE0uEs_00B1b_ () at /home/jeremy/apechem/rust-avr-nightly-builder/build/rust/library/std/src/thread/mod.rs:477
#24 _RNvXsl_NtNtCsaEJlGfOjSyv_4core5panic11unwind_safeINtB5_16AssertUnwindSafeNCNCINvMNtCsiRCMJYjrQF4_3std6threadNtB1h_7Builder15spawn_uncheckedNCINvXs0_Cs2Hy2UJfC2YD_18rustc_codegen_llvmNtB2m_18LlvmCodegenBackendNtNtNtCscKPc0T4Mv2G_17rustc_codegen_ssa6traits7backend19ExtraBackendMethods18spawn_named_threadNCINvNtNtB3q_4back5write10spawn_workB2U_E0uE0uEs_00EINtNtNtB9_3ops8function6FnOnceuE9call_onceB2m_ ()
    at /home/jeremy/apechem/rust-avr-nightly-builder/build/rust/library/core/src/panic/unwind_safe.rs:271
#25 _RINvNvNtCsiRCMJYjrQF4_3std9panicking3try7do_callINtNtNtCsaEJlGfOjSyv_4core5panic11unwind_safe16AssertUnwindSafeNCNCINvMNtB6_6threadNtB1T_7Builder15spawn_uncheckedNCINvXs0_Cs2Hy2UJfC2YD_18rustc_codegen_llvmNtB2J_18LlvmCodegenBackendNtNtNtCscKPc0T4Mv2G_17rustc_codegen_ssa6traits7backend19ExtraBackendMethods18spawn_named_threadNCINvNtNtB3N_4back5write10spawn_workB3h_E0uE0uEs_00EuEB2J_ ()
    at /home/jeremy/apechem/rust-avr-nightly-builder/build/rust/library/std/src/panicking.rs:406
#26 _RINvNtCsiRCMJYjrQF4_3std9panicking3tryuINtNtNtCsaEJlGfOjSyv_4core5panic11unwind_safe16AssertUnwindSafeNCNCINvMNtB4_6threadNtB1K_7Builder15spawn_uncheckedNCINvXs0_Cs2Hy2UJfC2YD_18rustc_codegen_llvmNtB2A_18LlvmCodegenBackendNtNtNtCscKPc0T4Mv2G_17rustc_codegen_ssa6traits7backend19ExtraBackendMethods18spawn_named_threadNCINvNtNtB3E_4back5write10spawn_workB38_E0uE0uEs_00EEB2A_ ()
    at /home/jeremy/apechem/rust-avr-nightly-builder/build/rust/library/std/src/panicking.rs:370
#27 _RINvNtCsiRCMJYjrQF4_3std5panic12catch_unwindINtNtNtCsaEJlGfOjSyv_4core5panic11unwind_safe16AssertUnwindSafeNCNCINvMNtB4_6threadNtB1P_7Builder15spawn_uncheckedNCINvXs0_Cs2Hy2UJfC2YD_18rustc_codegen_llvmNtB2F_18LlvmCodegenBackendNtNtNtCscKPc0T4Mv2G_17rustc_codegen_ssa6traits7backend19ExtraBackendMethods18spawn_named_threadNCINvNtNtB3J_4back5write10spawn_workB3d_E0uE0uEs_00EuEB2F_ ()
    at /home/jeremy/apechem/rust-avr-nightly-builder/build/rust/library/std/src/panic.rs:133
#28 _RNCINvMNtCsiRCMJYjrQF4_3std6threadNtB5_7Builder15spawn_uncheckedNCINvXs0_Cs2Hy2UJfC2YD_18rustc_codegen_llvmNtB19_18LlvmCodegenBackendNtNtNtCscKPc0T4Mv2G_17rustc_codegen_ssa6traits7backend19ExtraBackendMethods18spawn_named_threadNCINvNtNtB2d_4back5write10spawn_workB1H_E0uE0uEs_0B19_
    () at /home/jeremy/apechem/rust-avr-nightly-builder/build/rust/library/std/src/thread/mod.rs:476
#29 _RNSNvYNCINvMNtCsiRCMJYjrQF4_3std6threadNtBa_7Builder15spawn_uncheckedNCINvXs0_Cs2Hy2UJfC2YD_18rustc_codegen_llvmNtB1e_18LlvmCodegenBackendNtNtNtCscKPc0T4Mv2G_17rustc_codegen_ssa6traits7backend19ExtraBackendMethods18spawn_named_threadNCINvNtNtB2i_4back5write10spawn_workB1M_E0uE0uEs_0INtNtNtCsaEJlGfOjSyv_4core3ops8function6FnOnceuE9call_once6vtableB1e_ ()
    at /home/jeremy/apechem/rust-avr-nightly-builder/build/rust/library/core/src/ops/function.rs:227
#30 0x00007fffeefd67e3 in <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once ()
--Type <RET> for more, q to quit, c to continue without paging--
    at /home/jeremy/apechem/rust-avr-nightly-builder/build/rust/library/alloc/src/boxed.rs:1854
#31 <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once ()
    at /home/jeremy/apechem/rust-avr-nightly-builder/build/rust/library/alloc/src/boxed.rs:1854
#32 std::sys::unix::thread::Thread::new::thread_start () at library/std/src/sys/unix/thread.rs:108
#33 0x00007fffeeedbd80 in start_thread (arg=0x7fffe4ffe640) at pthread_create.c:481
#34 0x00007fffeee06b6f in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
