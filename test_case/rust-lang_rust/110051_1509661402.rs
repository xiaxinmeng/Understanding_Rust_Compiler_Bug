plain
[ 17%] Built target WebAssemblyCommonTableGen
[ 17%] Building X86GenAsmWriter1.inc...
[ 17%] Building X86GenCallingConv.inc...
[ 17%] Building AVRGenAsmMatcher.inc...
PLEASE submit a bug report to https://github.com/llvm/llvm-project/issues/ and include the crash backtrace.
Stack dump:
0. Program arguments: ../../../bin/llvm-tblgen -gen-pseudo-lowering -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/AArch64 -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/AArch64/AArch64.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/AArch64/AArch64GenMCPseudoLowering.inc
Stack dump without symbol names (ensure you have llvm-symbolizer in your PATH or set the environment var `LLVM_SYMBOLIZER_PATH` to point to it):
0  llvm-tblgen              0x000000010a5dd6ca llvm::sys::PrintStackTrace(llvm::raw_ostream&, int) + 42
1  llvm-tblgen              0x000000010a5dc868 llvm::sys::RunSignalHandlers() + 248
2  llvm-tblgen              0x000000010a5ddd50 SignalHandler(int) + 288
3  libsystem_platform.dylib 0x00007ff809f2adfd _sigtramp + 29
4  libsystem_platform.dylib 0x00007fe8627058a8 _sigtramp + 18446744006474705608
5  llvm-tblgen              0x000000010a5fcca3 std::__1::pair<llvm::StringMapIterator<llvm::StringInit*>, bool> llvm::StringMap<llvm::StringInit*, llvm::BumpPtrAllocatorImpl<llvm::MallocAllocator, 4096ul, 4096ul, 128ul>&>::try_emplace<llvm::StringInit*>(llvm::StringRef, llvm::StringInit*&&) + 35
6  llvm-tblgen              0x000000010a5e7c6d llvm::StringInit::get(llvm::RecordKeeper&, llvm::StringRef, llvm::StringInit::StringFormat) + 77
[ 17%] Building AVRGenAsmWriter.inc...
7  llvm-tblgen              0x000000010a6117fb llvm::TGParser::ParseSimpleValue(llvm::Record*, llvm::RecTy*, llvm::TGParser::IDParseMode) + 1787
8  llvm-tblgen              0x000000010a60a2de llvm::TGParser::ParseValue(llvm::Record*, llvm::RecTy*, llvm::TGParser::IDParseMode) + 62
9  llvm-tblgen              0x000000010a60b4e5 llvm::TGParser::ParseTemplateArgValueList(llvm::SmallVectorImpl<llvm::Init*>&, llvm::Record*, llvm::Record*) + 197
10 llvm-tblgen              0x000000010a60b3e8 llvm::TGParser::ParseSubClassReference(llvm::Record*, bool) + 168
11 llvm-tblgen              0x000000010a617a78 llvm::TGParser::ParseDefm(llvm::MultiClass*) + 312
12 llvm-tblgen              0x000000010a616e27 llvm::TGParser::ParseTopLevelLet(llvm::MultiClass*) + 1111
13 llvm-tblgen              0x000000010a618b02 llvm::TGParser::ParseFile() + 82
14 llvm-tblgen              0x000000010a5e3221 llvm::TableGenMain(char const*, bool (*)(llvm::raw_ostream&, llvm::RecordKeeper&)) + 721
15 llvm-tblgen              0x000000010a57ad19 main + 137
16 dyld                     0x00000001133ff52e start + 462
16 dyld                     0x00000001133ff52e start + 462
/bin/sh: line 1: 56093 Segmentation fault: 11  ../../../bin/llvm-tblgen -gen-pseudo-lowering -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/AArch64 -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/AArch64/AArch64.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/AArch64/AArch64GenMCPseudoLowering.inc
make[2]: *** [lib/Target/AArch64/AArch64GenMCPseudoLowering.inc] Error 139
make[2]: *** Waiting for unfinished jobs....
[ 17%] Building CXX object tools/remarks-shlib/CMakeFiles/Remarks.dir/libremarks.cpp.o
[ 17%] Building M68kGenRegisterInfo.inc...
[ 17%] Linking CXX shared library ../../lib/libRemarks.dylib
[ 17%] Building AVRGenDAGISel.inc...
---
[ 17%] Building AVRGenSubtargetInfo.inc...
[ 17%] Building M68kGenMCCodeEmitter.inc...
[ 17%] Building X86GenDisassemblerTables.inc...
[ 17%] Building CXX object lib/TargetParser/CMakeFiles/LLVMTargetParser.dir/CSKYTargetParser.cpp.o
PLEASE submit a bug report to https://github.com/llvm/llvm-project/issues/ and include the crash backtrace.
Stack dump:
0. Program arguments: ../../../bin/llvm-tblgen -gen-asm-writer -asmwriternum=1 -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/X86 -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/X86/X86.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/X86/X86GenAsmWriter1.inc
Stack dump without symbol names (ensure you have llvm-symbolizer in your PATH or set the environment var `LLVM_SYMBOLIZER_PATH` to point to it):
0  llvm-tblgen              0x0000000100ec06ca llvm::sys::PrintStackTrace(llvm::raw_ostream&, int) + 42
1  llvm-tblgen              0x0000000100ebf868 llvm::sys::RunSignalHandlers() + 248
2  llvm-tblgen              0x0000000100ec0d50 SignalHandler(int) + 288
3  libsystem_platform.dylib 0x00007ff809f2adfd _sigtramp + 29
4  libsystem_platform.dylib 0x00007ff7bf2b48e0 _sigtramp + 18446744072454970112
5  libsystem_c.dylib        0x00007ff809e60d24 abort + 123
6  libsystem_malloc.dylib   0x00007ff809d3e357 has_default_zone0 + 0
7  libsystem_malloc.dylib   0x00007ff809d4152b malloc_report + 151
8  llvm-tblgen              0x0000000100cefda9 llvm::Record::~Record() + 121
9  llvm-tblgen              0x0000000100ee1e9f std::__1::__tree<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::__map_value_compare<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::less<void>, true>, std::__1::allocator<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>>>::destroy(std::__1::__tree_node<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, void*>*) + 63
10 llvm-tblgen              0x0000000100ee1e7a std::__1::__tree<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::__map_value_compare<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::less<void>, true>, std::__1::allocator<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>>>::destroy(std::__1::__tree_node<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, void*>*) + 26
11 llvm-tblgen              0x0000000100ee1e7a std::__1::__tree<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::__map_value_compare<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::less<void>, true>, std::__1::allocator<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>>>::destroy(std::__1::__tree_node<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, void*>*) + 26
12 llvm-tblgen              0x0000000100ee1e86 std::__1::__tree<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::__map_value_compare<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::less<void>, true>, std::__1::allocator<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>>>::destroy(std::__1::__tree_node<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, void*>*) + 38
13 llvm-tblgen              0x0000000100ee1e7a std::__1::__tree<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::__map_value_compare<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::less<void>, true>, std::__1::allocator<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>>>::destroy(std::__1::__tree_node<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, void*>*) + 26
14 llvm-tblgen              0x0000000100ee1e7a std::__1::__tree<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::__map_value_compare<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::less<void>, true>, std::__1::allocator<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>>>::destroy(std::__1::__tree_node<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, void*>*) + 26
15 llvm-tblgen              0x0000000100ee1e7a std::__1::__tree<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::__map_value_compare<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::less<void>, true>, std::__1::allocator<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>>>::destroy(std::__1::__tree_node<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, void*>*) + 26
16 llvm-tblgen              0x0000000100ee1e86 std::__1::__tree<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::__map_value_compare<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::less<void>, true>, std::__1::allocator<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>>>::destroy(std::__1::__tree_node<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, void*>*) + 38
17 llvm-tblgen              0x0000000100ee1e86 std::__1::__tree<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::__map_value_compare<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::less<void>, true>, std::__1::allocator<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>>>::destroy(std::__1::__tree_node<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, void*>*) + 38
18 llvm-tblgen              0x0000000100ee1e86 std::__1::__tree<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::__map_value_compare<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::less<void>, true>, std::__1::allocator<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>>>::destroy(std::__1::__tree_node<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, void*>*) + 38
19 llvm-tblgen              0x0000000100ee1e86 std::__1::__tree<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::__map_value_compare<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::less<void>, true>, std::__1::allocator<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>>>::destroy(std::__1::__tree_node<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, void*>*) + 38
20 llvm-tblgen              0x0000000100ee1e86 std::__1::__tree<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::__map_value_compare<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::less<void>, true>, std::__1::allocator<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>>>::destroy(std::__1::__tree_node<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, void*>*) + 38
21 llvm-tblgen              0x0000000100ee1e7a std::__1::__tree<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::__map_value_compare<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::less<void>, true>, std::__1::allocator<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>>>::destroy(std::__1::__tree_node<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, void*>*) + 26
22 llvm-tblgen              0x0000000100ee1e7a std::__1::__tree<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::__map_value_compare<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::less<void>, true>, std::__1::allocator<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>>>::destroy(std::__1::__tree_node<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, void*>*) + 26
23 llvm-tblgen              0x0000000100ee1e7a std::__1::__tree<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::__map_value_compare<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::less<void>, true>, std::__1::allocator<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>>>::destroy(std::__1::__tree_node<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, void*>*) + 26
24 llvm-tblgen              0x0000000100ee1e7a std::__1::__tree<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::__map_value_compare<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::less<void>, true>, std::__1::allocator<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>>>::destroy(std::__1::__tree_node<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, void*>*) + 26
25 llvm-tblgen              0x0000000100ee1e7a std::__1::__tree<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::__map_value_compare<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, std::__1::less<void>, true>, std::__1::allocator<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>>>::destroy(std::__1::__tree_node<std::__1::__value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>>, void*>*) + 26
26 llvm-tblgen              0x0000000100eda66e llvm::RecordKeeper::~RecordKeeper() + 190
27 llvm-tblgen              0x0000000100ec6d63 llvm::TableGenMain(char const*, bool (*)(llvm::raw_ostream&, llvm::RecordKeeper&)) + 3603
28 llvm-tblgen              0x0000000100e5dd19 main + 137
29 dyld                     0x000000010bbd852e start + 462
/bin/sh: line 1: 56123 Abort trap: 6           ../../../bin/llvm-tblgen -gen-asm-writer -asmwriternum=1 -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/X86 -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/X86/X86.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/X86/X86GenAsmWriter1.inc
make[2]: *** [lib/Target/X86/X86GenAsmWriter1.inc] Error 134
make[2]: *** Deleting file `lib/Target/X86/X86GenAsmWriter1.inc'
make[2]: *** Deleting file `lib/Target/X86/X86GenAsmWriter1.inc'
make[2]: *** Waiting for unfinished jobs....
[ 17%] Building CXX object lib/Target/ARM/Utils/CMakeFiles/LLVMARMUtils.dir/ARMBaseInfo.cpp.o
[ 17%] Built target AVRCommonTableGen
[ 17%] Building M68kGenMCPseudoLowering.inc...
[ 17%] Building M68kGenDAGISel.inc...
[ 17%] Building M68kGenDAGISel.inc...
[ 17%] Building CXX object lib/TargetParser/CMakeFiles/LLVMTargetParser.dir/LoongArchTargetParser.cpp.o
[ 17%] Building CXX object lib/TargetParser/CMakeFiles/LLVMTargetParser.dir/RISCVTargetParser.cpp.o
[ 17%] Building M68kGenCallingConv.inc...
[ 17%] Linking CXX static library ../../../libLLVMARMUtils.a
[ 17%] Building CXX object lib/TargetParser/CMakeFiles/LLVMTargetParser.dir/TargetParser.cpp.o
[ 17%] Built target LLVMARMUtils
[ 17%] Building M68kGenAsmWriter.inc...
PLEASE submit a bug report to https://github.com/llvm/llvm-project/issues/ and include the crash backtrace.
Stack dump:
[ 17%] Building M68kGenAsmMatcher.inc...
[ 17%] Building M68kGenAsmMatcher.inc...
0. Program arguments: ../../../bin/llvm-tblgen -gen-disassembler -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/X86 -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/X86/X86.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/X86/X86GenDisassemblerTables.inc
[ 17%] Building CXX object lib/TargetParser/CMakeFiles/LLVMTargetParser.dir/X86TargetParser.cpp.o
Stack dump without symbol names (ensure you have llvm-symbolizer in your PATH or set the environment var `LLVM_SYMBOLIZER_PATH` to point to it):
0  llvm-tblgen              0x000000010e3a16ca llvm::sys::PrintStackTrace(llvm::raw_ostream&, int) + 42
1  llvm-tblgen              0x000000010e3a0868 llvm::sys::RunSignalHandlers() + 248
2  llvm-tblgen              0x000000010e3a1d50 SignalHandler(int) + 288
3  libsystem_platform.dylib 0x00007ff809f2adfd _sigtramp + 29
4  libsystem_malloc.dylib   0x00007ff809d351c4 free_tiny + 446
5  llvm-tblgen              0x000000010e3b5309 llvm::VarInit::resolveReferences(llvm::Resolver&) const + 25
6  llvm-tblgen              0x000000010e3ab5f8 llvm::BitsInit::resolveReferences(llvm::Resolver&) const + 312
7  llvm-tblgen              0x000000010e3b2701 llvm::TernOpInit::resolveReferences(llvm::Resolver&) const + 33
8  llvm-tblgen              0x000000010e3ad8e7 llvm::UnOpInit::resolveReferences(llvm::Resolver&) const + 23
9  llvm-tblgen              0x000000010e3ab5f8 llvm::BitsInit::resolveReferences(llvm::Resolver&) const + 312
10 llvm-tblgen              0x000000010e3bc33c llvm::RecordResolver::resolve(llvm::Init*) + 428
11 llvm-tblgen              0x000000010e3b5309 llvm::VarInit::resolveReferences(llvm::Resolver&) const + 25
12 llvm-tblgen              0x000000010e3ab5f8 llvm::BitsInit::resolveReferences(llvm::Resolver&) const + 312
13 llvm-tblgen              0x000000010e3b68c5 llvm::Record::resolveReferences(llvm::Resolver&, llvm::RecordVal const*) + 133
14 llvm-tblgen              0x000000010e3b6ccf llvm::Record::resolveReferences(llvm::Init*) + 95
15 llvm-tblgen              0x000000010e3cda67 llvm::TGParser::addDefOne(std::__1::unique_ptr<llvm::Record, std::__1::default_delete<llvm::Record>>) + 199
16 llvm-tblgen              0x000000010e3cd1d2 llvm::TGParser::addEntry(llvm::RecordsEntry) + 290
17 llvm-tblgen              0x000000010e3dc8dd llvm::TGParser::ParseDefm(llvm::MultiClass*) + 3997
18 llvm-tblgen              0x000000010e3dcb02 llvm::TGParser::ParseFile() + 82
19 llvm-tblgen              0x000000010e3a7221 llvm::TableGenMain(char const*, bool (*)(llvm::raw_ostream&, llvm::RecordKeeper&)) + 721
20 llvm-tblgen              0x000000010e33ed19 main + 137
21 dyld                     0x0000000117f9352e start + 462
/bin/sh: line 1: 56347 Segmentation fault: 11  ../../../bin/llvm-tblgen -gen-disassembler -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/X86 -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/X86/X86.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/X86/X86GenDisassemblerTables.inc
make[2]: *** [lib/Target/X86/X86GenDisassemblerTables.inc] Error 139
[ 17%] Building M68kGenDisassemblerTable.inc...
PLEASE submit a bug report to https://github.com/llvm/llvm-project/issues/ and include the crash backtrace.
Stack dump:
0. Program arguments: ../../../bin/llvm-tblgen -gen-asm-writer -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/M68k -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/M68k/M68k.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/M68k/M68kGenAsmWriter.inc
Stack dump without symbol names (ensure you have llvm-symbolizer in your PATH or set the environment var `LLVM_SYMBOLIZER_PATH` to point to it):
0  llvm-tblgen              0x00000001045176ca llvm::sys::PrintStackTrace(llvm::raw_ostream&, int) + 42
1  llvm-tblgen              0x0000000104516868 llvm::sys::RunSignalHandlers() + 248
2  llvm-tblgen              0x0000000104517d50 SignalHandler(int) + 288
3  libsystem_platform.dylib 0x00007ff809f2adfd _sigtramp + 29
4  libsystem_platform.dylib 0x000000000000000f _sigtramp + 18446603370414035503
5  llvm-tblgen              0x0000000104531ba9 llvm::RecordKeeper::getAllDerivedDefinitions(llvm::StringRef) const + 73
6  llvm-tblgen              0x00000001043228c0 llvm::CodeGenHwModes::CodeGenHwModes(llvm::RecordKeeper&) + 96
7  llvm-tblgen              0x0000000104366b07 llvm::CodeGenTarget::CodeGenTarget(llvm::RecordKeeper&) + 71
8  llvm-tblgen              0x00000001042d44c9 llvm::EmitAsmWriter(llvm::RecordKeeper&, llvm::raw_ostream&) + 89
9  llvm-tblgen              0x00000001044b4e02 (anonymous namespace)::LLVMTableGenMain(llvm::raw_ostream&, llvm::RecordKeeper&) + 178
10 llvm-tblgen              0x000000010451d2dc llvm::TableGenMain(char const*, bool (*)(llvm::raw_ostream&, llvm::RecordKeeper&)) + 908
11 llvm-tblgen              0x00000001044b4d19 main + 137
12 dyld                     0x000000010cc9852e start + 462
/bin/sh: line 1: 56404 Segmentation fault: 11  ../../../bin/llvm-tblgen -gen-asm-writer -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/M68k -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/M68k/M68k.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/M68k/M68kGenAsmWriter.inc
make[2]: *** [lib/Target/M68k/M68kGenAsmWriter.inc] Error 139
make[2]: *** Waiting for unfinished jobs....
Included from /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/M68k/M68k.td:87:
Included from /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/M68k/M68kInstrInfo.td:782:
/Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/M68k/M68kInstrArithmetic.td:352:7: error: Record `ADDX16dd' does not have a field named `isMoveImm'!

  def NAME#SZ#"dd"  : MxBiArOp_R_RRX<MN, NODE, !cast<MxType>("MxType"#SZ#"d"), CMD>;
      ^
Included from /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/M68k/M68k.td:87:
Included from /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/M68k/M68kInstrInfo.td:782:
/Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/M68k/M68kInstrArithmetic.td:358:13: note: instantiated from multiclass
defm ADDX : MxBiArOp_RFF<"addx", MxAddX, 1, 0xD>;
            ^
make[2]: *** [lib/Target/M68k/M68kGenAsmMatcher.inc] Error 1
make[1]: *** [lib/Target/M68k/CMakeFiles/M68kCommonTableGen.dir/all] Error 2
make[1]: *** Waiting for unfinished jobs....
[ 17%] Built target LLVMTargetParser
[ 17%] Built target LLVMTargetParser
make[1]: *** [lib/Target/X86/CMakeFiles/X86CommonTableGen.dir/all] Error 2
make[1]: *** [lib/Target/AArch64/CMakeFiles/AArch64CommonTableGen.dir/all] Error 2
PLEASE submit a bug report to https://github.com/llvm/llvm-project/issues/ and include the crash backtrace.
Stack dump:
0. Program arguments: ../../../bin/llvm-tblgen -gen-global-isel -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV/RISCV.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/RISCV/RISCVGenGlobalISel.inc
Stack dump without symbol names (ensure you have llvm-symbolizer in your PATH or set the environment var `LLVM_SYMBOLIZER_PATH` to point to it):
0  llvm-tblgen              0x00000001046b16ca llvm::sys::PrintStackTrace(llvm::raw_ostream&, int) + 42
1  llvm-tblgen              0x00000001046b0868 llvm::sys::RunSignalHandlers() + 248
2  llvm-tblgen              0x00000001046b1d50 SignalHandler(int) + 288
3  libsystem_platform.dylib 0x00007ff809f2adfd _sigtramp + 29
4  libsystem_platform.dylib 0x00007fc24ccc0008 _sigtramp + 18446743842902856232
5  llvm-tblgen              0x00000001044b25f6 getInstructionsInTree(llvm::TreePatternNode*, llvm::SmallVectorImpl<llvm::Record*>&) + 54
6  llvm-tblgen              0x00000001044acdd2 llvm::CodeGenDAGPatterns::InferInstructionFlags() + 210
7  llvm-tblgen              0x00000001044a8ba5 llvm::CodeGenDAGPatterns::CodeGenDAGPatterns(llvm::RecordKeeper&, std::__1::function<void (llvm::TreePattern*)>) + 933
8  llvm-tblgen              0x00000001045832a9 llvm::EmitGlobalISel(llvm::RecordKeeper&, llvm::raw_ostream&) + 73
9  llvm-tblgen              0x000000010464f11e (anonymous namespace)::LLVMTableGenMain(llvm::raw_ostream&, llvm::RecordKeeper&) + 974
10 llvm-tblgen              0x00000001046b72dc llvm::TableGenMain(char const*, bool (*)(llvm::raw_ostream&, llvm::RecordKeeper&)) + 908
11 llvm-tblgen              0x000000010464ed19 main + 137
12 dyld                     0x000000010e4b852e start + 462
/bin/sh: line 1: 55797 Segmentation fault: 11  ../../../bin/llvm-tblgen -gen-global-isel -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV -I/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/include -I/Users/runner/work/rust/rust/src/llvm-project/llvm/include -I /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target -no-warn-on-unused-template-args /Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/RISCV/RISCV.td --write-if-changed -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib/Target/RISCV/RISCVGenGlobalISel.inc
make[2]: *** [lib/Target/RISCV/RISCVGenGlobalISel.inc] Error 139
make[2]: *** Waiting for unfinished jobs....
make[1]: *** [lib/Target/RISCV/CMakeFiles/RISCVCommonTableGen.dir/all] Error 2
make: *** [all] Error 2
command did not execute successfully, got: exit status: 2


build script failed, must exit now', /Users/runner/.cargo/registry/src/index.crates.io-6f17d22bba15001f/cmake-0.1.48/src/lib.rs:975:5
 finished in 118.748 seconds
Build completed unsuccessfully in 0:03:27
