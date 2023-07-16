plain
[00:18:58] [ 25%] Building CXX object lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/AsmPrinterInlineAsm.cpp.o
[00:18:58] [ 25%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Use.cpp.o
[00:18:58] [ 25%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/User.cpp.o
[00:18:58] [ 25%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/IfConversion.cpp.o
[00:18:58] [ 25%] Building CXX object lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/DbgEntityHistoryCalculator.cpp.o
[00:18:58] [ 25%] Building CXX object lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/CallLowering.cpp.o
[00:18:58] [ 25%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Value.cpp.o
[00:18:58] [ 25%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ImplicitNullChecks.cpp.o
[00:18:59] [ 25%] Building CXX object lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/DebugHandlerBase.cpp.o
---
[00:20:25] [ 52%] Building PPCGenRegisterInfo.inc...
[00:20:26] [ 52%] Building X86GenSubtargetInfo.inc...
[00:20:26] [ 52%] Building SystemZGenRegisterInfo.inc...
[00:20:26] [ 52%] Building PPCGenSubtargetInfo.inc...
[00:20:26] warning: SubRegIndex SystemZ::subreg_h64 and SystemZ::subreg_h32 compose ambiguously as SystemZ::subreg_hh32 or SystemZ::subreg_h32
[00:20:26] [ 53%] Updating PPCGenAsmMatcher.inc...
[00:20:26] [ 53%] Updating PPCGenAsmWriter.inc...
[00:20:26] [ 53%] Updating PPCGenCallingConv.inc...
[00:20:26] [ 53%] Updating PPCGenDAGISel.inc...
---
[00:21:36] [ 82%] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonHazardRecognizer.cpp.o
[00:21:36] [ 82%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyFixIrreducibleControlFlow.cpp.o
[00:21:36] [ 82%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCExpr.cpp.o
[00:21:36] In file included from /checkout/src/llvm/lib/Target/WebAssembly/WebAssemblyFastISel.cpp:201:0:
[00:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/WebAssembly/WebAssemblyGenFastISel.inc: In static member function 'static bool {anonymous}::WebAssemblyFastISel::Predicate_ImmI8(int64_t)':
[00:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/WebAssembly/WebAssemblyGenFastISel.inc:12:33: warning: comparison between signed and unsigned integer expressions [-Wsign-compare]
[00:21:36]  return (Imm & ((1UL << 8) - 1)) == Imm;
[00:21:36]                                  ^
[00:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/WebAssembly/WebAssemblyGenFastISel.inc: In static member function 'static bool {anonymous}::WebAssemblyFastISel::Predicate_ImmI16(int64_t)':
[00:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/WebAssembly/WebAssemblyGenFastISel.inc:15:34: warning: comparison between signed and unsigned integer expressions [-Wsign-compare]
[00:21:36]  return (Imm & ((1UL << 16) - 1)) == Imm;
[00:21:36] [ 82%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyFixFunctionBitcasts.cpp.o
[00:21:36] [ 82%] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonInstrInfo.cpp.o
[00:21:36] [ 82%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCInstrInfo.cpp.o
[00:21:36] [ 82%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyFrameLowering.cpp.o
[00:21:36] [ 82%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyFrameLowering.cpp.o
[00:21:37] [ 82%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCShuffler.cpp.o
[00:21:37] [ 82%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyISelDAGToDAG.cpp.o
[00:21:37] [ 82%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCTargetDesc.cpp.o
[00:21:37] [ 82%] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonISelDAGToDAG.cpp.o
[00:21:37] [ 82%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonShuffler.cpp.o
[00:21:37] In file included from /checkout/src/llvm/lib/Target/WebAssembly/WebAssemblyISelDAGToDAG.cpp:63:0:
[00:21:37] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/WebAssembly/WebAssemblyGenDAGISel.inc: In member function 'virtual bool {anonymous}::WebAssemblyDAGToDAGISel::CheckNodePredicate(llvm::SDNode*, unsigned int) const':
[00:21:37] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/WebAssembly/WebAssemblyGenDAGISel.inc:27304:33: warning: comparison between signed and unsigned integer expressions [-Wsign-compare]
[00:21:37]  return (Imm & ((1UL << 8) - 1)) == Imm;
[00:21:37] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/WebAssembly/WebAssemblyGenDAGISel.inc:27309:34: warning: comparison between signed and unsigned integer expressions [-Wsign-compare]
[00:21:37] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/WebAssembly/WebAssemblyGenDAGISel.inc:27309:34: warning: comparison between signed and unsigned integer expressions [-Wsign-compare]
[00:21:37]  return (Imm & ((1UL << 16) - 1)) == Imm;
[00:21:37] [ 82%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyISelLowering.cpp.o
[00:21:38] [ 82%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyInstrInfo.cpp.o
[00:21:38] [ 82%] Linking CXX static library ../../../libLLVMHexagonDesc.a
[00:21:38] [ 82%] Built target LLVMHexagonDesc
---
[00:22:13] [ 94%] Linking CXX executable ../../bin/llvm-exegesis
[00:22:13] [ 95%] Linking CXX executable ../../bin/yaml2obj
[00:22:13] [ 95%] Building CXX object tools/opt/CMakeFiles/opt.dir/BreakpointPrinter.cpp.o
[00:22:13] [ 95%] Built target llvm-stress
[00:22:13] Scanning dependencies of target llvm-yaml-numeric-parser-fuzzer
[00:22:13] [ 95%] Building CXX object tools/llvm-yaml-numeric-parser-fuzzer/CMakeFiles/llvm-yaml-numeric-parser-fuzzer.dir/DummyYAMLNumericParserFuzzer.cpp.o
[00:22:13] [ 95%] Building CXX object tools/llvm-yaml-numeric-parser-fuzzer/CMakeFiles/llvm-yaml-numeric-parser-fuzzer.dir/yaml-numeric-parser-fuzzer.cpp.o
[00:22:14] [ 95%] Built target yaml2obj
[00:22:14] Scanning dependencies of target llvm-split
[00:22:14] [ 95%] Building CXX object tools/llvm-split/CMakeFiles/llvm-split.dir/llvm-split.cpp.o
---
[00:22:27] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ErrorOr.h
[00:22:27] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Signals.h
[00:22:27] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/MD5.h
[00:22:27] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/CodeGenCoverage.h
[00:22:27] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/CFGUpdate.h
[00:22:27] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/GenericDomTreeConstruction.h
[00:22:27] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Errc.h
[00:22:27] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ScaledNumber.h
[00:22:27] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ScopedPrinter.h
---
[00:22:27] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IRReader/IRReader.h
[00:22:27] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/PassSupport.h
[00:22:27] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay
[00:22:27] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/YAMLXRayRecord.h
[00:22:27] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/FileHeaderReader.h
[00:22:27] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/Graph.h
[00:22:27] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/XRayRecord.h
[00:22:27] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/InstrumentationMap.h
[00:22:27] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/WindowsManifest
---
[00:53:04] ---- [ui] ui/rfc-1937-termination-trait/termination-trait-in-test.rs stdout ----
[00:53:04] 
[00:53:04] error: test run failed!
[00:53:04] status: exit code: 101
[00:53:04] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/a.wasm"
[00:53:04] ------------------------------------------
[00:53:04] 
[00:53:04] ------------------------------------------
[00:53:04] stderr:
[00:53:04] stderr:
[00:53:04] ------------------------------------------
[00:53:04] RuntimeError: unreachable
[00:53:04]     at .L_ZN105_$LT$std..ffi..os_str..OsStr$u20$as$u20$std..sys_common..AsInner$LT$std..sys..wasm..os_str..Slice$GT$$GT$8as_inner17h310f6db7f925ccafE_bitcast_invalid (wasm-function[588]:35)
[00:53:04]     at std::ffi::os_str::_$LT$impl$u20$core..convert..AsRef$LT$std..ffi..os_str..OsStr$GT$$u20$for$u20$str$GT$::as_ref::h41473c266afd9fd8 (wasm-function[589]:29)
[00:53:04]     at test::parse_opts::h761c4bc8fba6d0da (wasm-function[44]:2103)
[00:53:04]     at test::test_main::h96cffcb0f0fad280 (wasm-function[43]:31)
[00:53:04]     at test::test_main_static::hc0eb2276b2fd88a4 (wasm-function[47]:514)
[00:53:04]     at termination_trait_in_test::__test::main::h58a2340cc7761266 (wasm-function[5]:9)
[00:53:04]     at std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h7708c80d32ee89a3 (wasm-function[9]:25)
[00:53:04]     at std::sys_common::backtrace::__rust_begin_short_backtrace::hef891d3fbfd53ed7 (wasm-function[477]:8)
[00:53:04]     at std::panicking::try::do_call::hcd4e06221efde26f (.llvm.11842636124907191108) (wasm-function[574]:20)
[00:53:04]     at __rust_maybe_catch_panic (wasm-function[590]:5)
[00:53:04]     at std::rt::lang_start_internal::h4ec0d1991570d1a0 (wasm-function[455]:150)
[00:53:04]     at std::rt::lang_start::hdc135220797677f9 (wasm-function[8]:42)
[00:53:04]     at main (wasm-function[6]:11)
[00:53:04]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:136:20)
[00:53:04]     at Module._compile (module.js:641:30)
[00:53:04]     at Object.Module._extensions..js (module.js:652:10)
[00:53:04]     at Module.load (module.js:560:32)
[00:53:04]     at tryModuleLoad (module.js:503:12)
[00:53:04]     at Function.Module._load (module.js:495:3)
[00:53:04]     at Function.Module.runMain (module.js:682:10)
[00:53:04] ------------------------------------------
[00:53:04] 
[00:53:04] thread '[ui] ui/rfc-1937-termination-trait/termination-trait-in-test.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:53:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:53:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:53:04] 
[00:53:04] ---- [ui] ui/test-should-panic-attr.rs stdout ----
[00:53:04] 
[00:53:04] error: test run failed!
[00:53:04] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:53:04] status: exit code: 101
[00:53:04] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-should-panic-attr/a.wasm"
[00:53:04] ------------------------------------------
[00:53:04] 
[00:53:04] ------------------------------------------
[00:53:04] stderr:
[00:53:04] stderr:
[00:53:04] ------------------------------------------
[00:53:04] RuntimeError: unreachable
[00:53:04]     at .L_ZN105_$LT$std..ffi..os_str..OsStr$u20$as$u20$std..sys_common..AsInner$LT$std..sys..wasm..os_str..Slice$GT$$GT$8as_inner17h310f6db7f925ccafE_bitcast_invalid (wasm-function[599]:35)
[00:53:04]     at std::ffi::os_str::_$LT$impl$u20$core..convert..AsRef$LT$std..ffi..os_str..OsStr$GT$$u20$for$u20$str$GT$::as_ref::h41473c266afd9fd8 (wasm-function[600]:29)
[00:53:04]     at test::parse_opts::h761c4bc8fba6d0da (wasm-function[55]:2103)
[00:53:04]     at test::test_main::h96cffcb0f0fad280 (wasm-function[54]:31)
[00:53:04]     at test::test_main_static::hc0eb2276b2fd88a4 (wasm-function[58]:514)
[00:53:04]     at test_should_panic_attr::__test::main::he5c884ddc49ea873 (wasm-function[6]:9)
[00:53:04]     at std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h7963799190ba42bb (wasm-function[20]:25)
[00:53:04]     at std::sys_common::backtrace::__rust_begin_short_backtrace::hef891d3fbfd53ed7 (wasm-function[488]:8)
[00:53:04]     at std::panicking::try::do_call::hcd4e06221efde26f (.llvm.11842636124907191108) (wasm-function[585]:20)
[00:53:04]     at __rust_maybe_catch_panic (wasm-function[601]:5)
[00:53:04]     at std::rt::lang_start_internal::h4ec0d1991570d1a0 (wasm-function[466]:150)
[00:53:04]     at std::rt::lang_start::hd95b771f5003226c (wasm-function[19]:42)
[00:53:04]     at main (wasm-function[7]:11)
[00:53:04]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:136:20)
[00:53:04]     at Module._compile (module.js:641:30)
[00:53:04]     at Object.Module._extensions..js (module.js:652:10)
[00:53:04]     at Module.load (module.js:560:32)
[00:53:04]     at tryModuleLoad (module.js:503:12)
[00:53:04]     at Function.Module._load (module.js:495:3)
[00:53:04]     at Function.Module.runMain (module.js:682:10)
[00:53:04] ------------------------------------------
[00:53:04] 
[00:53:04] thread '[ui] ui/test-should-panic-attr.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:53:04] 
---
[00:53:04] test result: FAILED. 4126 passed; 2 failed; 40 ignored; 0 measured; 0 filtered out
[00:53:04] 
[00:53:04] 
[00:53:04] 
[00:53:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:53:04] 
[00:53:04] 
[00:53:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/parse-fail src/test/mir-opt src/test/codegen-units src/libcore
[00:53:04] Build completed unsuccessfully in 0:49:17
---
travis_time:end:0f4c2afe:start=1535256019735336171,finish=1535256019741714566,duration=6378395
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0be24a0a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0aa08200
travis_time:start:0aa08200
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06bd20d0
$ dmesg | grep -i kill
