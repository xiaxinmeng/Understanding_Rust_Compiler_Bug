plain
-- Performing Test HAS_WERROR_GLOBAL_CTORS
-- Performing Test HAS_WERROR_GLOBAL_CTORS - Success
-- Performing Test LLVM_HAS_NOGLOBAL_CTOR_MUTEX
-- Performing Test LLVM_HAS_NOGLOBAL_CTOR_MUTEX - Success
-- Looking for __x86_64__
-- Looking for __x86_64__ - found
-- Looking for __aarch64__
-- Looking for __aarch64__ - not found
-- LLVMHello ignored -- Loadable modules not supported on this platform.
-- Targeting AArch64
-- Targeting ARM
-- Targeting BPF
---
[  1%] Creating export file for LTO
[  1%] Building C object utils/count/CMakeFiles/count.dir/count.c.o
[  1%] Building CXX object utils/PerfectShuffle/CMakeFiles/llvm-PerfectShuffle.dir/PerfectShuffle.cpp.o
Scanning dependencies of target LLVMDemangle
[  1%] Building C object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3.c.o
[  1%] Building C object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_dispatch.c.o
[  1%] Building ASM object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_sse2_x86-64_unix.S.o
[  1%] Building C object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_portable.c.o
[  1%] Building ASM object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_sse41_x86-64_unix.S.o
[  1%] Building ASM object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_avx2_x86-64_unix.S.o
[  1%] Building ASM object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_avx512_x86-64_unix.S.o
[  1%] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/Demangle.cpp.o
[  1%] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/ItaniumDemangle.cpp.o
[  2%] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangle.cpp.o
[  2%] Built target LTO_exports
---
[  4%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DeltaAlgorithm.cpp.o
[  4%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DivisionByConstantInfo.cpp.o
[  4%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DAGDeltaAlgorithm.cpp.o
[  4%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DJB.cpp.o
[  4%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86MnemonicTables.cpp.o
[  4%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86ModRMFilters.cpp.o
[  4%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86RecognizableInstr.cpp.o
[  4%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/WebAssemblyDisassemblerEmitter.cpp.o
[  4%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CTagsEmitter.cpp.o
---
[ 10%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/CodeViewError.cpp.o
[ 10%] Building CXX object lib/DebugInfo/MSF/CMakeFiles/LLVMDebugInfoMSF.dir/MSFCommon.cpp.o
[ 10%] Linking CXX executable ../../bin/not
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/JSONBackend.cpp.o
ld.lld: error: undefined symbol: llvm::sys::findProgramByName[abi:cxx11](llvm::StringRef, llvm::ArrayRef<llvm::StringRef>)
>>> referenced by not.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-ed3ac9.tmp.o:(main)

ld.lld: error: undefined symbol: llvm::WithColor::error()
>>> referenced by not.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-ed3ac9.tmp.o:(main)
>>> referenced by not.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-ed3ac9.tmp.o:(main)

ld.lld: error: undefined symbol: llvm::raw_ostream::write(char const*, unsigned long)
>>> referenced by not.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-ed3ac9.tmp.o:(main)
>>> referenced by not.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-ed3ac9.tmp.o:(main)
[ 10%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/CodeViewRecordIO.cpp.o
>>> referenced by not.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-ed3ac9.tmp.o:(main)
>>> referenced 5 more times

ld.lld: error: undefined symbol: llvm::sys::ExecuteAndWait(llvm::StringRef, llvm::ArrayRef<llvm::StringRef>, llvm::Optional<llvm::ArrayRef<llvm::StringRef> >, llvm::ArrayRef<llvm::Optional<llvm::StringRef> >, unsigned int, unsigned int, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >*, bool*, llvm::Optional<llvm::sys::ProcessStatistics>*, llvm::BitVector*)
>>> referenced by not.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-ed3ac9.tmp.o:(main)
clang-13: error: linker command failed with exit code 1 (use -v to see invocation)
make[2]: *** [bin/not] Error 1
make[1]: *** [utils/not/CMakeFiles/not.dir/all] Error 2
make[1]: *** Waiting for unfinished jobs....
[ 10%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/ContinuationRecordBuilder.cpp.o
[ 10%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/Dwarf.cpp.o
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Main.cpp.o
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Parser.cpp.o
---
[ 10%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/CVSymbolVisitor.cpp.o
[ 10%] Built target LLVMBitstreamReader
[ 10%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/MachO.cpp.o
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Record.cpp.o
ld.lld: error: undefined symbol: vtable for llvm::cl::Option
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<bool, false, llvm::cl::parser<bool> >::opt<char [7], llvm::cl::desc, llvm::cl::initializer<bool> >(char const (&) [7], llvm::cl::desc const&, llvm::cl::initializer<bool> const&))
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<bool, false, llvm::cl::parser<bool> >::~opt())
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<bool, false, llvm::cl::parser<bool> >::opt<char [10], llvm::cl::desc, llvm::cl::initializer<bool> >(char const (&) [10], llvm::cl::desc const&, llvm::cl::initializer<bool> const&))
>>> referenced 9 more times
>>> the vtable symbol may be undefined because the class is missing its key function (see https://lld.llvm.org/missingkeyfunction)

ld.lld: error: undefined symbol: llvm::cl::getGeneralCategory()
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<bool, false, llvm::cl::parser<bool> >::opt<char [7], llvm::cl::desc, llvm::cl::initializer<bool> >(char const (&) [7], llvm::cl::desc const&, llvm::cl::initializer<bool> const&))
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<bool, false, llvm::cl::parser<bool> >::opt<char [10], llvm::cl::desc, llvm::cl::initializer<bool> >(char const (&) [10], llvm::cl::desc const&, llvm::cl::initializer<bool> const&))
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >, false, llvm::cl::parser<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > > >::opt<llvm::cl::FormattingFlags, llvm::cl::desc>(llvm::cl::FormattingFlags const&, llvm::cl::desc const&))
>>> referenced 3 more times

ld.lld: error: undefined symbol: vtable for llvm::cl::opt<bool, false, llvm::cl::parser<bool> >
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<bool, false, llvm::cl::parser<bool> >::opt<char [7], llvm::cl::desc, llvm::cl::initializer<bool> >(char const (&) [7], llvm::cl::desc const&, llvm::cl::initializer<bool> const&))
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<bool, false, llvm::cl::parser<bool> >::~opt())
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<bool, false, llvm::cl::parser<bool> >::opt<char [10], llvm::cl::desc, llvm::cl::initializer<bool> >(char const (&) [10], llvm::cl::desc const&, llvm::cl::initializer<bool> const&))
>>> the vtable symbol may be undefined because the class is missing its key function (see https://lld.llvm.org/missingkeyfunction)

ld.lld: error: undefined symbol: vtable for llvm::cl::parser<bool>
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<bool, false, llvm::cl::parser<bool> >::opt<char [7], llvm::cl::desc, llvm::cl::initializer<bool> >(char const (&) [7], llvm::cl::desc const&, llvm::cl::initializer<bool> const&))
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<bool, false, llvm::cl::parser<bool> >::opt<char [10], llvm::cl::desc, llvm::cl::initializer<bool> >(char const (&) [10], llvm::cl::desc const&, llvm::cl::initializer<bool> const&))
>>> the vtable symbol may be undefined because the class is missing its key function (see https://lld.llvm.org/missingkeyfunction)

ld.lld: error: undefined symbol: llvm::cl::Option::setArgStr(llvm::StringRef)
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<bool, false, llvm::cl::parser<bool> >::opt<char [7], llvm::cl::desc, llvm::cl::initializer<bool> >(char const (&) [7], llvm::cl::desc const&, llvm::cl::initializer<bool> const&))
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<bool, false, llvm::cl::parser<bool> >::opt<char [10], llvm::cl::desc, llvm::cl::initializer<bool> >(char const (&) [10], llvm::cl::desc const&, llvm::cl::initializer<bool> const&))
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<unsigned int, false, llvm::cl::parser<unsigned int> >::opt<char [13], llvm::cl::desc, llvm::cl::initializer<int> >(char const (&) [13], llvm::cl::desc const&, llvm::cl::initializer<int> const&))
>>> referenced 1 more times

ld.lld: error: undefined symbol: llvm::cl::Option::addArgument()
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<bool, false, llvm::cl::parser<bool> >::opt<char [7], llvm::cl::desc, llvm::cl::initializer<bool> >(char const (&) [7], llvm::cl::desc const&, llvm::cl::initializer<bool> const&))
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<bool, false, llvm::cl::parser<bool> >::opt<char [10], llvm::cl::desc, llvm::cl::initializer<bool> >(char const (&) [10], llvm::cl::desc const&, llvm::cl::initializer<bool> const&))
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >, false, llvm::cl::parser<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > > >::opt<llvm::cl::FormattingFlags, llvm::cl::desc>(llvm::cl::FormattingFlags const&, llvm::cl::desc const&))
>>> referenced 2 more times

ld.lld: error: undefined symbol: llvm::SmallVectorBase<unsigned int>::grow_pod(void*, unsigned long, unsigned long)
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<bool, false, llvm::cl::parser<bool> >::opt<char [7], llvm::cl::desc, llvm::cl::initializer<bool> >(char const (&) [7], llvm::cl::desc const&, llvm::cl::initializer<bool> const&))
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<bool, false, llvm::cl::parser<bool> >::opt<char [10], llvm::cl::desc, llvm::cl::initializer<bool> >(char const (&) [10], llvm::cl::desc const&, llvm::cl::initializer<bool> const&))
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >, false, llvm::cl::parser<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > > >::opt<llvm::cl::FormattingFlags, llvm::cl::desc>(llvm::cl::FormattingFlags const&, llvm::cl::desc const&))
>>> referenced 3 more times

ld.lld: error: undefined symbol: vtable for llvm::cl::OptionValue<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > >
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >, false, llvm::cl::parser<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > > >::opt<llvm::cl::FormattingFlags, llvm::cl::desc>(llvm::cl::FormattingFlags const&, llvm::cl::desc const&))
>>> the vtable symbol may be undefined because the class is missing its key function (see https://lld.llvm.org/missingkeyfunction)

ld.lld: error: undefined symbol: vtable for llvm::cl::opt<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >, false, llvm::cl::parser<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > > >
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >, false, llvm::cl::parser<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > > >::opt<llvm::cl::FormattingFlags, llvm::cl::desc>(llvm::cl::FormattingFlags const&, llvm::cl::desc const&))
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >, false, llvm::cl::parser<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > > >::~opt())
>>> the vtable symbol may be undefined because the class is missing its key function (see https://lld.llvm.org/missingkeyfunction)

ld.lld: error: undefined symbol: vtable for llvm::cl::parser<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > >
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >, false, llvm::cl::parser<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > > >::opt<llvm::cl::FormattingFlags, llvm::cl::desc>(llvm::cl::FormattingFlags const&, llvm::cl::desc const&))
>>> the vtable symbol may be undefined because the class is missing its key function (see https://lld.llvm.org/missingkeyfunction)

ld.lld: error: undefined symbol: vtable for llvm::cl::opt<unsigned int, false, llvm::cl::parser<unsigned int> >
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<unsigned int, false, llvm::cl::parser<unsigned int> >::opt<char [13], llvm::cl::desc, llvm::cl::initializer<int> >(char const (&) [13], llvm::cl::desc const&, llvm::cl::initializer<int> const&))
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<unsigned int, false, llvm::cl::parser<unsigned int> >::~opt())
>>> the vtable symbol may be undefined because the class is missing its key function (see https://lld.llvm.org/missingkeyfunction)

ld.lld: error: undefined symbol: vtable for llvm::cl::parser<unsigned int>
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<unsigned int, false, llvm::cl::parser<unsigned int> >::opt<char [13], llvm::cl::desc, llvm::cl::initializer<int> >(char const (&) [13], llvm::cl::desc const&, llvm::cl::initializer<int> const&))
>>> the vtable symbol may be undefined because the class is missing its key function (see https://lld.llvm.org/missingkeyfunction)

ld.lld: error: undefined symbol: vtable for llvm::cl::OptionValue<llvm::cl::boolOrDefault>
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<llvm::cl::boolOrDefault, false, llvm::cl::parser<llvm::cl::boolOrDefault> >::opt<char [10], llvm::cl::desc, llvm::cl::initializer<llvm::cl::boolOrDefault> >(char const (&) [10], llvm::cl::desc const&, llvm::cl::initializer<llvm::cl::boolOrDefault> const&))
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<llvm::cl::boolOrDefault, false, llvm::cl::parser<llvm::cl::boolOrDefault> >::printOptionValue(unsigned long, bool) const)
>>> the vtable symbol may be undefined because the class is missing its key function (see https://lld.llvm.org/missingkeyfunction)

ld.lld: error: undefined symbol: vtable for llvm::cl::parser<llvm::cl::boolOrDefault>
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(llvm::cl::opt<llvm::cl::boolOrDefault, false, llvm::cl::parser<llvm::cl::boolOrDefault> >::opt<char [10], llvm::cl::desc, llvm::cl::initializer<llvm::cl::boolOrDefault> >(char const (&) [10], llvm::cl::desc const&, llvm::cl::initializer<llvm::cl::boolOrDefault> const&))
>>> the vtable symbol may be undefined because the class is missing its key function (see https://lld.llvm.org/missingkeyfunction)

ld.lld: error: undefined symbol: llvm::cl::ParseCommandLineOptions(int, char const* const*, llvm::StringRef, llvm::raw_ostream*, char const*, bool)
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(main)

ld.lld: error: undefined symbol: llvm::sys::Process::StandardOutHasColors()
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(main)

ld.lld: error: undefined symbol: llvm::MemoryBuffer::getFileOrSTDIN(llvm::Twine const&, bool, bool)
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(main)

ld.lld: error: undefined symbol: llvm::outs()
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(main)
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(main)
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(main)
>>> referenced 16 more times

ld.lld: error: undefined symbol: llvm::yaml::dumpTokens(llvm::StringRef, llvm::raw_ostream&)
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(main)

ld.lld: error: undefined symbol: llvm::yaml::Stream::Stream(llvm::StringRef, llvm::SourceMgr&, bool, std::error_code*)
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(main)
>>> referenced by YAMLBench.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-c4034a.tmp.o:(benchmark(llvm::TimerGroup&, llvm::StringRef, llvm::StringRef, llvm::StringRef))

ld.lld: error: too many errors emitted, stopping now (use -error-limit=0 to see all errors)
clang-13: error: linker command failed with exit code 1 (use -v to see invocation)
make[2]: *** [bin/yaml-bench] Error 1
make[1]: *** [utils/yaml-bench/CMakeFiles/yaml-bench.dir/all] Error 2
[ 10%] Linking CXX executable ../../bin/llvm-config
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/SetTheory.cpp.o
[ 10%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugChecksumsSubsection.cpp.o
[ 10%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugCrossExSubsection.cpp.o
---
[ 11%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/MsgPackDocument.cpp.o
[ 12%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugFrameDataSubsection.cpp.o
[ 12%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugInlineeLinesSubsection.cpp.o
[ 12%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TableGenBackendSkeleton.cpp.o
ld.lld: error: undefined symbol: llvm::sys::fs::getMainExecutable[abi:cxx11](char const*, void*)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(GetExecutablePath[abi:cxx11](char const*))
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)

ld.lld: error: undefined symbol: llvm::StringRef::lower[abi:cxx11]() const
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(ComputeLibsForComponents(std::vector<llvm::StringRef, std::allocator<llvm::StringRef> > const&, bool, bool, std::function<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > (llvm::StringRef const&)> const*, std::vector<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >, std::allocator<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > > >*, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&))

ld.lld: error: undefined symbol: llvm::StringMapImpl::FindKey(llvm::StringRef) const
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(ComputeLibsForComponents(std::vector<llvm::StringRef, std::allocator<llvm::StringRef> > const&, bool, bool, std::function<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > (llvm::StringRef const&)> const*, std::vector<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >, std::allocator<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > > >*, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&))
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(VisitComponent(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&, llvm::StringMap<AvailableComponent*, llvm::MallocAllocator> const&, std::set<AvailableComponent*, std::less<AvailableComponent*>, std::allocator<AvailableComponent*> >&, std::vector<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >, std::allocator<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > > >&, bool, bool, std::function<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > (llvm::StringRef const&)> const*, std::vector<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >, std::allocator<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > > >*, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&))

ld.lld: error: undefined symbol: llvm::deallocate_buffer(void*, unsigned long, unsigned long)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(ComputeLibsForComponents(std::vector<llvm::StringRef, std::allocator<llvm::StringRef> > const&, bool, bool, std::function<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > (llvm::StringRef const&)> const*, std::vector<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >, std::allocator<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > > >*, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&))
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(llvm::StringMap<AvailableComponent*, llvm::MallocAllocator>::~StringMap())

ld.lld: error: undefined symbol: llvm::errs()
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(ComputeLibsForComponents(std::vector<llvm::StringRef, std::allocator<llvm::StringRef> > const&, bool, bool, std::function<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > (llvm::StringRef const&)> const*, std::vector<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >, std::allocator<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > > >*, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&))
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced 8 more times

ld.lld: error: undefined symbol: llvm::SmallVectorBase<unsigned long>::grow_pod(void*, unsigned long, unsigned long)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(void llvm::SmallVectorImpl<char>::append<char const*, void>(char const*, char const*))

ld.lld: error: undefined symbol: llvm::sys::fs::make_absolute(llvm::SmallVectorImpl<char>&)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)

ld.lld: error: undefined symbol: llvm::sys::path::parent_path(llvm::StringRef, llvm::sys::path::Style)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)

ld.lld: error: undefined symbol: llvm::sys::fs::equivalent(llvm::Twine const&, llvm::Twine const&, bool&)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)

ld.lld: error: undefined symbol: llvm::sys::fs::make_absolute(llvm::Twine const&, llvm::SmallVectorImpl<char>&)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)

ld.lld: error: undefined symbol: llvm::Triple::normalize[abi:cxx11](llvm::StringRef)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)

ld.lld: error: undefined symbol: llvm::Triple::Triple(llvm::Twine const&)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)

ld.lld: error: undefined symbol: llvm::Twine::str[abi:cxx11]() const
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main::$_2::operator()[abi:cxx11](llvm::StringRef const&, bool) const)
>>> referenced 2 more times

ld.lld: error: undefined symbol: llvm::sys::fs::access(llvm::Twine const&, llvm::sys::fs::AccessMode)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(VisitComponent(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&, llvm::StringMap<AvailableComponent*, llvm::MallocAllocator> const&, std::set<AvailableComponent*, std::less<AvailableComponent*>, std::allocator<AvailableComponent*> >&, std::vector<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >, std::allocator<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > > >&, bool, bool, std::function<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > (llvm::StringRef const&)> const*, std::vector<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >, std::allocator<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > > >*, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&))

ld.lld: error: undefined symbol: llvm::outs()
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)

ld.lld: error: undefined symbol: llvm::raw_ostream::write(char const*, unsigned long)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced 43 more times

ld.lld: error: undefined symbol: llvm::raw_ostream::write(unsigned char)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced 9 more times
ld.lld: error: undefined symbol: llvm::WithColor::error(llvm::raw_ostream&, llvm::StringRef, bool)
>>> referenced by llvm-config.cpp
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(main)
>>> referenced 2 more times

ld.lld: error: undefined symbol: llvm::report_fatal_error(char const*, bool)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(VisitComponent(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&, llvm::StringMap<AvailableComponent*, llvm::MallocAllocator> const&, std::set<AvailableComponent*, std::less<AvailableComponent*>, std::allocator<AvailableComponent*> >&, std::vector<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >, std::allocator<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > > >&, bool, bool, std::function<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > (llvm::StringRef const&)> const*, std::vector<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >, std::allocator<std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > > >*, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&))

ld.lld: error: undefined symbol: llvm::StringMapImpl::LookupBucketFor(llvm::StringRef)
>>> referenced by llvm-config.cpp
>>>               /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lto.cache/Thin-b6b9e0.tmp.o:(std::pair<llvm::StringMapIterator<AvailableComponent*>, bool> llvm::StringMap<AvailableComponent*, llvm::MallocAllocator>::try_emplace<>(llvm::StringRef))

ld.lld: error: too many errors emitted, stopping now (use -error-limit=0 to see all errors)
clang-13: error: linker command failed with exit code 1 (use -v to see invocation)
make[2]: *** [bin/llvm-config] Error 1
make[1]: *** [tools/llvm-config/CMakeFiles/llvm-config.dir/all] Error 2
[ 12%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/MsgPackDocumentYAML.cpp.o
[ 12%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugStringTableSubsection.cpp.o
[ 12%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugSubsection.cpp.o
[ 12%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugSubsectionRecord.cpp.o
---
[ 13%] Linking CXX static library ../libLLVMTableGen.a
[ 13%] Built target LLVMTableGen
[ 13%] Linking CXX static library ../../libLLVMDebugInfoCodeView.a
[ 13%] Built target LLVMDebugInfoCodeView
make: *** [all] Error 2
command did not execute successfully, got: exit status: 2


build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
 finished in 55.407 seconds
Build completed unsuccessfully in 0:01:46
