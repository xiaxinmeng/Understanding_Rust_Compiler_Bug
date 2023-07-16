plain
[00:20:16] [  1%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/MsgPackReader.cpp.o
[00:20:16] [  2%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TGParser.cpp.o
[00:20:16] [  2%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Allocator.cpp.o
[00:20:16] [  2%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenDAGPatterns.cpp.o
[00:20:16] [  2%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/MsgPackWriter.cpp.o
[00:20:17] [  2%] Linking CXX static library ../libLLVMTableGen.a
[00:20:17] [  2%] Built target LLVMTableGen
[00:20:17] [  2%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/BinaryStreamReader.cpp.o
[00:20:17] [  2%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/Wasm.cpp.o
---
[00:20:28] [  8%] Building CXX object lib/ObjectYAML/CMakeFiles/LLVMObjectYAML.dir/COFFYAML.cpp.o
[00:20:28] [  8%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCLabel.cpp.o
[00:20:28] [  8%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/WebAssemblyDisassemblerEmitter.cpp.o
[00:20:28] [  8%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Options.cpp.o
[00:20:28] [  8%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/WebAssemblyStackifierEmitter.cpp.o
[00:20:28] [  8%] Building CXX object lib/ObjectYAML/CMakeFiles/LLVMObjectYAML.dir/DWARFEmitter.cpp.o
[00:20:28] [  8%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Parallel.cpp.o
[00:20:28] [  8%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CTagsEmitter.cpp.o
[00:20:28] [  8%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCMachOStreamer.cpp.o
---
[00:21:41] [ 33%] Building MSP430GenAsmWriter.inc...
[00:21:41] [ 33%] Building SystemZGenRegisterInfo.inc...
[00:21:41] [ 33%] Building MSP430GenCallingConv.inc...
[00:21:41] [ 33%] Building MSP430GenDAGISel.inc...
[00:21:41] warning: SubRegIndex SystemZ::subreg_h64 and SystemZ::subreg_h32 compose ambiguously as SystemZ::subreg_hh32 or SystemZ::subreg_h32
[00:21:41] [ 33%] Building MSP430GenInstrInfo.inc...
[00:21:42] [ 33%] Building MSP430GenRegisterInfo.inc...
[00:21:42] [ 33%] Building MSP430GenSubtargetInfo.inc...
[00:21:42] [ 33%] Updating MSP430GenAsmWriter.inc...
---
[00:22:00] [ 43%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/BuiltinGCs.cpp.o
[00:22:00] [ 44%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/DiagnosticInfo.cpp.o
[00:22:01] [ 44%] Building CXX object lib/CodeGen/SelectionDAG/CMakeFiles/LLVMSelectionDAG.dir/LegalizeFloatTypes.cpp.o
[00:22:01] [ 44%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/CalcSpillWeights.cpp.o
[00:22:01] [ 44%] Building CXX object lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/DbgEntityHistoryCalculator.cpp.o
[00:22:01] [ 44%] Building CXX object lib/CodeGen/SelectionDAG/CMakeFiles/LLVMSelectionDAG.dir/LegalizeIntegerTypes.cpp.o
[00:22:01] [ 44%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/CallingConvLower.cpp.o
[00:22:01] [ 44%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Dominators.cpp.o
[00:22:01] [ 44%] Building CXX object lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/DebugHandlerBase.cpp.o
---
[00:24:35] [ 81%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyFastISel.cpp.o
[00:24:35] [ 81%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCTargetDesc.cpp.o
[00:24:35] [ 81%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyFixIrreducibleControlFlow.cpp.o
[00:24:35] In file included from /checkout/src/llvm/lib/Target/WebAssembly/WebAssemblyFastISel.cpp:201:0:
[00:24:35] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/WebAssembly/WebAssemblyGenFastISel.inc: In static member function 'static bool {anonymous}::WebAssemblyFastISel::Predicate_ImmI8(int64_t)':
[00:24:35] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/WebAssembly/WebAssemblyGenFastISel.inc:12:33: warning: comparison between signed and unsigned integer expressions [-Wsign-compare]
[00:24:35]  return (Imm & ((1UL << 8) - 1)) == Imm;
[00:24:35]                                  ^
[00:24:35] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/WebAssembly/WebAssemblyGenFastISel.inc: In static member function 'static bool {anonymous}::WebAssemblyFastISel::Predicate_ImmI16(int64_t)':
[00:24:35] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/WebAssembly/WebAssemblyGenFastISel.inc:15:34: warning: comparison between signed and unsigned integer expressions [-Wsign-compare]
[00:24:35]  return (Imm & ((1UL << 16) - 1)) == Imm;
[00:24:35] [ 81%] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonRDFOpt.cpp.o
[00:24:36] [ 81%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonShuffler.cpp.o
[00:24:36] [ 81%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyFixFunctionBitcasts.cpp.o
[00:24:36] [ 81%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyFrameLowering.cpp.o
[00:24:36] [ 81%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyFrameLowering.cpp.o
[00:24:36] [ 81%] Linking CXX static library ../../../libLLVMHexagonDesc.a
[00:24:36] [ 81%] Built target LLVMHexagonDesc
[00:24:36] [ 81%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyISelDAGToDAG.cpp.o
[00:24:37] [ 81%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyISelLowering.cpp.o
[00:24:37] In file included from /checkout/src/llvm/lib/Target/WebAssembly/WebAssemblyISelDAGToDAG.cpp:63:0:
[00:24:37] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/WebAssembly/WebAssemblyGenDAGISel.inc: In member function 'virtual bool {anonymous}::WebAssemblyDAGToDAGISel::CheckNodePredicate(llvm::SDNode*, unsigned int) const':
[00:24:37] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/WebAssembly/WebAssemblyGenDAGISel.inc:27837:33: warning: comparison between signed and unsigned integer expressions [-Wsign-compare]
[00:24:37]  return (Imm & ((1UL << 8) - 1)) == Imm;
[00:24:37] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/WebAssembly/WebAssemblyGenDAGISel.inc:27842:34: warning: comparison between signed and unsigned integer expressions [-Wsign-compare]
[00:24:37] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/WebAssembly/WebAssemblyGenDAGISel.inc:27842:34: warning: comparison between signed and unsigned integer expressions [-Wsign-compare]
[00:24:37]  return (Imm & ((1UL << 16) - 1)) == Imm;
[00:24:37] [ 81%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyInstrInfo.cpp.o
[00:24:37] [ 81%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyLowerBrUnless.cpp.o
[00:24:37] [ 81%] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonRegisterInfo.cpp.o
[00:24:38] [ 81%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyLowerEmscriptenEHSjLj.cpp.o
---
[00:25:24] [ 86%] Building CXX object tools/llvm-mca/lib/CMakeFiles/LLVMMCA.dir/Context.cpp.o
[00:25:24] [ 86%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/Latency.cpp.o
[00:25:24] [ 86%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/LlvmState.cpp.o
[00:25:24] [ 86%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/MCInstrDescView.cpp.o
[00:25:24] [ 86%] Building CXX object tools/llvm-mca/lib/CMakeFiles/LLVMMCA.dir/HWEventListener.cpp.o
[00:25:25] [ 86%] Building CXX object tools/llvm-mca/lib/CMakeFiles/LLVMMCA.dir/HardwareUnits/HardwareUnit.cpp.o
[00:25:25] [ 86%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/RegisterAliasing.cpp.o
[00:25:25] [ 86%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/RegisterAliasing.cpp.o
[00:25:25] [ 86%] Building CXX object tools/llvm-mca/lib/CMakeFiles/LLVMMCA.dir/HardwareUnits/LSUnit.cpp.o
[00:25:25] [ 86%] Building CXX object tools/llvm-mca/lib/CMakeFiles/LLVMMCA.dir/HardwareUnits/RegisterFile.cpp.o
[00:25:25] [ 86%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/Uops.cpp.o
[00:25:25] [ 86%] Building CXX object tools/llvm-mca/lib/CMakeFiles/LLVMMCA.dir/HardwareUnits/ResourceManager.cpp.o
[00:25:25] [ 86%] Building CXX object tools/llvm-mca/lib/CMakeFiles/LLVMMCA.dir/HardwareUnits/RetireControlUnit.cpp.o
[00:25:25] [ 86%] Building CXX object tools/llvm-mca/lib/CMakeFiles/LLVMMCA.dir/HardwareUnits/RetireControlUnit.cpp.o
[00:25:25] [ 86%] Building CXX object tools/llvm-mca/lib/CMakeFiles/LLVMMCA.dir/HardwareUnits/Scheduler.cpp.o
[00:25:25] [ 86%] Building CXX object tools/llvm-mca/lib/CMakeFiles/LLVMMCA.dir/InstrBuilder.cpp.o
[00:25:25] [ 86%] Built target LLVMExegesis
[00:25:25] [ 87%] Building CXX object tools/llvm-mca/lib/CMakeFiles/LLVMMCA.dir/Instruction.cpp.o
[00:25:25] [ 87%] Building CXX object tools/llvm-mca/lib/CMakeFiles/LLVMMCA.dir/Pipeline.cpp.o
[00:25:25] [ 87%] Building CXX object tools/llvm-mca/lib/CMakeFiles/LLVMMCA.dir/Stages/DispatchStage.cpp.o
---
[00:27:09] [ 96%] Building CXX object tools/opt/CMakeFiles/opt.dir/BreakpointPrinter.cpp.o
[00:27:09] [ 96%] Building CXX object tools/opt/CMakeFiles/opt.dir/Debugify.cpp.o
[00:27:11] [ 97%] Linking CXX executable ../../bin/yaml2obj
[00:27:12] [ 97%] Built target yaml2obj
[00:27:12] Scanning dependencies of target llvm-yaml-numeric-parser-fuzzer
[00:27:12] [ 97%] Building CXX object tools/llvm-yaml-numeric-parser-fuzzer/CMakeFiles/llvm-yaml-numeric-parser-fuzzer.dir/DummyYAMLNumericParserFuzzer.cpp.o
[00:27:13] [ 97%] Building CXX object tools/llvm-yaml-numeric-parser-fuzzer/CMakeFiles/llvm-yaml-numeric-parser-fuzzer.dir/yaml-numeric-parser-fuzzer.cpp.o
[00:27:13] [ 97%] Building CXX object tools/llvm-split/CMakeFiles/llvm-split.dir/llvm-split.cpp.o
[00:27:13] [ 97%] Linking CXX executable ../../bin/llvm-yaml-numeric-parser-fuzzer
[00:27:14] [ 97%] Built target llvm-yaml-numeric-parser-fuzzer
[00:27:14] Scanning dependencies of target llvm-readobj
---
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ErrorOr.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Signals.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/MD5.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/CodeGenCoverage.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/CFGUpdate.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/GenericDomTreeConstruction.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Errc.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ScaledNumber.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ScopedPrinter.h
---
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Testing/Support/SupportHelpers.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Config
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/WasmRelocs.def
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/MsgPack.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/MsgPack.def
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/Magic.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/DynamicTags.def
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/Dwarf.def
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/Dwarf.h
---
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/PowerPC64.def
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELF.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/MachO.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/COFF.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/MsgPackReader.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/MsgPackWriter.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/LineEditor/LineEditor.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Linker
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Linker/IRMover.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Linker/Linker.h
---
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IRReader/IRReader.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/PassSupport.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/YAMLXRayRecord.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/FileHeaderReader.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/Graph.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/XRayRecord.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/InstrumentationMap.h
[00:28:14] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/WindowsManifest
---
Building stage2 std artifacts (x86_64-unknown-linux-gnu -> riscv32imac-unknown-none-elf)
[01:19:02]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[01:19:02]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[01:19:24] [RUSTC-TIMING] core test:false 21.366
[01:19:24] error: no rules expected the token `a`
[01:19:24]  --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/riscv32.rs:4:36
[01:19:24]   |
[01:19:24] 4 |     pub extern "C" fn __mulsi3(mut a: u32, mut b: u32) -> u32 {
[01:19:24] 
[01:19:24] error: aborting due to previous error
[01:19:24] 
[01:19:24] [RUSTC-TIMING] compiler_builtins test:false 0.120
[01:19:24] [RUSTC-TIMING] compiler_builtins test:false 0.120
[01:19:24] error: Could not compile `compiler_builtins`.
[01:19:24] 
[01:19:24] Caused by:
[01:19:24]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name compiler_builtins rustc/compiler_builtins_shim/../../libcompiler_builtins/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 --cfg 'feature="c"' --cfg 'feature="compiler-builtins"' --cfg 'feature="default"' --cfg 'feature="mem"' --cfg 'feature="rustbuild"' -C metadata=f824b9fc30e0c8e5 -C extra-filename=-f824b9fc30e0c8e5 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/riscv32imac-unknown-none-elf/release/deps --target riscv32imac-unknown-none-elf -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/riscv32imac-unknown-none-elf/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/riscv32imac-unknown-none-elf/release/deps/libcore-b51bbe9fe9be3200.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/riscv32imac-unknown-none-elf/release/build/compiler_builtins-0484e3fe534fd11e/out --cfg use_c -l static=compiler-rt` (exit code: 1)
[01:19:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "riscv32imac-unknown-none-elf" "-j" "4" "--release" "--locked" "--color" "always" "--features" "c mem" "-p" "alloc" "-p" "compiler_builtins" "--manifest-path" "/checkout/src/rustc/compiler_builtins_shim/Cargo.toml" "--message-format" "json"
[01:19:24] travis_fold:end:stage2-std

[01:19:24] travis_time:end:stage2-std:start=1535617144001553976,finish=1535617165843112707,duration=21841558731


[01:19:24] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[01:19:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,riscv32imac-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf
[01:19:24] Build completed unsuccessfully in 0:21:28
travis_time:end:0ebbe8c4:start=1535612401663221283,finish=1535617166137845755,duration=4764474624472

---
travis_time:end:06bf958e:start=1535617168270555891,finish=1535617168279867595,duration=9311704
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:114314d0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13d335b6
travis_time:start:13d335b6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a534062
$ dmesg | grep -i kill
