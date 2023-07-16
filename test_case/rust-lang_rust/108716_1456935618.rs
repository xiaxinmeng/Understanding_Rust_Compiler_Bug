plain
[154/3023] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Threading.cpp.o
[155/3023] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/YAMLTraits.cpp.o
[156/3023] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Process.cpp.o
In file included from /checkout/src/llvm-project/llvm/lib/Support/Process.cpp:107:
/checkout/src/llvm-project/llvm/lib/Support/Unix/Process.inc: In static member function ‘static size_t llvm::sys::Process::GetMallocUsage()’:
/checkout/src/llvm-project/llvm/lib/Support/Unix/Process.inc:97:10: warning: ‘mallinfo mallinfo()’ is deprecated [-Wdeprecated-declarations]
   97 |   mi = ::mallinfo();
In file included from /checkout/src/llvm-project/llvm/lib/Support/Unix/Process.inc:34,
                 from /checkout/src/llvm-project/llvm/lib/Support/Process.cpp:107:
/usr/include/malloc.h:114:24: note: declared here
/usr/include/malloc.h:114:24: note: declared here
  114 | extern struct mallinfo mallinfo (void) __THROW __MALLOC_DEPRECATED;
In file included from /checkout/src/llvm-project/llvm/lib/Support/Process.cpp:107:
/checkout/src/llvm-project/llvm/lib/Support/Unix/Process.inc:97:18: warning: ‘mallinfo mallinfo()’ is deprecated [-Wdeprecated-declarations]
   97 |   mi = ::mallinfo();
      |        ~~~~~~~~~~^~
      |        ~~~~~~~~~~^~
In file included from /checkout/src/llvm-project/llvm/lib/Support/Unix/Process.inc:34,
                 from /checkout/src/llvm-project/llvm/lib/Support/Process.cpp:107:
/usr/include/malloc.h:114:24: note: declared here
  114 | extern struct mallinfo mallinfo (void) __THROW __MALLOC_DEPRECATED;
[157/3023] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/raw_ostream.cpp.o
[158/3023] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/StringMatcher.cpp.o
[159/3023] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Host.cpp.o
[160/3023] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TableGenBackendSkeleton.cpp.o
---
      |       ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
[291/3023] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/IRPrintingPasses.cpp.o
[292/3023] Building CXX object utils/TableGen/CMakeFiles/llvm-tblgen.dir/GlobalISelEmitter.cpp.o
[293/3023] Linking CXX executable bin/llvm-tblgen
FAILED: bin/llvm-tblgen 
: && /usr/bin/c++ -ffunction-sections -fdata-sections -fPIC -m32 -march=pentium -fPIC -fno-semantic-interposition -fvisibility-inlines-hidden -Werror=date-time -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wimplicit-fallthrough -Wno-maybe-uninitialized -Wno-class-memaccess -Wno-redundant-move -Wno-pessimizing-move -Wno-noexcept-type -Wdelete-non-virtual-dtor -Wsuggest-override -Wno-comment -Wmisleading-indentation -fdiagnostics-color -ffunction-sections -fdata-sections -O3 -DNDEBUG -Wl,-Bsymbolic -static-libstdc++    -Wl,-rpath-link,/checkout/obj/build/i586-unknown-linux-gnu/llvm/build/./lib  -Wl,--gc-sections utils/TableGen/CMakeFiles/llvm-tblgen.dir/AsmMatcherEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/AsmWriterEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/AsmWriterInst.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/Attributes.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/CallingConvEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeEmitterGen.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeGenDAGPatterns.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeGenHwModes.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeGenInstruction.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeGenMapTable.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeGenRegisters.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeGenSchedule.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/CodeGenTarget.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/DAGISelEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/DAGISelMatcherEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/DAGISelMatcherGen.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/DAGISelMatcherOpt.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/DAGISelMatcher.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/DecoderEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/DFAEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/DFAPacketizerEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/DirectiveEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/DisassemblerEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/DXILEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/ExegesisEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/FastISelEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/GICombinerEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/GlobalISelEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/InfoByHwMode.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/InstrInfoEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/InstrDocsEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/IntrinsicEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/OptEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/OptParserEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/OptRSTEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/PredicateExpander.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/PseudoLoweringEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/CompressInstEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/RegisterBankEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/RegisterInfoEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/SDNodeProperties.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/SearchableTableEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/SubtargetEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/SubtargetFeatureInfo.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/TableGen.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/Types.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/VarLenCodeEmitterGen.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/X86DisassemblerTables.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/X86EVEX2VEXTablesEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/X86FoldTablesEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/X86MnemonicTables.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/X86ModRMFilters.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/X86RecognizableInstr.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/WebAssemblyDisassemblerEmitter.cpp.o utils/TableGen/CMakeFiles/llvm-tblgen.dir/CTagsEmitter.cpp.o -o bin/llvm-tblgen -L/x-tools/i586-unknown-linux-gnu/lib/gcc/i586-unknown-linux-gnu/8.3.0   -L/x-tools/i586-unknown-linux-gnu/i586-unknown-linux-gnu/lib   -L/x-tools/i586-unknown-linux-gnu/i586-unknown-linux-gnu/sysroot/lib   -L/x-tools/i586-unknown-linux-gnu/i586-unknown-linux-gnu/sysroot/usr/lib -Wl,-rpath,"\$ORIGIN/../lib"  lib/libLLVMSupport.a  lib/libLLVMTableGen.a  -lpthread  lib/libLLVMTableGenGlobalISel.a  lib/libLLVMTableGen.a  lib/libLLVMSupport.a  -lrt  -ldl  -lpthread  -lm  lib/libLLVMDemangle.a && :
/usr/bin/ld: cannot find /lib/libpthread.so.0: No such file or directory
/usr/bin/ld: cannot find /usr/lib/libpthread_nonshared.a: No such file or directory
collect2: error: ld returned 1 exit status
[295/3023] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/AsmWriter.cpp.o
[296/3023] Building CXX object lib/Bitcode/Writer/CMakeFiles/LLVMBitWriter.dir/BitcodeWriter.cpp.o
[297/3023] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Globals.cpp.o
[298/3023] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/AutoUpgrade.cpp.o
---
[305/3023] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/DebugInfoMetadata.cpp.o
[306/3023] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Dominators.cpp.o
[307/3023] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Instructions.cpp.o
[308/3023] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/LLVMContextImpl.cpp.o
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit status: 1


build script failed, must exit now', /cargo/registry/src/index.crates.io-6f17d22bba15001f/cmake-0.1.48/src/lib.rs:975:5
 finished in 58.950 seconds
Build completed unsuccessfully in 0:10:00
