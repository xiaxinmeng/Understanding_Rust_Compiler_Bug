plain
-- Performing Test CXX_SUPPORTS_WERROR_UNGUARDED_AVAILABILITY_NEW
-- Performing Test CXX_SUPPORTS_WERROR_UNGUARDED_AVAILABILITY_NEW - Failed
-- Performing Test CXX_SUPPORTS_MISSING_FIELD_INITIALIZERS_FLAG
-- Performing Test CXX_SUPPORTS_MISSING_FIELD_INITIALIZERS_FLAG - Success
-- Performing Test C_SUPPORTS_CXX98_COMPAT_EXTRA_SEMI_FLAG
-- Performing Test C_SUPPORTS_CXX98_COMPAT_EXTRA_SEMI_FLAG - Failed
-- Performing Test CXX_SUPPORTS_CXX98_COMPAT_EXTRA_SEMI_FLAG
-- Performing Test CXX_SUPPORTS_CXX98_COMPAT_EXTRA_SEMI_FLAG - Failed
-- Performing Test C_SUPPORTS_IMPLICIT_FALLTHROUGH_FLAG - Failed
-- Performing Test CXX_SUPPORTS_IMPLICIT_FALLTHROUGH_FLAG
-- Performing Test CXX_SUPPORTS_IMPLICIT_FALLTHROUGH_FLAG - Failed
-- Performing Test C_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG
---
-- Performing Test CXX_SUPPORTS_FDATA_SECTIONS
-- Performing Test CXX_SUPPORTS_FDATA_SECTIONS - Success
-- Looking for os_signpost_interval_begin
-- Looking for os_signpost_interval_begin - not found
-- Found Python3: C:/hostedtoolcache/windows/Python/3.9.6/x86/python3.exe (found suitable version "3.9.6", minimum required is "3.0") found components: Interpreter 
-- Performing Test HAS_WERROR_GLOBAL_CTORS
-- Performing Test HAS_WERROR_GLOBAL_CTORS - Failed
-- LLVMHello ignored -- Loadable modules not supported on this platform.
-- Targeting AArch64
-- Targeting ARM
-- Targeting BPF
---
[159/2802] Linking CXX static library lib\libLLVMSupport.a
[160/2802] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/SetTheory.cpp.obj
[161/2802] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TableGenBackendSkeleton.cpp.obj
[162/2802] Linking CXX static library lib\libLLVMFileCheck.a
[163/2802] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeBeadsGen.cpp.obj
[165/2802] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeGenMapTable.cpp.obj
[166/2802] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TGParser.cpp.obj
[167/2802] Linking CXX static library lib\libLLVMTableGen.a
[168/2802] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeEmitterGen.cpp.obj
---
[270/2802] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/MCAsmParser.cpp.obj
[271/2802] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/MCAsmParserExtension.cpp.obj
[272/2802] Building CXX object lib/InterfaceStub/CMakeFiles/LLVMInterfaceStub.dir/IFSHandler.cpp.obj
[273/2802] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/WasmAsmParser.cpp.obj
[274/2802] Building CXX object lib/InterfaceStub/CMakeFiles/LLVMInterfaceStub.dir/IFSStub.cpp.obj
D:/a/rust/rust/src/llvm-project/llvm/lib/InterfaceStub/IFSStub.cpp: In copy constructor 'llvm::ifs::IFSStubTriple::IFSStubTriple(const llvm::ifs::IFSStubTriple&)':
D:/a/rust/rust/src/llvm-project/llvm/lib/InterfaceStub/IFSStub.cpp:32:1: warning: base class 'struct llvm::ifs::IFSStub' should be explicitly initialized in the copy constructor [-Wextra]
 IFSStubTriple::IFSStubTriple(IFSStubTriple const &Stub) {
[275/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/AllocationOrder.cpp.obj
[276/2802] Building CXX object lib/InterfaceStub/CMakeFiles/LLVMInterfaceStub.dir/ELFObjHandler.cpp.obj
[277/2802] Building CXX object lib/IRReader/CMakeFiles/LLVMIRReader.dir/IRReader.cpp.obj
[278/2802] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/AsmParser.cpp.obj
---
[293/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ExpandPostRAPseudos.cpp.obj
[294/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/BasicBlockSections.cpp.obj
[295/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/DeadMachineInstructionElim.cpp.obj
[296/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/AtomicExpandPass.cpp.obj
[297/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/EHContGuardCatchret.cpp.obj
[299/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/EdgeBundles.cpp.obj
[300/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/DFAPacketizer.cpp.obj
[301/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ExecutionDomainFix.cpp.obj
[302/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/EarlyIfConversion.cpp.obj
---
[388/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MBFIWrapper.cpp.obj
[389/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MIRYamlMapping.cpp.obj
[390/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MachineVerifier.cpp.obj
[391/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MachinePipeliner.cpp.obj
[392/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MIRFSDiscriminator.cpp.obj
[394/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MacroFusion.cpp.obj
[395/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ModuloSchedule.cpp.obj
[396/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/PHIEliminationUtils.cpp.obj
[397/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/OptimizePHIs.cpp.obj
---
[424/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegisterCoalescer.cpp.obj
[425/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RDFLiveness.cpp.obj
[426/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegUsageInfoCollector.cpp.obj
[427/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ResetMachineFunctionPass.cpp.obj
[428/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ReplaceWithVeclib.cpp.obj
[430/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegisterUsageInfo.cpp.obj
[431/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MIRVRegNamerUtils.cpp.obj
[432/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegAllocGreedy.cpp.obj
[433/2802] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/StackMapLivenessAnalysis.cpp.obj
---
[835/2802] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/ElimAvailExtern.cpp.obj
[836/2802] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/SimpleLoopUnswitch.cpp.obj
[837/2802] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/BarrierNoopPass.cpp.obj
[838/2802] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/FunctionImport.cpp.obj
FAILED: lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/FunctionImport.cpp.obj 
sccache D:\a\rust\rust\mingw32\bin\g++.exe -DGTEST_HAS_RTTI=0 -D_FILE_OFFSET_BITS=64 -D_LARGEFILE_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -ID:/a/rust/rust/build/i686-pc-windows-gnu/llvm/build/lib/Transforms/IPO -ID:/a/rust/rust/src/llvm-project/llvm/lib/Transforms/IPO -ID:/a/rust/rust/build/i686-pc-windows-gnu/llvm/build/include -ID:/a/rust/rust/src/llvm-project/llvm/include -ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer -static-libstdc++ -Wa,-mbig-obj -Werror=date-time -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -Wmisleading-indentation -ffunction-sections -fdata-sections  -O2 -DNDEBUG  -fno-exceptions -fno-rtti -std=c++14 -MD -MT lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/FunctionImport.cpp.obj -MF lib\Transforms\IPO\CMakeFiles\LLVMipo.dir\FunctionImport.cpp.obj.d -o lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/FunctionImport.cpp.obj -c D:/a/rust/rust/src/llvm-project/llvm/lib/Transforms/IPO/FunctionImport.cpp
D:/a/rust/rust/src/llvm-project/llvm/lib/Transforms/IPO/FunctionImport.cpp: In function 'void computeImportForFunction(const llvm::FunctionSummary&, const llvm::ModuleSummaryIndex&, unsigned int, const GVSummaryMapTy&, llvm::SmallVectorImpl<std::tuple<const llvm::GlobalValueSummary*, unsigned int> >&, llvm::FunctionImporter::ImportMapTy&, llvm::StringMap<llvm::DenseSet<llvm::ValueInfo> >*, llvm::FunctionImporter::ImportThresholdsTy&)':
D:/a/rust/rust/src/llvm-project/llvm/lib/Transforms/IPO/FunctionImport.cpp:499:41: error: 'operation_not_supported' is not a member of 'std::errc'
               Msg, std::make_error_code(std::errc::operation_not_supported));
[839/2802] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/CalledValuePropagation.cpp.obj
[840/2802] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/ArgumentPromotion.cpp.obj
[841/2802] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/GlobalDCE.cpp.obj
[842/2802] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/FunctionAttrs.cpp.obj
[842/2802] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/FunctionAttrs.cpp.obj
[843/2802] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/BlockExtractor.cpp.obj
[844/2802] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/Attributor.cpp.obj
[845/2802] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/AttributorAttributes.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\github.com-1285ae84e5963aae\cmake-0.1.44\src\lib.rs:885:5
 finished in 470.472 seconds
Build completed unsuccessfully in 0:10:46
