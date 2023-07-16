
[0x0]   rustc_driver_35567a90ab3243c7!llvm::SelectionDAG::ReplaceAllUsesWith + 0x171   
[0x1]   rustc_driver_35567a90ab3243c7!llvm::SmallVectorImpl<llvm::IntervalMapImpl::NodeRef>::swap + 0x49a5   
[0x2]   rustc_driver_35567a90ab3243c7!llvm::EVT::bitsEq + 0x1dc6   
[0x3]   rustc_driver_35567a90ab3243c7!llvm::TargetLowering::DAGCombinerInfo::CommitTargetLoweringOpt + 0x8550   
[0x4]   rustc_driver_35567a90ab3243c7!llvm::SelectionDAG::Combine + 0x34   
[0x5]   rustc_driver_35567a90ab3243c7!llvm::SelectionDAGISel::CodeGenAndEmitDAG + 0xee   
[0x6]   rustc_driver_35567a90ab3243c7!llvm::SelectionDAGISel::SelectAllBasicBlocks + 0x1393   
[0x7]   rustc_driver_35567a90ab3243c7!llvm::SelectionDAGISel::runOnMachineFunction + 0x5ce   
[0x8]   rustc_driver_35567a90ab3243c7!llvm::SmallVectorImpl<std::pair<llvm::SDValue,llvm::SDNode * __ptr64> >::resize + 0x104   
[0x9]   rustc_driver_35567a90ab3243c7!llvm::MachineFunctionPass::runOnFunction + 0x1d4   
[0xa]   rustc_driver_35567a90ab3243c7!llvm::FPPassManager::runOnFunction + 0x288   
[0xb]   rustc_driver_35567a90ab3243c7!llvm::FPPassManager::runOnModule + 0x43   
[0xc]   rustc_driver_35567a90ab3243c7!llvm::FPPassManager::runOnModule + 0x287   
[0xd]   rustc_driver_35567a90ab3243c7!llvm::legacy::PassManager::run + 0xb0   
[0xe]   rustc_driver_35567a90ab3243c7!LLVMRustWriteOutputFile + 0x353   
[0xf]   rustc_driver_35567a90ab3243c7!rustc_codegen_llvm::back::write::write_output_file + 0xe3   
[0x10]   rustc_driver_35567a90ab3243c7!rustc_codegen_llvm::back::write::codegen::{{closure}} + 0x3a   
[0x11]   rustc_driver_35567a90ab3243c7!rustc_codegen_llvm::back::write::codegen::with_codegen<closure-2,core::result::Result<tuple<>, rustc_span::fatal_error::FatalError>> + 0x84   
[0x12]   rustc_driver_35567a90ab3243c7!rustc_codegen_llvm::back::write::codegen + 0xdcd   
[0x13]   rustc_driver_35567a90ab3243c7!rustc_codegen_llvm::{{impl}}::codegen + 0x4a   
[0x14]   rustc_driver_35567a90ab3243c7!rustc_codegen_ssa::back::write::finish_intra_module_work<rustc_codegen_llvm::LlvmCodegenBackend> + 0xf9   
[0x15]   rustc_driver_35567a90ab3243c7!rustc_codegen_ssa::back::write::execute_optimize_work_item + 0x4d3   
[0x16]   rustc_driver_35567a90ab3243c7!rustc_codegen_ssa::back::write::execute_work_item<rustc_codegen_llvm::LlvmCodegenBackend> + 0x891   
[0x17]   rustc_driver_35567a90ab3243c7!rustc_codegen_ssa::back::write::spawn_work::{{closure}} + 0xb0   
[0x18]   rustc_driver_35567a90ab3243c7!std::sys_common::backtrace::__rust_begin_short_backtrace<closure-0,tuple<>> + 0xdf   
[0x19]   rustc_driver_35567a90ab3243c7!std::thread::{{impl}}::spawn_unchecked::{{closure}}::{{closure}} + 0x1c   
[0x1a]   rustc_driver_35567a90ab3243c7!std::panic::{{impl}}::call_once + 0x1c   
[0x1b]   rustc_driver_35567a90ab3243c7!std::panicking::try::do_call + 0x20   
[0x1c]   rustc_driver_35567a90ab3243c7!std::panicking::try<tuple<>,std::panic::AssertUnwindSafe<closure-0>> + 0x3e   
[0x1d]   rustc_driver_35567a90ab3243c7!std::panic::catch_unwind + 0x2b   
[0x1e]   rustc_driver_35567a90ab3243c7!std::thread::{{impl}}::spawn_unchecked::{{closure}} + 0xf9   
[0x1f]   rustc_driver_35567a90ab3243c7!core::ops::function::FnOnce::call_once<closure-0,tuple<>> + 0x12b   
[0x20]   std_590731d75ecdff17!alloc::boxed::{{impl}}::call_once + 0xe   
[0x21]   std_590731d75ecdff17!alloc::boxed::{{impl}}::call_once<tuple<>,alloc::boxed::Box<FnOnce<tuple<>>, alloc::alloc::Global>,alloc::alloc::Global> + 0x2b   
[0x22]   std_590731d75ecdff17!std::sys::windows::thread::{{impl}}::new::thread_start + 0x23   
[0x23]   KERNEL32!BaseThreadInitThunk + 0x14   
[0x24]   ntdll!RtlUserThreadStart + 0x21
