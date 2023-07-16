plain
[00:03:02]  ---> f8dc110b598f
[00:03:02] Step 25/41 : RUN ./build-clang.sh
[00:03:02]  ---> Running in bec99b1054f3
[00:03:03] + source shared.sh
[00:03:03] + LLVM=032b00a5404865765cda7db3039f39d54964d8b0
[00:03:03] + LLD=3e4aa4e8671523321af51449e0569f455ef3ad43
[00:03:03] + CLANG=a6b9739069763243020f4ea6fe586bc135fde1f9
[00:03:03] + mkdir clang
[00:03:03] + cd clang
[00:03:03] + curl -L https://github.com/llvm-mirror/llvm/archive/032b00a5404865765cda7db3039f39d54964d8b0.tar.gz
[00:03:03] + tar xzf - --strip-components=1
[00:03:03]                                  Dload  Upload   Total   Spent    Left  Speed
[00:03:03] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   158    0   158    0     0    871      0 --:--:-- --:--:-- --:--:--   908
---
100 12.1M    0 12.1M    0     0  2997k      0 --:--:--  0:00:04 --:--:-- 2676k
100 12.1M    0 12.1M    0     0  2235k      0 --:--:--  0:00:05 --:--:-- 1822k
100 17.1M    0 17.1M    0     0  2992k      0 --:--:--  0:00:05 --:--:-- 2787k
[00:03:18] + mkdir -p tools/lld
[00:03:18] + curl -L https://github.com/llvm-mirror/lld/archive/3e4aa4e8671523321af51449e0569f455ef3ad43.tar.gz
[00:03:18] + tar zxf - --strip-components=1 -C tools/lld
[00:03:18]                                  Dload  Upload   Total   Spent    Left  Speed
[00:03:18] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   157    0   157    0     0    845      0 --:--:-- --:--:-- --:--:--   887
---
[00:03:18] + cd clang-build
[00:03:18] + INC=/rustroot/include
[00:03:18] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:03:18] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:03:18] + hide_output cmake .. -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:03:48] Sun Dec 23 01:56:58 UTC 2018 - building ...
[00:04:05] + hide_output make -j10
[00:04:05] + set +x
[00:04:35] Sun Dec 23 01:57:46 UTC 2018 - building ...
---
[01:04:33]  ---> 229e4276e9c7
[01:04:33] Step 32/41 : RUN ./build-perl.sh
[01:04:33]  ---> Running in 89f5bf77d7bd
[01:04:33] + source shared.sh
[01:04:33] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[01:04:33]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:04:33]                                  Dload  Upload   Total   Spent    Left  Speed
[01:04:34] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[01:58:31] [ 30%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/DbiStream.cpp.o
[01:58:31] [ 30%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/DbiStreamBuilder.cpp.o
[01:58:32] In file included from /checkout/src/llvm-emscripten/lib/DebugInfo/PDB/Native/DbiModuleDescriptorBuilder.cpp:19:
[01:58:32] In file included from /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/GSIStreamBuilder.h:14:
[01:58:32] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/GlobalsStream.h:33:3: warning: explicitly defaulted default constructor is implicitly deleted [-Wdefaulted-function-deleted]
[01:58:32]   GSIHashIterator() = default;
[01:58:32]   ^
[01:58:32] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/GlobalsStream.h:29:7: note: default constructor of 'GSIHashIterator' is implicitly deleted because base class 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const uint32_t>' (aka 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const unsigned int>') has a deleted default constructor
[01:58:32]     : public iterator_adaptor_base<
[01:58:32] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:216:3: note: explicitly defaulted function was implicitly deleted here
[01:58:32] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:216:3: note: explicitly defaulted function was implicitly deleted here
[01:58:32]   iterator_adaptor_base() = default;
[01:58:32]   ^
[01:58:32] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:214:20: note: default constructor of 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, llvm::FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const unsigned int, int, const unsigned int *, const unsigned int &, std::iterator_traits<llvm::FixedStreamArrayIterator<llvm::pdb::PSHashRecord> > >' is implicitly deleted because field 'I' has no default constructor
[01:58:32]   WrappedIteratorT I;
[01:58:32] 1 warning generated.
[01:58:32] [ 30%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/EnumTables.cpp.o
[01:58:33] [ 30%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/GlobalsStream.cpp.o
[01:58:33] [ 30%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/Hash.cpp.o
[01:58:33] [ 30%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/Hash.cpp.o
[01:58:35] [ 30%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/HashTable.cpp.o
[01:58:35] [ 30%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/InfoStream.cpp.o
[01:58:35] [ 30%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/InfoStreamBuilder.cpp.o
[01:58:35] In file included from /checkout/src/llvm-emscripten/lib/DebugInfo/PDB/Native/GlobalsStream.cpp:22:
[01:58:35] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/GlobalsStream.h:33:3: warning: explicitly defaulted default constructor is implicitly deleted [-Wdefaulted-function-deleted]
[01:58:35]   GSIHashIterator() = default;
[01:58:35]   ^
[01:58:35] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/GlobalsStream.h:29:7: note: default constructor of 'GSIHashIterator' is implicitly deleted because base class 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const uint32_t>' (aka 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const unsigned int>') has a deleted default constructor
[01:58:35]     : public iterator_adaptor_base<
[01:58:35] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:216:3: note: explicitly defaulted function was implicitly deleted here
[01:58:35] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:216:3: note: explicitly defaulted function was implicitly deleted here
[01:58:35]   iterator_adaptor_base() = default;
[01:58:35]   ^
[01:58:35] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:214:20: note: default constructor of 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, llvm::FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const unsigned int, int, const unsigned int *, const unsigned int &, std::iterator_traits<llvm::FixedStreamArrayIterator<llvm::pdb::PSHashRecord> > >' is implicitly deleted because field 'I' has no default constructor
[01:58:35]   WrappedIteratorT I;
[01:58:35] 1 warning generated.
[01:58:35] [ 30%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/ModuleDebugStream.cpp.o
[01:58:37] [ 31%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeBuiltinSymbol.cpp.o
[01:58:37] [ 31%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeCompilandSymbol.cpp.o
---
[01:58:38] [ 32%] Built target RcTableGen
[01:58:38] Scanning dependencies of target LLVMCore
[01:58:38] [ 33%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/AsmWriter.cpp.o
[01:58:39] In file included from /checkout/src/llvm-emscripten/lib/DebugInfo/PDB/Native/ModuleDebugStream.cpp:10:
[01:58:39] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/ModuleDebugStream.h:52:25: warning: explicitly defaulted move assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[01:58:39]   ModuleDebugStreamRef &operator=(ModuleDebugStreamRef &&Other) = default;
[01:58:39]                         ^
[01:58:39] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/ModuleDebugStream.h:67:30: note: move assignment operator of 'ModuleDebugStreamRef' is implicitly deleted because field 'Mod' is of reference type 'const llvm::pdb::DbiModuleDescriptor &'
[01:58:39]   const DbiModuleDescriptor &Mod;
[01:58:39] 1 warning generated.
[01:58:39] [ 33%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeEnumModules.cpp.o
[01:58:39] [ 33%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeEnumSymbol.cpp.o
[01:58:39] [ 33%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeEnumTypes.cpp.o
---
[01:58:44] [ 33%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/PDBFile.cpp.o
[01:58:44] [ 33%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/PDBFileBuilder.cpp.o
[01:58:48] [ 33%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/PDBStringTable.cpp.o
[01:58:48] In file included from /checkout/src/llvm-emscripten/lib/DebugInfo/PDB/Native/PDBFile.cpp:16:
[01:58:48] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/GlobalsStream.h:33:3: warning: explicitly defaulted default constructor is implicitly deleted [-Wdefaulted-function-deleted]
[01:58:48]   GSIHashIterator() = default;
[01:58:48]   ^
[01:58:48] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/GlobalsStream.h:29:7: note: default constructor of 'GSIHashIterator' is implicitly deleted because base class 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const uint32_t>' (aka 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const unsigned int>') has a deleted default constructor
[01:58:48]     : public iterator_adaptor_base<
[01:58:48] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:216:3: note: explicitly defaulted function was implicitly deleted here
[01:58:48] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:216:3: note: explicitly defaulted function was implicitly deleted here
[01:58:48]   iterator_adaptor_base() = default;
[01:58:48]   ^
[01:58:48] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:214:20: note: default constructor of 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, llvm::FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const unsigned int, int, const unsigned int *, const unsigned int &, std::iterator_traits<llvm::FixedStreamArrayIterator<llvm::pdb::PSHashRecord> > >' is implicitly deleted because field 'I' has no default constructor
[01:58:48]   WrappedIteratorT I;
[01:58:48] 1 warning generated.
[01:58:48] [ 33%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/PDBStringTableBuilder.cpp.o
[01:58:48] In file included from /checkout/src/llvm-emscripten/lib/DebugInfo/PDB/Native/PDBFileBuilder.cpp:18:
[01:58:48] In file included from /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/GSIStreamBuilder.h:14:
[01:58:48] In file included from /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/GSIStreamBuilder.h:14:
[01:58:48] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/GlobalsStream.h:33:3: warning: explicitly defaulted default constructor is implicitly deleted [-Wdefaulted-function-deleted]
[01:58:48]   GSIHashIterator() = default;
[01:58:48]   ^
[01:58:48] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/GlobalsStream.h:29:7: note: default constructor of 'GSIHashIterator' is implicitly deleted because base class 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const uint32_t>' (aka 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const unsigned int>') has a deleted default constructor
[01:58:48]     : public iterator_adaptor_base<
[01:58:48] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:216:3: note: explicitly defaulted function was implicitly deleted here
[01:58:48] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:216:3: note: explicitly defaulted function was implicitly deleted here
[01:58:48]   iterator_adaptor_base() = default;
[01:58:48]   ^
[01:58:48] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:214:20: note: default constructor of 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, llvm::FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const unsigned int, int, const unsigned int *, const unsigned int &, std::iterator_traits<llvm::FixedStreamArrayIterator<llvm::pdb::PSHashRecord> > >' is implicitly deleted because field 'I' has no default constructor
[01:58:48]   WrappedIteratorT I;
[01:58:48] 1 warning generated.
[01:58:48] [ 33%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/PublicsStream.cpp.o
[01:58:50] [ 33%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/GSIStreamBuilder.cpp.o
[01:58:50] [ 34%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/RawError.cpp.o
[01:58:50] [ 34%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/RawError.cpp.o
[01:58:50] In file included from /checkout/src/llvm-emscripten/lib/DebugInfo/PDB/Native/PublicsStream.cpp:25:
[01:58:50] In file included from /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/PublicsStream.h:15:
[01:58:50] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/GlobalsStream.h:33:3: warning: explicitly defaulted default constructor is implicitly deleted [-Wdefaulted-function-deleted]
[01:58:50]   GSIHashIterator() = default;
[01:58:50]   ^
[01:58:50] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/GlobalsStream.h:29:7: note: default constructor of 'GSIHashIterator' is implicitly deleted because base class 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const uint32_t>' (aka 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const unsigned int>') has a deleted default constructor
[01:58:50]     : public iterator_adaptor_base<
[01:58:50] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:216:3: note: explicitly defaulted function was implicitly deleted here
[01:58:50] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:216:3: note: explicitly defaulted function was implicitly deleted here
[01:58:50]   iterator_adaptor_base() = default;
[01:58:50]   ^
[01:58:50] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:214:20: note: default constructor of 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, llvm::FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const unsigned int, int, const unsigned int *, const unsigned int &, std::iterator_traits<llvm::FixedStreamArrayIterator<llvm::pdb::PSHashRecord> > >' is implicitly deleted because field 'I' has no default constructor
[01:58:50]   WrappedIteratorT I;
[01:58:50] 1 warning generated.
[01:58:50] [ 34%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/SymbolStream.cpp.o
[01:58:52] [ 34%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/TpiHashing.cpp.o
[01:58:52] [ 34%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Attributes.cpp.o
[01:58:52] [ 34%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Attributes.cpp.o
[01:58:53] [ 34%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/TpiStream.cpp.o
[01:58:55] [ 34%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/TpiStreamBuilder.cpp.o
[01:58:56] In file included from /checkout/src/llvm-emscripten/lib/DebugInfo/PDB/Native/GSIStreamBuilder.cpp:10:
[01:58:56] In file included from /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/GSIStreamBuilder.h:14:
[01:58:56] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/GlobalsStream.h:33:3: warning: explicitly defaulted default constructor is implicitly deleted [-Wdefaulted-function-deleted]
[01:58:56]   GSIHashIterator() = default;
[01:58:56]   ^
[01:58:56] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/GlobalsStream.h:29:7: note: default constructor of 'GSIHashIterator' is implicitly deleted because base class 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const uint32_t>' (aka 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const unsigned int>') has a deleted default constructor
[01:58:56]     : public iterator_adaptor_base<
[01:58:56] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:216:3: note: explicitly defaulted function was implicitly deleted here
[01:58:56] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:216:3: note: explicitly defaulted function was implicitly deleted here
[01:58:56]   iterator_adaptor_base() = default;
[01:58:56]   ^
[01:58:56] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:214:20: note: default constructor of 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, llvm::FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const unsigned int, int, const unsigned int *, const unsigned int &, std::iterator_traits<llvm::FixedStreamArrayIterator<llvm::pdb::PSHashRecord> > >' is implicitly deleted because field 'I' has no default constructor
[01:58:56]   WrappedIteratorT I;
[01:58:56] 1 warning generated.
[01:58:56] [ 34%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/AutoUpgrade.cpp.o
[01:58:56] [ 34%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/BasicBlock.cpp.o
[01:58:58] [ 34%] Linking CXX static library ../../libLLVMDebugInfoPDB.a
---
[02:14:53] [ 78%] Building CXX object lib/Target/JSBackend/CMakeFiles/LLVMJSBackendCodeGen.dir/Relooper.cpp.o
[02:14:54] [ 78%] Building CXX object lib/Target/JSBackend/CMakeFiles/LLVMJSBackendCodeGen.dir/RemoveLLVMAssume.cpp.o
[02:14:54] [ 78%] Building CXX object lib/Target/JSBackend/NaCl/CMakeFiles/LLVMPNaClTransforms.dir/ExpandSmallArguments.cpp.o
[02:14:56] [ 78%] Building CXX object lib/Target/JSBackend/CMakeFiles/LLVMJSBackendCodeGen.dir/SimplifyAllocas.cpp.o
[02:14:57] /checkout/src/llvm-emscripten/lib/Analysis/MemorySSA.cpp:121:3: warning: explicitly defaulted default constructor is implicitly deleted [-Wdefaulted-function-deleted]
[02:14:57]   MemoryLocOrCall() = default;
[02:14:57]   ^
[02:14:57] /checkout/src/llvm-emscripten/lib/Analysis/MemorySSA.cpp:168:23: note: default constructor of 'MemoryLocOrCall' is implicitly deleted because variant field 'CS' has a non-trivial default constructor
[02:14:57]     ImmutableCallSite CS;
[02:14:57] 1 warning generated.
[02:14:57] [ 78%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/ModuleDebugInfoPrinter.cpp.o
[02:14:57] [ 78%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/ModuleSummaryAnalysis.cpp.o
[02:14:58] [ 79%] Building CXX object lib/Target/JSBackend/NaCl/CMakeFiles/LLVMPNaClTransforms.dir/ExpandStructRegs.cpp.o
---
[02:16:07] [ 84%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/TargetTransformInfo.cpp.o
[02:16:08] [ 84%] Building CXX object lib/ProfileData/CMakeFiles/LLVMProfileData.dir/InstrProfReader.cpp.o
[02:16:10] [ 84%] Building CXX object lib/ProfileData/CMakeFiles/LLVMProfileData.dir/InstrProfWriter.cpp.o
[02:16:13] In file included from /checkout/src/llvm-emscripten/lib/ProfileData/Coverage/CoverageMapping.cpp:15:
[02:16:13] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:643:25: warning: explicitly defaulted copy assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:16:13]   LineCoverageIterator &operator=(const LineCoverageIterator &R) = default;
[02:16:13]                         ^
[02:16:13] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:663:23: note: copy assignment operator of 'LineCoverageIterator' is implicitly deleted because field 'CD' is of reference type 'const llvm::coverage::CoverageData &'
[02:16:13]   const CoverageData &CD;
[02:16:13] 1 warning generated.
[02:16:13] [ 84%] Building CXX object lib/ProfileData/Coverage/CMakeFiles/LLVMCoverage.dir/CoverageMappingWriter.cpp.o
[02:16:13] [ 84%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/Trace.cpp.o
[02:16:14] [ 84%] Building CXX object lib/ProfileData/CMakeFiles/LLVMProfileData.dir/ProfileSummaryBuilder.cpp.o
[02:16:14] [ 84%] Building CXX object lib/ProfileData/CMakeFiles/LLVMProfileData.dir/ProfileSummaryBuilder.cpp.o
[02:16:14] [ 84%] Building CXX object lib/ProfileData/CMakeFiles/LLVMProfileData.dir/SampleProf.cpp.o
[02:16:15] [ 84%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/TypeBasedAliasAnalysis.cpp.o
[02:16:17] In file included from /checkout/src/llvm-emscripten/lib/ProfileData/Coverage/CoverageMappingWriter.cpp:15:
[02:16:17] In file included from /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMappingWriter.h:20:
[02:16:17] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:643:25: warning: explicitly defaulted copy assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:16:17]   LineCoverageIterator &operator=(const LineCoverageIterator &R) = default;
[02:16:17]                         ^
[02:16:17] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:663:23: note: copy assignment operator of 'LineCoverageIterator' is implicitly deleted because field 'CD' is of reference type 'const llvm::coverage::CoverageData &'
[02:16:17]   const CoverageData &CD;
[02:16:17] 1 warning generated.
[02:16:17] [ 84%] Building CXX object lib/ProfileData/Coverage/CMakeFiles/LLVMCoverage.dir/CoverageMappingReader.cpp.o
[02:16:18] [ 84%] Building CXX object lib/ProfileData/CMakeFiles/LLVMProfileData.dir/SampleProfReader.cpp.o
[02:16:18] [ 84%] Building CXX object lib/ProfileData/CMakeFiles/LLVMProfileData.dir/SampleProfWriter.cpp.o
[02:16:18] [ 84%] Building CXX object lib/ProfileData/CMakeFiles/LLVMProfileData.dir/SampleProfWriter.cpp.o
[02:16:19] [ 84%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/TypeMetadataUtils.cpp.o
[02:16:23] [ 84%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/ScopedNoAliasAA.cpp.o
[02:16:24] [ 84%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/ValueLattice.cpp.o
[02:16:24] In file included from /checkout/src/llvm-emscripten/lib/ProfileData/Coverage/CoverageMappingReader.cpp:15:
[02:16:24] In file included from /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMappingReader.h:20:
[02:16:24] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:643:25: warning: explicitly defaulted copy assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:16:24]   LineCoverageIterator &operator=(const LineCoverageIterator &R) = default;
[02:16:24]                         ^
[02:16:24] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:663:23: note: copy assignment operator of 'LineCoverageIterator' is implicitly deleted because field 'CD' is of reference type 'const llvm::coverage::CoverageData &'
[02:16:24]   const CoverageData &CD;
[02:16:24] 1 warning generated.
[02:16:24] [ 85%] Linking CXX static library ../libLLVMProfileData.a
[02:16:24] [ 85%] Linking CXX static library ../../libLLVMCoverage.a
[02:16:24] [ 85%] Built target LLVMProfileData
---
[02:16:56] [ 87%] Built target llvm-cxxdump
[02:16:56] Scanning dependencies of target llvm-dis
[02:16:56] [ 87%] Building CXX object tools/llvm-dis/CMakeFiles/llvm-dis.dir/llvm-dis.cpp.o
[02:16:57] [ 87%] Building CXX object tools/llvm-diff/CMakeFiles/llvm-diff.dir/DiffConsumer.cpp.o
[02:16:59] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CodeCoverage.cpp:16:
[02:16:59] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageFilters.h:17:
[02:16:59] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageSummaryInfo.h:18:
[02:16:59] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:643:25: warning: explicitly defaulted copy assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:16:59]   LineCoverageIterator &operator=(const LineCoverageIterator &R) = default;
[02:16:59]                         ^
[02:16:59] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:663:23: note: copy assignment operator of 'LineCoverageIterator' is implicitly deleted because field 'CD' is of reference type 'const llvm::coverage::CoverageData &'
[02:16:59]   const CoverageData &CD;
[02:16:59] 1 warning generated.
[02:16:59] [ 87%] Building CXX object tools/llvm-cov/CMakeFiles/llvm-cov.dir/CoverageExporterJson.cpp.o
[02:17:00] [ 87%] Building CXX object tools/llvm-diff/CMakeFiles/llvm-diff.dir/DiffLog.cpp.o
[02:17:01] [ 87%] Building CXX object tools/llvm-diff/CMakeFiles/llvm-diff.dir/DifferenceEngine.cpp.o
[02:17:01] [ 87%] Building CXX object tools/llvm-diff/CMakeFiles/llvm-diff.dir/DifferenceEngine.cpp.o
[02:17:02] [ 87%] Linking CXX executable ../../bin/llvm-dis
[02:17:02] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageExporterJson.cpp:44:
[02:17:02] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageReport.h:17:
[02:17:02] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageFilters.h:17:
[02:17:02] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageSummaryInfo.h:18:
[02:17:02] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:643:25: warning: explicitly defaulted copy assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:17:02]   LineCoverageIterator &operator=(const LineCoverageIterator &R) = default;
[02:17:02]                         ^
[02:17:02] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:663:23: note: copy assignment operator of 'LineCoverageIterator' is implicitly deleted because field 'CD' is of reference type 'const llvm::coverage::CoverageData &'
[02:17:02]   const CoverageData &CD;
[02:17:02] 1 warning generated.
[02:17:02] [ 87%] Building CXX object tools/llvm-cov/CMakeFiles/llvm-cov.dir/CoverageFilters.cpp.o
[02:17:02] [ 87%] Built target llvm-dis
[02:17:02] Scanning dependencies of target llvm-dwarfdump
[02:17:02] Scanning dependencies of target llvm-dwarfdump
[02:17:02] [ 87%] Building CXX object tools/llvm-dwarfdump/CMakeFiles/llvm-dwarfdump.dir/Statistics.cpp.o
[02:17:04] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageFilters.cpp:14:
[02:17:04] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageFilters.h:17:
[02:17:04] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageSummaryInfo.h:18:
[02:17:04] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:643:25: warning: explicitly defaulted copy assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:17:04]   LineCoverageIterator &operator=(const LineCoverageIterator &R) = default;
[02:17:04]                         ^
[02:17:04] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:663:23: note: copy assignment operator of 'LineCoverageIterator' is implicitly deleted because field 'CD' is of reference type 'const llvm::coverage::CoverageData &'
[02:17:04]   const CoverageData &CD;
[02:17:04] 1 warning generated.
[02:17:04] [ 87%] Building CXX object tools/llvm-cov/CMakeFiles/llvm-cov.dir/CoverageReport.cpp.o
[02:17:05] [ 87%] Building CXX object tools/llvm-dwarfdump/CMakeFiles/llvm-dwarfdump.dir/llvm-dwarfdump.cpp.o
[02:17:07] [ 87%] Linking CXX executable ../../bin/llvm-diff
[02:17:07] [ 87%] Linking CXX executable ../../bin/llvm-diff
[02:17:07] [ 87%] Built target llvm-diff
[02:17:07] Scanning dependencies of target llvm-dwp
[02:17:07] [ 87%] Building CXX object tools/llvm-dwp/CMakeFiles/llvm-dwp.dir/llvm-dwp.cpp.o
[02:17:08] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageReport.cpp:14:
[02:17:08] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageReport.h:17:
[02:17:08] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageFilters.h:17:
[02:17:08] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageSummaryInfo.h:18:
[02:17:08] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:643:25: warning: explicitly defaulted copy assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:17:08]   LineCoverageIterator &operator=(const LineCoverageIterator &R) = default;
[02:17:08]                         ^
[02:17:08] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:663:23: note: copy assignment operator of 'LineCoverageIterator' is implicitly deleted because field 'CD' is of reference type 'const llvm::coverage::CoverageData &'
[02:17:08]   const CoverageData &CD;
[02:17:08] 1 warning generated.
[02:17:08] [ 87%] Building CXX object tools/llvm-cov/CMakeFiles/llvm-cov.dir/CoverageSummaryInfo.cpp.o
[02:17:09] [ 87%] Linking CXX static library ../libLLVMPasses.a
[02:17:09] [ 87%] Built target LLVMPasses
[02:17:09] [ 87%] Built target LLVMPasses
[02:17:09] Scanning dependencies of target llvm-extract
[02:17:09] [ 87%] Building CXX object tools/llvm-extract/CMakeFiles/llvm-extract.dir/llvm-extract.cpp.o
[02:17:11] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageSummaryInfo.cpp:15:
[02:17:11] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageSummaryInfo.h:18:
[02:17:11] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:643:25: warning: explicitly defaulted copy assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:17:11]   LineCoverageIterator &operator=(const LineCoverageIterator &R) = default;
[02:17:11]                         ^
[02:17:11] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:663:23: note: copy assignment operator of 'LineCoverageIterator' is implicitly deleted because field 'CD' is of reference type 'const llvm::coverage::CoverageData &'
[02:17:11]   const CoverageData &CD;
[02:17:11] 1 warning generated.
[02:17:11] [ 87%] Building CXX object tools/llvm-cov/CMakeFiles/llvm-cov.dir/SourceCoverageView.cpp.o
[02:17:11] [ 87%] Linking CXX executable ../../bin/llvm-dwarfdump
[02:17:12] [ 87%] Built target llvm-dwarfdump
---
[02:17:14] [ 88%] Built target llvm-extract
[02:17:14] Scanning dependencies of target llvm-link
[02:17:14] [ 88%] Building CXX object tools/llvm-link/CMakeFiles/llvm-link.dir/llvm-link.cpp.o
[02:17:15] [ 88%] Building CXX object tools/llvm-dwp/CMakeFiles/llvm-dwp.dir/DWPError.cpp.o
[02:17:15] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/SourceCoverageView.cpp:14:
[02:17:15] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/SourceCoverageView.h:18:
[02:17:15] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageSummaryInfo.h:18:
[02:17:15] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:643:25: warning: explicitly defaulted copy assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:17:15]   LineCoverageIterator &operator=(const LineCoverageIterator &R) = default;
[02:17:15]                         ^
[02:17:15] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:663:23: note: copy assignment operator of 'LineCoverageIterator' is implicitly deleted because field 'CD' is of reference type 'const llvm::coverage::CoverageData &'
[02:17:15]   const CoverageData &CD;
[02:17:15] 1 warning generated.
[02:17:15] [ 88%] Building CXX object tools/llvm-cov/CMakeFiles/llvm-cov.dir/SourceCoverageViewHTML.cpp.o
[02:17:16] [ 88%] Linking CXX executable ../../bin/llvm-dwp
[02:17:17] [ 88%] Built target llvm-dwp
[02:17:17] [ 88%] Built target llvm-dwp
[02:17:18] Scanning dependencies of target llvm-lto2
[02:17:18] [ 88%] Building CXX object tools/llvm-lto2/CMakeFiles/llvm-lto2.dir/llvm-lto2.cpp.o
[02:17:19] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/SourceCoverageViewHTML.cpp:14:
[02:17:19] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageReport.h:17:
[02:17:19] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageFilters.h:17:
[02:17:19] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageSummaryInfo.h:18:
[02:17:19] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:643:25: warning: explicitly defaulted copy assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:17:19]   LineCoverageIterator &operator=(const LineCoverageIterator &R) = default;
[02:17:19]                         ^
[02:17:19] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:663:23: note: copy assignment operator of 'LineCoverageIterator' is implicitly deleted because field 'CD' is of reference type 'const llvm::coverage::CoverageData &'
[02:17:19]   const CoverageData &CD;
[02:17:19] 1 warning generated.
[02:17:19] [ 88%] Building CXX object tools/llvm-cov/CMakeFiles/llvm-cov.dir/SourceCoverageViewText.cpp.o
[02:17:20] [ 88%] Linking CXX executable ../../bin/llvm-link
[02:17:20] [ 88%] Built target llvm-link
[02:17:20] [ 88%] Built target llvm-link
[02:17:20] [ 88%] Building CXX object tools/llvm-cov/CMakeFiles/llvm-cov.dir/TestingSupport.cpp.o
[02:17:22] [ 88%] Linking CXX executable ../../bin/llvm-isel-fuzzer
[02:17:22] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/SourceCoverageViewText.cpp:14:
[02:17:22] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageReport.h:17:
[02:17:22] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageFilters.h:17:
[02:17:22] In file included from /checkout/src/llvm-emscripten/tools/llvm-cov/CoverageSummaryInfo.h:18:
[02:17:22] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:643:25: warning: explicitly defaulted copy assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:17:22]   LineCoverageIterator &operator=(const LineCoverageIterator &R) = default;
[02:17:22]                         ^
[02:17:22] /checkout/src/llvm-emscripten/include/llvm/ProfileData/Coverage/CoverageMapping.h:663:23: note: copy assignment operator of 'LineCoverageIterator' is implicitly deleted because field 'CD' is of reference type 'const llvm::coverage::CoverageData &'
[02:17:22]   const CoverageData &CD;
[02:17:22] 1 warning generated.
[02:17:22] Scanning dependencies of target llvm-mc
[02:17:22] [ 88%] Building CXX object tools/llvm-mc/CMakeFiles/llvm-mc.dir/llvm-mc.cpp.o
[02:17:23] [ 88%] Built target llvm-isel-fuzzer
---
[02:17:49] [ 89%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/BytesOutputStyle.cpp.o
[02:17:49] [ 89%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/Diff.cpp.o
[02:17:51] [ 89%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceFileWriter.cpp.o
[02:17:55] In file included from /checkout/src/llvm-emscripten/tools/llvm-pdbutil/BytesOutputStyle.cpp:22:
[02:17:55] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/ModuleDebugStream.h:52:25: warning: explicitly defaulted move assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:17:55]   ModuleDebugStreamRef &operator=(ModuleDebugStreamRef &&Other) = default;
[02:17:55]                         ^
[02:17:55] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/ModuleDebugStream.h:67:30: note: move assignment operator of 'ModuleDebugStreamRef' is implicitly deleted because field 'Mod' is of reference type 'const llvm::pdb::DbiModuleDescriptor &'
[02:17:55]   const DbiModuleDescriptor &Mod;
[02:17:55] 1 warning generated.
[02:17:55] [ 89%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/DiffPrinter.cpp.o
[02:17:55] [ 89%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/DumpOutputStyle.cpp.o
[02:17:56] [ 89%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptParser.cpp.o
[02:17:56] [ 89%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptParser.cpp.o
[02:17:57] [ 89%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/InputFile.cpp.o
[02:18:02] [ 89%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptStmt.cpp.o
[02:18:02] In file included from /checkout/src/llvm-emscripten/tools/llvm-pdbutil/InputFile.cpp:10:
[02:18:02] In file included from /checkout/src/llvm-emscripten/tools/llvm-pdbutil/InputFile.h:19:
[02:18:02] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/ModuleDebugStream.h:52:25: warning: explicitly defaulted move assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:18:02]   ModuleDebugStreamRef &operator=(ModuleDebugStreamRef &&Other) = default;
[02:18:02]                         ^
[02:18:02] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/ModuleDebugStream.h:67:30: note: move assignment operator of 'ModuleDebugStreamRef' is implicitly deleted because field 'Mod' is of reference type 'const llvm::pdb::DbiModuleDescriptor &'
[02:18:02]   const DbiModuleDescriptor &Mod;
[02:18:02] 1 warning generated.
[02:18:02] [ 89%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/llvm-pdbutil.cpp.o
[02:18:05] [ 89%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptToken.cpp.o
[02:18:07] [ 89%] Linking CXX executable ../../bin/llvm-objdump
---
[02:18:07] Scanning dependencies of target llvm-readobj
[02:18:07] [ 91%] Building CXX object tools/llvm-readobj/CMakeFiles/llvm-readobj.dir/ARMWinEHPrinter.cpp.o
[02:18:10] [ 91%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/LinePrinter.cpp.o
[02:18:11] [ 91%] Building CXX object tools/llvm-readobj/CMakeFiles/llvm-readobj.dir/COFFDumper.cpp.o
[02:18:12] In file included from /checkout/src/llvm-emscripten/tools/llvm-pdbutil/DumpOutputStyle.cpp:13:
[02:18:12] In file included from /checkout/src/llvm-emscripten/tools/llvm-pdbutil/InputFile.h:19:
[02:18:12] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/ModuleDebugStream.h:52:25: warning: explicitly defaulted move assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:18:12]   ModuleDebugStreamRef &operator=(ModuleDebugStreamRef &&Other) = default;
[02:18:12]                         ^
[02:18:12] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/ModuleDebugStream.h:67:30: note: move assignment operator of 'ModuleDebugStreamRef' is implicitly deleted because field 'Mod' is of reference type 'const llvm::pdb::DbiModuleDescriptor &'
[02:18:12]   const DbiModuleDescriptor &Mod;
[02:18:12] In file included from /checkout/src/llvm-emscripten/tools/llvm-pdbutil/DumpOutputStyle.cpp:40:
[02:18:12] In file included from /checkout/src/llvm-emscripten/tools/llvm-pdbutil/DumpOutputStyle.cpp:40:
[02:18:12] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/GlobalsStream.h:33:3: warning: explicitly defaulted default constructor is implicitly deleted [-Wdefaulted-function-deleted]
[02:18:12]   GSIHashIterator() = default;
[02:18:12]   ^
[02:18:12] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/GlobalsStream.h:29:7: note: default constructor of 'GSIHashIterator' is implicitly deleted because base class 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const uint32_t>' (aka 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const unsigned int>') has a deleted default constructor
[02:18:12]     : public iterator_adaptor_base<
[02:18:12] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:216:3: note: explicitly defaulted function was implicitly deleted here
[02:18:12] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:216:3: note: explicitly defaulted function was implicitly deleted here
[02:18:12]   iterator_adaptor_base() = default;
[02:18:12]   ^
[02:18:12] /checkout/src/llvm-emscripten/include/llvm/ADT/iterator.h:214:20: note: default constructor of 'iterator_adaptor_base<llvm::pdb::GSIHashIterator, llvm::FixedStreamArrayIterator<llvm::pdb::PSHashRecord>, std::random_access_iterator_tag, const unsigned int, int, const unsigned int *, const unsigned int &, std::iterator_traits<llvm::FixedStreamArrayIterator<llvm::pdb::PSHashRecord> > >' is implicitly deleted because field 'I' has no default constructor
[02:18:12]   WrappedIteratorT I;
[02:18:12] 2 warnings generated.
[02:18:12] [ 91%] Building CXX object tools/llvm-readobj/CMakeFiles/llvm-readobj.dir/COFFImportDumper.cpp.o
[02:18:14] In file included from /checkout/src/llvm-emscripten/tools/llvm-pdbutil/llvm-pdbutil.cpp:20:
[02:18:14] In file included from /checkout/src/llvm-emscripten/tools/llvm-pdbutil/InputFile.h:19:
[02:18:14] In file included from /checkout/src/llvm-emscripten/tools/llvm-pdbutil/InputFile.h:19:
[02:18:14] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/ModuleDebugStream.h:52:25: warning: explicitly defaulted move assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:18:14]   ModuleDebugStreamRef &operator=(ModuleDebugStreamRef &&Other) = default;
[02:18:14]                         ^
[02:18:14] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/ModuleDebugStream.h:67:30: note: move assignment operator of 'ModuleDebugStreamRef' is implicitly deleted because field 'Mod' is of reference type 'const llvm::pdb::DbiModuleDescriptor &'
[02:18:14]   const DbiModuleDescriptor &Mod;
[02:18:14] 1 warning generated.
[02:18:14] [ 91%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/MinimalSymbolDumper.cpp.o
[02:18:14] [ 91%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/MinimalTypeDumper.cpp.o
[02:18:15] [ 91%] Building CXX object tools/llvm-readobj/CMakeFiles/llvm-readobj.dir/ELFDumper.cpp.o
---
[02:18:43] Scanning dependencies of target llvm-rtdyld
[02:18:43] [ 92%] Building CXX object tools/llvm-rtdyld/CMakeFiles/llvm-rtdyld.dir/llvm-rtdyld.cpp.o
[02:18:43] Scanning dependencies of target llvm-size
[02:18:43] [ 92%] Building CXX object tools/llvm-size/CMakeFiles/llvm-size.dir/llvm-size.cpp.o
[02:18:43] In file included from /checkout/src/llvm-emscripten/tools/llvm-pdbutil/YAMLOutputStyle.cpp:22:
[02:18:43] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/ModuleDebugStream.h:52:25: warning: explicitly defaulted move assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:18:43]   ModuleDebugStreamRef &operator=(ModuleDebugStreamRef &&Other) = default;
[02:18:43]                         ^
[02:18:43] /checkout/src/llvm-emscripten/include/llvm/DebugInfo/PDB/Native/ModuleDebugStream.h:67:30: note: move assignment operator of 'ModuleDebugStreamRef' is implicitly deleted because field 'Mod' is of reference type 'const llvm::pdb::DbiModuleDescriptor &'
[02:18:43]   const DbiModuleDescriptor &Mod;
[02:18:43] 1 warning generated.
[02:18:43] [ 92%] Linking CXX executable ../../bin/llvm-pdbutil
[02:18:44] [ 92%] Built target llvm-pdbutil
[02:18:44] Scanning dependencies of target llvm-special-case-list-fuzzer
---
[02:20:12] [ 99%] Building CXX object tools/lli/ChildTarget/CMakeFiles/lli-child-target.dir/ChildTarget.cpp.o
[02:20:14] Scanning dependencies of target llvm-as
[02:20:14] [ 99%] Building CXX object tools/llvm-as/CMakeFiles/llvm-as.dir/llvm-as.cpp.o
[02:20:15] In file included from /checkout/src/llvm-emscripten/tools/lli/lli.cpp:28:
[02:20:15] /checkout/src/llvm-emscripten/include/llvm/ExecutionEngine/Orc/OrcRemoteTargetClient.h:74:5: warning: explicitly defaulted move assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:20:15]     operator=(RemoteRTDyldMemoryManager &&) = default;
[02:20:15]     ^
[02:20:15] /checkout/src/llvm-emscripten/include/llvm/ExecutionEngine/Orc/OrcRemoteTargetClient.h:315:28: note: move assignment operator of 'RemoteRTDyldMemoryManager' is implicitly deleted because field 'Client' is of reference type 'llvm::orc::remote::OrcRemoteTargetClient &'
[02:20:15]     OrcRemoteTargetClient &Client;
[02:20:15] 1 warning generated.
[02:20:15] [ 99%] Building CXX object tools/lli/CMakeFiles/lli.dir/OrcLazyJIT.cpp.o
[02:20:17] [ 99%] Linking CXX executable ../../bin/bugpoint
[02:20:17] [ 99%] Linking CXX executable ../../bin/llvm-as
---
[02:57:16] Dist compiler docs (i686-unknown-linux-gnu)
[02:57:16]  skipping - compiler docs disabled
[02:57:16] Dist rustc stage2 (i686-unknown-linux-gnu)

Broadcast message from root@travis-job-5ae89b89-8396-46fb-8372-70160a90268a
 (unknown) at 4:52 ...
The system is going down for power off NOW!
