
(lldb) thread backtrace 1
* thread #1, stop reason = signal SIGSTOP
  * frame #0: 0x00007fff57f1c9de libsystem_kernel.dylib`__ulock_wait + 10
    frame #1: 0x00007fff57fd76de libsystem_pthread.dylib`_pthread_join + 358
    frame #2: 0x000000010f81f450 libstd-8d84a71a2f773b92.dylib`std::sys::unix::thread::Thread::join::h8564f5ef97632f26 + 16
    frame #3: 0x000000010b5873d2 librustc_driver-5e3149cfc076ab77.dylib`std::thread::JoinHandle$LT$T$GT$::join::h26734723f6318af8 + 66
    frame #4: 0x000000010b586276 librustc_driver-5e3149cfc076ab77.dylib`rustc_interface::util::spawn_thread_pool::h02c27e2fe510dc4b + 790
    frame #5: 0x000000010b5df84b librustc_driver-5e3149cfc076ab77.dylib`rustc_driver::run_compiler::hd6ffdb208b57cc57 + 5707
    frame #6: 0x000000010b58b661 librustc_driver-5e3149cfc076ab77.dylib`std::panicking::try::do_call::h7567d6ca86efb955 (.llvm.18415579661075415780) + 129
    frame #7: 0x000000010f8205ef libstd-8d84a71a2f773b92.dylib`__rust_maybe_catch_panic + 31
    frame #8: 0x000000010b5e5bd2 librustc_driver-5e3149cfc076ab77.dylib`rustc_driver::report_ices_to_stderr_if_any::h37fc09a37e4811ae + 66
    frame #9: 0x000000010b5e670e librustc_driver-5e3149cfc076ab77.dylib`rustc_driver::main::h31dcaa2e76b91ff3 + 14
    frame #10: 0x000000010b2ef566 rustc`std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h780d4c2ef30fb988 + 6
    frame #11: 0x000000010f810538 libstd-8d84a71a2f773b92.dylib`std::panicking::try::do_call::h5ee2775b0f32dc74 + 24
    frame #12: 0x000000010f8205ef libstd-8d84a71a2f773b92.dylib`__rust_maybe_catch_panic + 31
    frame #13: 0x000000010f81101e libstd-8d84a71a2f773b92.dylib`std::rt::lang_start_internal::h015bd3cd7004bd85 + 542
    frame #14: 0x000000010b2ef559 rustc`main + 41
    frame #15: 0x000000010b2ef514 rustc`start + 52
(lldb) thread backtrace 2
  thread #2, stop reason = signal SIGSTOP
    frame #0: 0x000000010dc5f7b0 librustc-bc7cc2f4f3a6e74f.dylib`rustc::ty::structural_impls::_$LT$impl$u20$rustc..ty..fold..TypeFoldable$u20$for$u20$$RF$rustc..ty..TyS$GT$::super_fold_with::hf3ecf83afe9cfa49
    frame #1: 0x000000010dce1421 librustc-bc7cc2f4f3a6e74f.dylib`rustc::ty::erase_regions::erase_regions_ty::h9420de1b0ed69b01 (.llvm.13950983945961193090) + 33
    frame #2: 0x000000010dc3973e librustc-bc7cc2f4f3a6e74f.dylib`rustc::dep_graph::graph::DepGraph::with_anon_task::h023efe6e23caccbb + 462
    frame #3: 0x000000010de900e9 librustc-bc7cc2f4f3a6e74f.dylib`rustc::ty::query::plumbing::_$LT$impl$u20$rustc..ty..context..TyCtxt$GT$::get_query::h5ab75c6b7dfa7cf8 + 2169
    frame #4: 0x00000001138725d5 librustc_codegen_llvm-llvm.dylib`_$LT$smallvec..SmallVec$LT$A$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$$LT$A$u20$as$u20$smallvec..Array$GT$..Item$GT$$GT$::from_iter::hb426e11935311c28 + 149
    frame #5: 0x0000000113804428 librustc_codegen_llvm-llvm.dylib`rustc::ty::fold::TypeFoldable::fold_with::h93155c485749e643 + 72
    frame #6: 0x000000011392f88c librustc_codegen_llvm-llvm.dylib`rustc::traits::query::normalize_erasing_regions::_$LT$impl$u20$rustc..ty..context..TyCtxt$GT$::normalize_erasing_late_bound_regions::h93cc59554af14e11 + 812
    frame #7: 0x0000000113841716 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::declare::_$LT$impl$u20$rustc_codegen_ssa..traits..declare..DeclareMethods$u20$for$u20$rustc_codegen_llvm..context..CodegenCx$GT$::declare_fn::h9d5ac91b83c9be90 + 86
    frame #8: 0x0000000113841de0 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::mono_item::_$LT$impl$u20$rustc_codegen_ssa..traits..declare..PreDefineMethods$u20$for$u20$rustc_codegen_llvm..context..CodegenCx$GT$::predefine_fn::h15caf8450555940e + 192
    frame #9: 0x000000011397fc0e librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::base::compile_codegen_unit::module_codegen::h22369ca0d54174b6 + 2462
    frame #10: 0x00000001138c23a4 librustc_codegen_llvm-llvm.dylib`rustc::dep_graph::graph::DepGraph::with_task::h24f1fc8005d9a75d + 468
    frame #11: 0x000000011397f133 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::base::compile_codegen_unit::h4cfec87dbb6367b3 + 227
    frame #12: 0x000000011384ed1e librustc_codegen_llvm-llvm.dylib`rustc_codegen_ssa::base::codegen_crate::h40331f24291424ce + 3774
    frame #13: 0x00000001138901a1 librustc_codegen_llvm-llvm.dylib`_$LT$rustc_codegen_llvm..LlvmCodegenBackend$u20$as$u20$rustc_codegen_utils..codegen_backend..CodegenBackend$GT$::codegen_crate::h1bc9127970393180 + 113
    frame #14: 0x000000010b7e406e librustc_interface-6ef349071c7b09f2.dylib`rustc::util::common::time::hceec4ddb46fc4c3a + 158
    frame #15: 0x000000010b73b7d5 librustc_interface-6ef349071c7b09f2.dylib`rustc_interface::passes::start_codegen::h8a57d37e47e58868 + 421
    frame #16: 0x000000010b7c4ec1 librustc_interface-6ef349071c7b09f2.dylib`rustc::ty::context::tls::enter_global::h8be9ffe8488f2b17 + 609
    frame #17: 0x000000010b73bee1 librustc_interface-6ef349071c7b09f2.dylib`rustc_interface::passes::BoxedGlobalCtxt::access::_$u7b$$u7b$closure$u7d$$u7d$::hcf82da4e158c37b8 + 129
    frame #18: 0x000000010b787b65 librustc_interface-6ef349071c7b09f2.dylib`rustc_interface::passes::create_global_ctxt::_$u7b$$u7b$closure$u7d$$u7d$::h903c09c70c254566 + 117
    frame #19: 0x000000010b73ab03 librustc_interface-6ef349071c7b09f2.dylib`rustc_interface::passes::BoxedGlobalCtxt::enter::hc5c007900b114061 + 147
    frame #20: 0x000000010b7cd0a5 librustc_interface-6ef349071c7b09f2.dylib`rustc_interface::queries::Query$LT$T$GT$::compute::h82fd0df60b59f057 + 437
    frame #21: 0x000000010b7b816c librustc_interface-6ef349071c7b09f2.dylib`rustc_interface::queries::_$LT$impl$u20$rustc_interface..interface..Compiler$GT$::ongoing_codegen::he9ecb6d5eb7f0dad + 28
    frame #22: 0x000000010b5ab859 librustc_driver-5e3149cfc076ab77.dylib`rustc_interface::interface::run_compiler_in_existing_thread_pool::hed5392ff52302c1d + 4009
    frame #23: 0x000000010b604a57 librustc_driver-5e3149cfc076ab77.dylib`std::thread::local::LocalKey$LT$T$GT$::with::h609091f75fe87ad8 + 279
    frame #24: 0x000000010b5d3096 librustc_driver-5e3149cfc076ab77.dylib`scoped_tls::ScopedKey$LT$T$GT$::set::h2a0845a5b4b13b2e + 518
    frame #25: 0x000000010b600703 librustc_driver-5e3149cfc076ab77.dylib`syntax::with_globals::h90417e911e4c0f10 + 83
    frame #26: 0x000000010b56f559 librustc_driver-5e3149cfc076ab77.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h4f8be2a6d6d269f8 + 569
    frame #27: 0x000000010f8205ef libstd-8d84a71a2f773b92.dylib`__rust_maybe_catch_panic + 31
    frame #28: 0x000000010b588147 librustc_driver-5e3149cfc076ab77.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h69185ebc15a210c1 + 119
    frame #29: 0x000000010f7f255e libstd-8d84a71a2f773b92.dylib`_$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hf374624fb08997e7 + 62
    frame #30: 0x000000010f81f3ae libstd-8d84a71a2f773b92.dylib`std::sys::unix::thread::Thread::new::thread_start::hfe8d238f75e9e68d + 142
    frame #31: 0x00007fff57fd32eb libsystem_pthread.dylib`_pthread_body + 126
    frame #32: 0x00007fff57fd6249 libsystem_pthread.dylib`_pthread_start + 66
    frame #33: 0x00007fff57fd240d libsystem_pthread.dylib`thread_start + 13
(lldb) thread backtrace 3
  thread #3, stop reason = signal SIGSTOP
    frame #0: 0x00007fff57f2236e libsystem_kernel.dylib`poll + 10
    frame #1: 0x000000010f6389f4 librustc_data_structures-77b6f90c54398a8c.dylib`jobserver::imp::Client::acquire::h623d23d7b92a4e22 + 68
    frame #2: 0x000000010f637560 librustc_data_structures-77b6f90c54398a8c.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h99d3814a1806ad07 + 176
    frame #3: 0x000000010f636140 librustc_data_structures-77b6f90c54398a8c.dylib`std::panicking::try::do_call::h86f8d965769e44bd (.llvm.13933957308689913887) + 80
    frame #4: 0x000000010f8205ef libstd-8d84a71a2f773b92.dylib`__rust_maybe_catch_panic + 31
    frame #5: 0x000000010f636717 librustc_data_structures-77b6f90c54398a8c.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h570edcc8a145534f + 167
    frame #6: 0x000000010f7f255e libstd-8d84a71a2f773b92.dylib`_$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hf374624fb08997e7 + 62
    frame #7: 0x000000010f81f3ae libstd-8d84a71a2f773b92.dylib`std::sys::unix::thread::Thread::new::thread_start::hfe8d238f75e9e68d + 142
    frame #8: 0x00007fff57fd32eb libsystem_pthread.dylib`_pthread_body + 126
    frame #9: 0x00007fff57fd6249 libsystem_pthread.dylib`_pthread_start + 66
    frame #10: 0x00007fff57fd240d libsystem_pthread.dylib`thread_start + 13
(lldb) thread backtrace 4
  thread #4, stop reason = signal SIGSTOP
    frame #0: 0x00007fff57f1d86a libsystem_kernel.dylib`__psynch_cvwait + 10
    frame #1: 0x00007fff57fd656e libsystem_pthread.dylib`_pthread_cond_wait + 722
    frame #2: 0x000000010f7f2f22 libstd-8d84a71a2f773b92.dylib`std::thread::park::hb4616f6052b47640 + 242
    frame #3: 0x000000010f809d61 libstd-8d84a71a2f773b92.dylib`std::sync::mpsc::blocking::WaitToken::wait::h0bb84fd816c7f43b + 49
    frame #4: 0x00000001138a8289 librustc_codegen_llvm-llvm.dylib`std::sync::mpsc::shared::Packet$LT$T$GT$::recv::hbbfcac3ba6afaf1a + 585
    frame #5: 0x0000000113855b91 librustc_codegen_llvm-llvm.dylib`std::sync::mpsc::Receiver$LT$T$GT$::recv::h57f739759d18fbde + 321
    frame #6: 0x0000000113973185 librustc_codegen_llvm-llvm.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::hef0265ead93bf37d + 4229
    frame #7: 0x0000000113855dcc librustc_codegen_llvm-llvm.dylib`std::panicking::try::do_call::h3485533b8b7a27bc (.llvm.371732380873419076) + 60
    frame #8: 0x000000010f8205ef libstd-8d84a71a2f773b92.dylib`__rust_maybe_catch_panic + 31
    frame #9: 0x00000001137ffbe6 librustc_codegen_llvm-llvm.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h0b553514d995afe7 + 134
    frame #10: 0x000000010f7f255e libstd-8d84a71a2f773b92.dylib`_$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hf374624fb08997e7 + 62
    frame #11: 0x000000010f81f3ae libstd-8d84a71a2f773b92.dylib`std::sys::unix::thread::Thread::new::thread_start::hfe8d238f75e9e68d + 142
    frame #12: 0x00007fff57fd32eb libsystem_pthread.dylib`_pthread_body + 126
    frame #13: 0x00007fff57fd6249 libsystem_pthread.dylib`_pthread_start + 66
    frame #14: 0x00007fff57fd240d libsystem_pthread.dylib`thread_start + 13
(lldb) thread backtrace 5
  thread #5, stop reason = signal SIGSTOP
    frame #0: 0x0000000114bab33f librustc_codegen_llvm-llvm.dylib`llvm::MachineRegisterInfo::addRegOperandToUseList(llvm::MachineOperand*) + 63
    frame #1: 0x0000000114b3eeab librustc_codegen_llvm-llvm.dylib`llvm::MachineInstr::AddRegOperandsToUseLists(llvm::MachineRegisterInfo&) + 75
    frame #2: 0x0000000114af4c71 librustc_codegen_llvm-llvm.dylib`llvm::ilist_traits<llvm::MachineInstr>::addNodeToList(llvm::MachineInstr*) + 33
    frame #3: 0x0000000114835812 librustc_codegen_llvm-llvm.dylib`llvm::InstrEmitter::EmitMachineNode(llvm::SDNode*, bool, bool, llvm::DenseMap<llvm::SDValue, unsigned int, llvm::DenseMapInfo<llvm::SDValue>, llvm::detail::DenseMapPair<llvm::SDValue, unsigned int> >&) + 1266
    frame #4: 0x00000001148c8b89 librustc_codegen_llvm-llvm.dylib`llvm::ScheduleDAGSDNodes::EmitSchedule(llvm::MachineInstrBundleIterator<llvm::MachineInstr, false>&) + 1401
    frame #5: 0x0000000114967630 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGISel::CodeGenAndEmitDAG() + 1760
    frame #6: 0x000000011496627d librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGISel::SelectAllBasicBlocks(llvm::Function const&) + 9453
    frame #7: 0x0000000114963376 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) + 1782
    frame #8: 0x0000000114b3b3a9 librustc_codegen_llvm-llvm.dylib`llvm::MachineFunctionPass::runOnFunction(llvm::Function&) + 329
    frame #9: 0x0000000115457eb9 librustc_codegen_llvm-llvm.dylib`llvm::FPPassManager::runOnFunction(llvm::Function&) + 1177
    frame #10: 0x0000000115458153 librustc_codegen_llvm-llvm.dylib`llvm::FPPassManager::runOnModule(llvm::Module&) + 67
    frame #11: 0x0000000115458529 librustc_codegen_llvm-llvm.dylib`llvm::legacy::PassManagerImpl::run(llvm::Module&) + 761
    frame #12: 0x00000001139840b8 librustc_codegen_llvm-llvm.dylib`LLVMRustWriteOutputFile + 568
    frame #13: 0x00000001138cfd36 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::write_output_file::h46c8b7792ec6c3bf (.llvm.14679628501088474520) + 86
    frame #14: 0x0000000113946cb7 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::codegen::_$u7b$$u7b$closure$u7d$$u7d$::h0a3d470f451a3cca (.llvm.12709569147983681104) + 983
    frame #15: 0x000000011393ccb0 librustc_codegen_llvm-llvm.dylib`rustc::util::common::time_ext::he76c8601a2e54611 + 80
    frame #16: 0x00000001138d2a0a librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::codegen::h1854c102d66890de + 2890
    frame #17: 0x0000000113846898 librustc_codegen_llvm-llvm.dylib`rustc_codegen_ssa::back::write::execute_work_item::h099f6feee3dc5c4f + 4744
    frame #18: 0x0000000113971f75 librustc_codegen_llvm-llvm.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::heb8aade351110442 + 181
    frame #19: 0x0000000113855e1b librustc_codegen_llvm-llvm.dylib`std::panicking::try::do_call::hc37c119df709ca26 (.llvm.371732380873419076) + 43
    frame #20: 0x000000010f8205ef libstd-8d84a71a2f773b92.dylib`__rust_maybe_catch_panic + 31
    frame #21: 0x00000001137ffdb6 librustc_codegen_llvm-llvm.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h89669d2534a37616 + 134
    frame #22: 0x000000010f7f255e libstd-8d84a71a2f773b92.dylib`_$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hf374624fb08997e7 + 62
    frame #23: 0x000000010f81f3ae libstd-8d84a71a2f773b92.dylib`std::sys::unix::thread::Thread::new::thread_start::hfe8d238f75e9e68d + 142
    frame #24: 0x00007fff57fd32eb libsystem_pthread.dylib`_pthread_body + 126
    frame #25: 0x00007fff57fd6249 libsystem_pthread.dylib`_pthread_start + 66
    frame #26: 0x00007fff57fd240d libsystem_pthread.dylib`thread_start + 13
(lldb) thread backtrace 6
  thread #6, stop reason = signal SIGSTOP
    frame #0: 0x00000001156009f6 librustc_codegen_llvm-llvm.dylib`(anonymous namespace)::WasmObjectWriter::writeObject(llvm::MCAssembler&, llvm::MCAsmLayout const&) + 20838
    frame #1: 0x00000001155bb00e librustc_codegen_llvm-llvm.dylib`llvm::MCAssembler::Finish() + 62
    frame #2: 0x00000001149b4b5b librustc_codegen_llvm-llvm.dylib`llvm::AsmPrinter::doFinalization(llvm::Module&) + 6395
    frame #3: 0x0000000115458213 librustc_codegen_llvm-llvm.dylib`llvm::FPPassManager::doFinalization(llvm::Module&) + 51
    frame #4: 0x0000000115458693 librustc_codegen_llvm-llvm.dylib`llvm::legacy::PassManagerImpl::run(llvm::Module&) + 1123
    frame #5: 0x00000001139840b8 librustc_codegen_llvm-llvm.dylib`LLVMRustWriteOutputFile + 568
    frame #6: 0x00000001138cfd36 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::write_output_file::h46c8b7792ec6c3bf (.llvm.14679628501088474520) + 86
    frame #7: 0x0000000113946cb7 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::codegen::_$u7b$$u7b$closure$u7d$$u7d$::h0a3d470f451a3cca (.llvm.12709569147983681104) + 983
    frame #8: 0x000000011393ccb0 librustc_codegen_llvm-llvm.dylib`rustc::util::common::time_ext::he76c8601a2e54611 + 80
    frame #9: 0x00000001138d2a0a librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::codegen::h1854c102d66890de + 2890
    frame #10: 0x0000000113846898 librustc_codegen_llvm-llvm.dylib`rustc_codegen_ssa::back::write::execute_work_item::h099f6feee3dc5c4f + 4744
    frame #11: 0x0000000113971f75 librustc_codegen_llvm-llvm.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::heb8aade351110442 + 181
    frame #12: 0x0000000113855e1b librustc_codegen_llvm-llvm.dylib`std::panicking::try::do_call::hc37c119df709ca26 (.llvm.371732380873419076) + 43
    frame #13: 0x000000010f8205ef libstd-8d84a71a2f773b92.dylib`__rust_maybe_catch_panic + 31
    frame #14: 0x00000001137ffdb6 librustc_codegen_llvm-llvm.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h89669d2534a37616 + 134
    frame #15: 0x000000010f7f255e libstd-8d84a71a2f773b92.dylib`_$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hf374624fb08997e7 + 62
    frame #16: 0x000000010f81f3ae libstd-8d84a71a2f773b92.dylib`std::sys::unix::thread::Thread::new::thread_start::hfe8d238f75e9e68d + 142
    frame #17: 0x00007fff57fd32eb libsystem_pthread.dylib`_pthread_body + 126
    frame #18: 0x00007fff57fd6249 libsystem_pthread.dylib`_pthread_start + 66
    frame #19: 0x00007fff57fd240d libsystem_pthread.dylib`thread_start + 13
(lldb) thread backtrace 7
  thread #7, stop reason = signal SIGSTOP
    frame #0: 0x00000001148bf331 librustc_codegen_llvm-llvm.dylib`(anonymous namespace)::ScheduleDAGRRList::PickNodeToScheduleBottomUp()::$_0::operator()() const + 33
    frame #1: 0x00000001148bbb45 librustc_codegen_llvm-llvm.dylib`(anonymous namespace)::ScheduleDAGRRList::Schedule() + 965
    frame #2: 0x000000011496759d librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGISel::CodeGenAndEmitDAG() + 1613
    frame #3: 0x000000011496627d librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGISel::SelectAllBasicBlocks(llvm::Function const&) + 9453
    frame #4: 0x0000000114963376 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) + 1782
    frame #5: 0x0000000114b3b3a9 librustc_codegen_llvm-llvm.dylib`llvm::MachineFunctionPass::runOnFunction(llvm::Function&) + 329
    frame #6: 0x0000000115457eb9 librustc_codegen_llvm-llvm.dylib`llvm::FPPassManager::runOnFunction(llvm::Function&) + 1177
    frame #7: 0x0000000115458153 librustc_codegen_llvm-llvm.dylib`llvm::FPPassManager::runOnModule(llvm::Module&) + 67
    frame #8: 0x0000000115458529 librustc_codegen_llvm-llvm.dylib`llvm::legacy::PassManagerImpl::run(llvm::Module&) + 761
    frame #9: 0x00000001139840b8 librustc_codegen_llvm-llvm.dylib`LLVMRustWriteOutputFile + 568
    frame #10: 0x00000001138cfd36 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::write_output_file::h46c8b7792ec6c3bf (.llvm.14679628501088474520) + 86
    frame #11: 0x0000000113946cb7 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::codegen::_$u7b$$u7b$closure$u7d$$u7d$::h0a3d470f451a3cca (.llvm.12709569147983681104) + 983
    frame #12: 0x000000011393ccb0 librustc_codegen_llvm-llvm.dylib`rustc::util::common::time_ext::he76c8601a2e54611 + 80
    frame #13: 0x00000001138d2a0a librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::codegen::h1854c102d66890de + 2890
    frame #14: 0x0000000113846898 librustc_codegen_llvm-llvm.dylib`rustc_codegen_ssa::back::write::execute_work_item::h099f6feee3dc5c4f + 4744
    frame #15: 0x0000000113971f75 librustc_codegen_llvm-llvm.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::heb8aade351110442 + 181
    frame #16: 0x0000000113855e1b librustc_codegen_llvm-llvm.dylib`std::panicking::try::do_call::hc37c119df709ca26 (.llvm.371732380873419076) + 43
    frame #17: 0x000000010f8205ef libstd-8d84a71a2f773b92.dylib`__rust_maybe_catch_panic + 31
    frame #18: 0x00000001137ffdb6 librustc_codegen_llvm-llvm.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h89669d2534a37616 + 134
    frame #19: 0x000000010f7f255e libstd-8d84a71a2f773b92.dylib`_$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hf374624fb08997e7 + 62
    frame #20: 0x000000010f81f3ae libstd-8d84a71a2f773b92.dylib`std::sys::unix::thread::Thread::new::thread_start::hfe8d238f75e9e68d + 142
    frame #21: 0x00007fff57fd32eb libsystem_pthread.dylib`_pthread_body + 126
    frame #22: 0x00007fff57fd6249 libsystem_pthread.dylib`_pthread_start + 66
    frame #23: 0x00007fff57fd240d libsystem_pthread.dylib`thread_start + 13
(lldb) thread backtrace 8
  thread #8, stop reason = signal SIGSTOP
    frame #0: 0x000000011492ddc5 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAG::getRegister(unsigned int, llvm::EVT) + 325
    frame #1: 0x00000001148ce7d5 librustc_codegen_llvm-llvm.dylib`llvm::RegsForValue::getCopyFromRegs(llvm::SelectionDAG&, llvm::FunctionLoweringInfo&, llvm::SDLoc const&, llvm::SDValue&, llvm::SDValue*, llvm::Value const*) const + 1509
    frame #2: 0x00000001148e5b52 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGBuilder::getValueImpl(llvm::Value const*) + 1442
    frame #3: 0x00000001148e54c7 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGBuilder::getValue(llvm::Value const*) + 231
    frame #4: 0x00000001148f2f62 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGBuilder::LowerCallTo(llvm::ImmutableCallSite, llvm::SDValue, bool, llvm::BasicBlock const*) + 450
    frame #5: 0x00000001148df3cf librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGBuilder::visitCall(llvm::CallInst const&) + 415
    frame #6: 0x00000001148d3fb9 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGBuilder::visit(llvm::Instruction const&) + 105
    frame #7: 0x0000000114966eb0 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGISel::SelectBasicBlock(llvm::ilist_iterator<llvm::ilist_detail::node_options<llvm::Instruction, false, false, void>, false, true>, llvm::ilist_iterator<llvm::ilist_detail::node_options<llvm::Instruction, false, false, void>, false, true>, bool&) + 384
    frame #8: 0x0000000114965d92 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGISel::SelectAllBasicBlocks(llvm::Function const&) + 8194
    frame #9: 0x0000000114963376 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) + 1782
    frame #10: 0x0000000114b3b3a9 librustc_codegen_llvm-llvm.dylib`llvm::MachineFunctionPass::runOnFunction(llvm::Function&) + 329
    frame #11: 0x0000000115457eb9 librustc_codegen_llvm-llvm.dylib`llvm::FPPassManager::runOnFunction(llvm::Function&) + 1177
    frame #12: 0x0000000115458153 librustc_codegen_llvm-llvm.dylib`llvm::FPPassManager::runOnModule(llvm::Module&) + 67
    frame #13: 0x0000000115458529 librustc_codegen_llvm-llvm.dylib`llvm::legacy::PassManagerImpl::run(llvm::Module&) + 761
    frame #14: 0x00000001139840b8 librustc_codegen_llvm-llvm.dylib`LLVMRustWriteOutputFile + 568
    frame #15: 0x00000001138cfd36 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::write_output_file::h46c8b7792ec6c3bf (.llvm.14679628501088474520) + 86
    frame #16: 0x0000000113946cb7 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::codegen::_$u7b$$u7b$closure$u7d$$u7d$::h0a3d470f451a3cca (.llvm.12709569147983681104) + 983
    frame #17: 0x000000011393ccb0 librustc_codegen_llvm-llvm.dylib`rustc::util::common::time_ext::he76c8601a2e54611 + 80
    frame #18: 0x00000001138d2a0a librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::codegen::h1854c102d66890de + 2890
    frame #19: 0x0000000113846898 librustc_codegen_llvm-llvm.dylib`rustc_codegen_ssa::back::write::execute_work_item::h099f6feee3dc5c4f + 4744
    frame #20: 0x0000000113971f75 librustc_codegen_llvm-llvm.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::heb8aade351110442 + 181
    frame #21: 0x0000000113855e1b librustc_codegen_llvm-llvm.dylib`std::panicking::try::do_call::hc37c119df709ca26 (.llvm.371732380873419076) + 43
    frame #22: 0x000000010f8205ef libstd-8d84a71a2f773b92.dylib`__rust_maybe_catch_panic + 31
    frame #23: 0x00000001137ffdb6 librustc_codegen_llvm-llvm.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h89669d2534a37616 + 134
    frame #24: 0x000000010f7f255e libstd-8d84a71a2f773b92.dylib`_$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hf374624fb08997e7 + 62
    frame #25: 0x000000010f81f3ae libstd-8d84a71a2f773b92.dylib`std::sys::unix::thread::Thread::new::thread_start::hfe8d238f75e9e68d + 142
    frame #26: 0x00007fff57fd32eb libsystem_pthread.dylib`_pthread_body + 126
    frame #27: 0x00007fff57fd6249 libsystem_pthread.dylib`_pthread_start + 66
    frame #28: 0x00007fff57fd240d libsystem_pthread.dylib`thread_start + 13
(lldb) thread backtrace 9
  thread #9, stop reason = signal SIGSTOP
    frame #0: 0x0000000114883414 librustc_codegen_llvm-llvm.dylib`llvm::DAGTypeLegalizer::ReplaceValueWith(llvm::SDValue, llvm::SDValue) + 788
    frame #1: 0x0000000114897bcf librustc_codegen_llvm-llvm.dylib`llvm::DAGTypeLegalizer::ScalarizeVectorOperand(llvm::SDNode*, unsigned int) + 95
    frame #2: 0x0000000114882b84 librustc_codegen_llvm-llvm.dylib`llvm::DAGTypeLegalizer::run() + 1108
    frame #3: 0x0000000114887565 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAG::LegalizeTypes() + 1221
    frame #4: 0x00000001149670a6 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGISel::CodeGenAndEmitDAG() + 342
    frame #5: 0x000000011496627d librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGISel::SelectAllBasicBlocks(llvm::Function const&) + 9453
    frame #6: 0x0000000114963376 librustc_codegen_llvm-llvm.dylib`llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) + 1782
    frame #7: 0x0000000114b3b3a9 librustc_codegen_llvm-llvm.dylib`llvm::MachineFunctionPass::runOnFunction(llvm::Function&) + 329
    frame #8: 0x0000000115457eb9 librustc_codegen_llvm-llvm.dylib`llvm::FPPassManager::runOnFunction(llvm::Function&) + 1177
    frame #9: 0x0000000115458153 librustc_codegen_llvm-llvm.dylib`llvm::FPPassManager::runOnModule(llvm::Module&) + 67
    frame #10: 0x0000000115458529 librustc_codegen_llvm-llvm.dylib`llvm::legacy::PassManagerImpl::run(llvm::Module&) + 761
    frame #11: 0x00000001139840b8 librustc_codegen_llvm-llvm.dylib`LLVMRustWriteOutputFile + 568
    frame #12: 0x00000001138cfd36 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::write_output_file::h46c8b7792ec6c3bf (.llvm.14679628501088474520) + 86
    frame #13: 0x0000000113946cb7 librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::codegen::_$u7b$$u7b$closure$u7d$$u7d$::h0a3d470f451a3cca (.llvm.12709569147983681104) + 983
    frame #14: 0x000000011393ccb0 librustc_codegen_llvm-llvm.dylib`rustc::util::common::time_ext::he76c8601a2e54611 + 80
    frame #15: 0x00000001138d2a0a librustc_codegen_llvm-llvm.dylib`rustc_codegen_llvm::back::write::codegen::h1854c102d66890de + 2890
    frame #16: 0x0000000113846898 librustc_codegen_llvm-llvm.dylib`rustc_codegen_ssa::back::write::execute_work_item::h099f6feee3dc5c4f + 4744
    frame #17: 0x0000000113971f75 librustc_codegen_llvm-llvm.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::heb8aade351110442 + 181
    frame #18: 0x0000000113855e1b librustc_codegen_llvm-llvm.dylib`std::panicking::try::do_call::hc37c119df709ca26 (.llvm.371732380873419076) + 43
    frame #19: 0x000000010f8205ef libstd-8d84a71a2f773b92.dylib`__rust_maybe_catch_panic + 31
    frame #20: 0x00000001137ffdb6 librustc_codegen_llvm-llvm.dylib`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h89669d2534a37616 + 134
    frame #21: 0x000000010f7f255e libstd-8d84a71a2f773b92.dylib`_$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hf374624fb08997e7 + 62
    frame #22: 0x000000010f81f3ae libstd-8d84a71a2f773b92.dylib`std::sys::unix::thread::Thread::new::thread_start::hfe8d238f75e9e68d + 142
    frame #23: 0x00007fff57fd32eb libsystem_pthread.dylib`_pthread_body + 126
    frame #24: 0x00007fff57fd6249 libsystem_pthread.dylib`_pthread_start + 66
    frame #25: 0x00007fff57fd240d libsystem_pthread.dylib`thread_start + 13
