plain
[TIMING] RustbookSrc { target: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None }, name: "rustdoc", src: "D:\\a\\rust\\rust\\src/doc/rustdoc" } -- 0.002
[TIMING] RustdocBook { target: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } } -- 0.000
[TIMING] RustbookSrc { target: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None }, name: "rust-by-example", src: "D:\\a\\rust\\rust\\src/doc/rust-by-example" } -- 0.002
[TIMING] RustByExample { target: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } } -- 0.000
[TIMING] ToolBuild { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } }, target: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None }, tool: "lint-docs", path: "src/tools/lint-docs", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 0.000
[TIMING] LintDocs { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } }, target: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } } -- 0.000
[TIMING] RustbookSrc { target: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None }, name: "rustc", src: "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\md-doc\\rustc" } -- 0.002
[TIMING] RustbookSrc { target: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None }, name: "cargo", src: "D:\\a\\rust\\rust\\src/tools/cargo/src/doc" } -- 0.002
[TIMING] CargoBook { target: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } } -- 0.000
[TIMING] RustbookSrc { target: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None }, name: "embedded-book", src: "D:\\a\\rust\\rust\\src/doc/embedded-book" } -- 0.002
[TIMING] EmbeddedBook { target: TargetSelection { triple: "x86_64-pc-windows-msvc", file: None } } -- 0.000
---
[2250/2684] Building RC object utils\count\CMakeFiles\count.dir\__\__\resources\windows_version_resource.rc.res
[2251/2684] Building RC object utils\not\CMakeFiles\not.dir\__\__\resources\windows_version_resource.rc.res
[2252/2684] Creating export file for LTO
[2253/2684] Linking CXX static library lib\LLVMARMCodeGen.lib
FAILED: lib/LLVMARMCodeGen.lib 
cmd.exe /C "cd . && C:\PROGRA~2\MICROS~1\2019\ENTERP~1\VC\Tools\MSVC\1427~1.291\bin\Hostx64\x64\lib.exe /nologo /machine:x64 /out:lib\LLVMARMCodeGen.lib lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\A15SDOptimizer.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMAsmPrinter.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMBaseInstrInfo.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMBaseRegisterInfo.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMBasicBlockInfo.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMCallingConv.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMCallLowering.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMConstantIslandPass.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMConstantPoolValue.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMExpandPseudoInsts.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMFastISel.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMFrameLowering.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMHazardRecognizer.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMInstructionSelector.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMISelDAGToDAG.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMISelLowering.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMInstrInfo.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMLegalizerInfo.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMParallelDSP.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMLoadStoreOptimizer.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMLowOverheadLoops.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMMCInstLower.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMMachineFunctionInfo.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMMacroFusion.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMRegisterInfo.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMOptimizeBarriersPass.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMRegisterBankInfo.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMSelectionDAGInfo.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMSubtarget.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMTargetMachine.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMTargetObjectFile.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMTargetTransformInfo.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\MLxExpansionPass.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\MVEGatherScatterLowering.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\MVETailPredication.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\MVEVPTBlockPass.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\MVEVPTOptimisationsPass.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\Thumb1FrameLowering.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\Thumb1InstrInfo.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ThumbRegisterInfo.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\Thumb2ITBlockPass.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\Thumb2InstrInfo.cpp.obj lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\Thumb2SizeReduction.cpp.obj  && cd ."
[2255/2684] Building C object utils\count\CMakeFiles\count.dir\count.c.obj
[2256/2684] Building CXX object utils\PerfectShuffle\CMakeFiles\llvm-PerfectShuffle.dir\PerfectShuffle.cpp.obj
[2257/2684] Building CXX object utils\not\CMakeFiles\not.dir\not.cpp.obj
[2258/2684] Building CXX object utils\FileCheck\CMakeFiles\FileCheck.dir\FileCheck.cpp.obj
[2258/2684] Building CXX object utils\FileCheck\CMakeFiles\FileCheck.dir\FileCheck.cpp.obj
[2259/2684] Building CXX object utils\yaml-bench\CMakeFiles\yaml-bench.dir\YAMLBench.cpp.obj
[2260/2684] Building CXX object tools\llvm-ar\CMakeFiles\llvm-ar.dir\llvm-ar.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1
 finished in 194.628


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.44\src\lib.rs:885:5
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap test --stage 2 src/test/ui src/test/compile-fail src/tools/linkchecker
Build completed unsuccessfully in 0:04:35
Build completed unsuccessfully in 0:04:35
make: *** [Makefile:75: ci-subset-2] Error 1
  local time: Tue Sep 15 04:39:44 CUT 2020
  network time: Tue, 15 Sep 2020 04:39:44 GMT
== end clock drift check ==
== end clock drift check ==
##[error]Process completed with exit code 2.
Terminate orphan process: pid (1236) (node)
Terminate orphan process: pid (7816) (python)
Terminate orphan process: pid (5248) (sccache)
