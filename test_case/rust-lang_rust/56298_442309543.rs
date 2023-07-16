plain
[00:02:26]       Memory: 8 GB
[00:02:26]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:26]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:26]       SMC Version (system): 2.8f0
[00:02:26]       Serial Number (system): VM+phhiNjJi0
[00:02:26] 
[00:02:26] hw.ncpu: 4
[00:02:26] hw.byteorder: 1234
[00:02:26] hw.memsize: 8589934592
---
[00:23:09] -- Looking for sys/resource.h - found
[00:23:09] -- Clang version: 8.0.0
[00:23:09] -- Performing Test CXX_SUPPORTS_NO_NESTED_ANON_TYPES_FLAG
[00:23:10] -- Performing Test CXX_SUPPORTS_NO_NESTED_ANON_TYPES_FLAG - Success
[00:23:10] -- Could NOT find Z3 (missing: Z3_LIBRARIES Z3_INCLUDE_DIR) (Required is exact version "4.7.1")
[00:23:12] -- Performing Test CXX_SUPPORTS_NO_DEPRECATED_DECLARATIONS
[00:23:12] -- Performing Test CXX_SUPPORTS_NO_DEPRECATED_DECLARATIONS - Success
[00:23:12] -- Performing Test CXX_SUPPORTS_NO_UNKNOWN_PRAGMAS
[00:23:12] -- Performing Test CXX_SUPPORTS_NO_UNKNOWN_PRAGMAS - Success
---
[00:57:07] [ 66%] Building CXX object tools/clang/lib/StaticAnalyzer/Checkers/CMakeFiles/clangStaticAnalyzerCheckers.dir/DynamicTypePropagation.cpp.o
[00:57:08] [ 66%] Building CXX object tools/lldb/source/Core/CMakeFiles/lldbCore.dir/Highlighter.cpp.o
[00:57:13] [ 66%] Building CXX object tools/lldb/source/Core/CMakeFiles/lldbCore.dir/IOHandler.cpp.o
[00:57:19] [ 66%] Building CXX object tools/lldb/source/Commands/CMakeFiles/lldbCommands.dir/CommandObjectVersion.cpp.o
[00:57:19] [ 66%] Building CXX object tools/clang/lib/StaticAnalyzer/Core/CMakeFiles/clangStaticAnalyzerCore.dir/SarifDiagnostics.cpp.o
[00:57:25] [ 66%] Building CXX object tools/lldb/source/Core/CMakeFiles/lldbCore.dir/Listener.cpp.o
[00:57:30] [ 66%] Building CXX object tools/lldb/source/Core/CMakeFiles/lldbCore.dir/Mangled.cpp.o
[00:57:30] [ 66%] Building CXX object tools/clang/lib/StaticAnalyzer/Core/CMakeFiles/clangStaticAnalyzerCore.dir/SimpleConstraintManager.cpp.o
[00:57:34] [ 66%] Building CXX object tools/lldb/source/Commands/CMakeFiles/lldbCommands.dir/CommandObjectWatchpointCommand.cpp.o
---
[01:00:23] [ 69%] Building CXX object tools/lldb/source/Host/CMakeFiles/lldbHost.dir/common/NativeWatchpointList.cpp.o
[01:00:24] [ 69%] Building CXX object tools/lldb/source/Host/macosx/objcxx/CMakeFiles/lldbHostMacOSXObjCXX.dir/HostInfoMacOSX.mm.o
[01:00:25] [ 69%] Building CXX object tools/lldb/source/Host/macosx/objcxx/CMakeFiles/lldbHostMacOSXObjCXX.dir/HostThreadMacOSX.mm.o
[01:00:25] [ 69%] Building CXX object tools/lldb/source/Host/CMakeFiles/lldbHost.dir/common/NativeProcessProtocol.cpp.o
[01:00:27] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Expression/IRMemoryMap.cpp:280:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:00:27]   case eAllocationPolicyHostOnly:
[01:00:27]   ^
[01:00:27] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Expression/IRMemoryMap.cpp:280:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:00:27]   case eAllocationPolicyHostOnly:
[01:00:27]   ^
[01:00:27]   LLVM_FALLTHROUGH; 
[01:00:27] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Expression/IRMemoryMap.cpp:280:3: note: insert 'break;' to avoid fall-through
[01:00:27]   case eAllocationPolicyHostOnly:
[01:00:27]   break; 
[01:00:27] 1 warning generated.
[01:00:27] [ 69%] Building CXX object tools/lldb/source/Expression/CMakeFiles/lldbExpression.dir/LLVMUserExpression.cpp.o
[01:00:29] [ 69%] Linking CXX static library ../../../../../../lib/liblldbHostMacOSXObjCXX.a
---
[01:00:40] [ 69%] Building CXX object tools/lldb/source/Host/CMakeFiles/lldbHost.dir/common/PipeBase.cpp.o
[01:00:40] Scanning dependencies of target lldbInitialization
[01:00:40] [ 69%] Building CXX object tools/lldb/source/Initialization/CMakeFiles/lldbInitialization.dir/SystemInitializerCommon.cpp.o
[01:00:41] In file included from /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Initialization/SystemInitializerCommon.cpp:12:
[01:00:41] In file included from /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Instruction/ARM/EmulateInstructionARM.h:13:
[01:00:41] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:00:41]   case COND_EQ:
[01:00:41]   ^
[01:00:41] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: note: insert '[[clang::fallthrough]];' to silence this warning
[01:00:41]   case COND_EQ:
[01:00:41]   ^
[01:00:41]   [[clang::fallthrough]]; 
[01:00:41] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: note: insert 'break;' to avoid fall-through
[01:00:41]   case COND_EQ:
[01:00:41]   break; 
[01:00:41] 1 warning generated.
[01:00:41] [ 69%] Building CXX object tools/lldb/source/Initialization/CMakeFiles/lldbInitialization.dir/SystemInitializer.cpp.o
[01:00:41] [ 70%] Building CXX object tools/lldb/source/Initialization/CMakeFiles/lldbInitialization.dir/SystemLifetimeManager.cpp.o
---
[01:02:08] [ 70%] Building CXX object tools/clang/lib/StaticAnalyzer/Checkers/CMakeFiles/clangStaticAnalyzerCheckers.dir/MPI-Checker/MPIFunctionClassifier.cpp.o
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1695:8: warning: unused variable 'starts' [-Wunused-variable]
[01:02:11]   bool starts = StartsTerm();
[01:02:11]        ^
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:11]   ^
[01:02:11]   ^
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<left_shift<lldb_private::Scalar>, false>' requested here
[01:02:11]         ^
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1868:18: note: expanded from macro 'BINARY_OPERATORS'
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1868:18: note: expanded from macro 'BINARY_OPERATORS'
[01:02:11]   DEFINE(LSH, 9, BINOP(LSH, left_shift))                        \
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1834:30: note: expanded from macro 'BINOP'
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1834:30: note: expanded from macro 'BINOP'
[01:02:11]   RustBinaryExpression< Tag, BinaryOperation< What<Scalar>, false > >
[01:02:11]                              ^
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:11]   ^
[01:02:11]   ^
[01:02:11]   LLVM_FALLTHROUGH; 
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:11]   ^
[01:02:11]   break; 
[01:02:11]   break; 
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:11]   ^
[01:02:11]   ^
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<right_shift<lldb_private::Scalar>, false>' requested here
[01:02:11]         ^
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1869:18: note: expanded from macro 'BINARY_OPERATORS'
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1869:18: note: expanded from macro 'BINARY_OPERATORS'
[01:02:11]   DEFINE(RSH, 9, BINOP(RSH, right_shift))                       \
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1834:30: note: expanded from macro 'BINOP'
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1834:30: note: expanded from macro 'BINOP'
[01:02:11]   RustBinaryExpression< Tag, BinaryOperation< What<Scalar>, false > >
[01:02:11]                              ^
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:11]   ^
[01:02:11]   ^
[01:02:11]   LLVM_FALLTHROUGH; 
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:11]   ^
[01:02:11]   break; 
[01:02:11]   break; 
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:11]   default:
[01:02:11]   ^
[01:02:11]   ^
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<std::__1::plus<lldb_private::Scalar>, true>' requested here
[01:02:11]         ^
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1870:22: note: expanded from macro 'BINARY_OPERATORS'
[01:02:11] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1870:22: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12]   DEFINE(PLUS_EQ, 1, ASSIGN(PLUS_EQ, std::plus))                \
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12]   RustAssignExpression< Tag, BinaryOperation< What<Scalar>, true > >
[01:02:12]                              ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:12]   ^
[01:02:12]   ^
[01:02:12]   LLVM_FALLTHROUGH; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:12]   ^
[01:02:12]   break; 
[01:02:12]   break; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:12]   ^
[01:02:12]   ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<std::__1::minus<lldb_private::Scalar>, true>' requested here
[01:02:12]         ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1871:23: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1871:23: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12]   DEFINE(MINUS_EQ, 1, ASSIGN(MINUS_EQ, std::minus))             \
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12]   RustAssignExpression< Tag, BinaryOperation< What<Scalar>, true > >
[01:02:12]                              ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:12]   ^
[01:02:12]   ^
[01:02:12]   LLVM_FALLTHROUGH; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:12]   ^
[01:02:12]   break; 
[01:02:12]   break; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:12]   ^
[01:02:12]   ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<std::__1::divides<lldb_private::Scalar>, true>' requested here
[01:02:12]         ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1872:23: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1872:23: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12]   DEFINE(SLASH_EQ, 1, ASSIGN(SLASH_EQ, std::divides))           \
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12]   RustAssignExpression< Tag, BinaryOperation< What<Scalar>, true > >
[01:02:12]                              ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:12]   ^
[01:02:12]   ^
[01:02:12]   LLVM_FALLTHROUGH; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:12]   ^
[01:02:12]   break; 
[01:02:12]   break; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:12]   ^
[01:02:12]   ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<std::__1::multiplies<lldb_private::Scalar>, true>' requested here
[01:02:12]         ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1873:22: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1873:22: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12]   DEFINE(STAR_EQ, 1, ASSIGN(STAR_EQ, std::multiplies))          \
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12]   RustAssignExpression< Tag, BinaryOperation< What<Scalar>, true > >
[01:02:12]                              ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:12]   ^
[01:02:12]   ^
[01:02:12]   LLVM_FALLTHROUGH; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:12]   ^
[01:02:12]   break; 
[01:02:12]   break; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:12]   ^
[01:02:12]   ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<std::__1::modulus<lldb_private::Scalar>, true>' requested here
[01:02:12]         ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1874:25: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1874:25: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12]   DEFINE(PERCENT_EQ, 1, ASSIGN(PERCENT_EQ, std::modulus))       \
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12]   RustAssignExpression< Tag, BinaryOperation< What<Scalar>, true > >
[01:02:12]                              ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:12]   ^
[01:02:12]   ^
[01:02:12]   LLVM_FALLTHROUGH; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:12]   ^
[01:02:12]   break; 
[01:02:12]   break; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:12]   ^
[01:02:12]   ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<right_shift<lldb_private::Scalar>, true>' requested here
[01:02:12]         ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1875:21: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1875:21: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12]   DEFINE(RSH_EQ, 1, ASSIGN(RSH_EQ, right_shift))                \
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12]   RustAssignExpression< Tag, BinaryOperation< What<Scalar>, true > >
[01:02:12]                              ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:12]   ^
[01:02:12]   ^
[01:02:12]   LLVM_FALLTHROUGH; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:12]   ^
[01:02:12]   break; 
[01:02:12]   break; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:12]   ^
[01:02:12]   ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<left_shift<lldb_private::Scalar>, true>' requested here
[01:02:12]         ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1876:21: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1876:21: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12]   DEFINE(LSH_EQ, 1, ASSIGN(LSH_EQ, left_shift))                 \
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12]   RustAssignExpression< Tag, BinaryOperation< What<Scalar>, true > >
[01:02:12]                              ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:12]   ^
[01:02:12]   ^
[01:02:12]   LLVM_FALLTHROUGH; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:12]   ^
[01:02:12]   break; 
[01:02:12]   break; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:12]   ^
[01:02:12]   ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<std::__1::bit_and<lldb_private::Scalar>, true>' requested here
[01:02:12]         ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1877:21: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1877:21: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12]   DEFINE(AND_EQ, 1, ASSIGN(AND_EQ, std::bit_and))               \
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12]   RustAssignExpression< Tag, BinaryOperation< What<Scalar>, true > >
[01:02:12]                              ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:12]   ^
[01:02:12]   ^
[01:02:12]   LLVM_FALLTHROUGH; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:12]   ^
[01:02:12]   break; 
[01:02:12]   break; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:12]   ^
[01:02:12]   ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<std::__1::bit_or<lldb_private::Scalar>, true>' requested here
[01:02:12]         ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1878:20: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1878:20: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12]   DEFINE(OR_EQ, 1, ASSIGN(OR_EQ, std::bit_or))                  \
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12]   RustAssignExpression< Tag, BinaryOperation< What<Scalar>, true > >
[01:02:12]                              ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:12]   ^
[01:02:12]   ^
[01:02:12]   LLVM_FALLTHROUGH; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:12]   ^
[01:02:12]   break; 
[01:02:12]   break; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:12]   ^
[01:02:12]   ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<std::__1::bit_xor<lldb_private::Scalar>, true>' requested here
[01:02:12]         ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1879:21: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1879:21: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12]   DEFINE(XOR_EQ, 1, ASSIGN(XOR_EQ, std::bit_xor))               \
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1838:30: note: expanded from macro 'ASSIGN'
[01:02:12]   RustAssignExpression< Tag, BinaryOperation< What<Scalar>, true > >
[01:02:12]                              ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:12]   ^
[01:02:12]   ^
[01:02:12]   LLVM_FALLTHROUGH; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:12]   ^
[01:02:12]   break; 
[01:02:12]   break; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:12]   ^
[01:02:12]   ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<std::__1::bit_or<lldb_private::Scalar>, false>' requested here
[01:02:12]         ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1880:18: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1880:18: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12]   DEFINE('|', 6, BINOP('|', std::bit_or))                       \
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1834:30: note: expanded from macro 'BINOP'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1834:30: note: expanded from macro 'BINOP'
[01:02:12]   RustBinaryExpression< Tag, BinaryOperation< What<Scalar>, false > >
[01:02:12]                              ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:12]   ^
[01:02:12]   ^
[01:02:12]   LLVM_FALLTHROUGH; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:12]   ^
[01:02:12]   break; 
[01:02:12]   break; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:12]   ^
[01:02:12]   ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<std::__1::bit_and<lldb_private::Scalar>, false>' requested here
[01:02:12]         ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1881:18: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1881:18: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12]   DEFINE('&', 8, BINOP('&', std::bit_and))                      \
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1834:30: note: expanded from macro 'BINOP'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1834:30: note: expanded from macro 'BINOP'
[01:02:12]   RustBinaryExpression< Tag, BinaryOperation< What<Scalar>, false > >
[01:02:12]                              ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:12]   ^
[01:02:12]   ^
[01:02:12]   LLVM_FALLTHROUGH; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:12]   ^
[01:02:12]   break; 
[01:02:12]   break; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:12]   ^
[01:02:12]   ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<std::__1::plus<lldb_private::Scalar>, false>' requested here
[01:02:12]         ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1885:19: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1885:19: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12]   DEFINE('+', 10, BINOP('+', std::plus))                        \
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1834:30: note: expanded from macro 'BINOP'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1834:30: note: expanded from macro 'BINOP'
[01:02:12]   RustBinaryExpression< Tag, BinaryOperation< What<Scalar>, false > >
[01:02:12]                              ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:12]   ^
[01:02:12]   ^
[01:02:12]   LLVM_FALLTHROUGH; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:12]   ^
[01:02:12]   break; 
[01:02:12]   break; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:12]   ^
[01:02:12]   ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<std::__1::minus<lldb_private::Scalar>, false>' requested here
[01:02:12]         ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1886:19: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1886:19: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12]   DEFINE('-', 10, BINOP('-', std::minus))                       \
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1834:30: note: expanded from macro 'BINOP'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1834:30: note: expanded from macro 'BINOP'
[01:02:12]   RustBinaryExpression< Tag, BinaryOperation< What<Scalar>, false > >
[01:02:12]                              ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:12]   ^
[01:02:12]   ^
[01:02:12]   LLVM_FALLTHROUGH; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:12]   ^
[01:02:12]   break; 
[01:02:12]   break; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:12]   ^
[01:02:12]   ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<std::__1::multiplies<lldb_private::Scalar>, false>' requested here
[01:02:12]         ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1887:19: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1887:19: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12]   DEFINE('*', 11, BINOP('*', std::multiplies))                  \
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1834:30: note: expanded from macro 'BINOP'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1834:30: note: expanded from macro 'BINOP'
[01:02:12]   RustBinaryExpression< Tag, BinaryOperation< What<Scalar>, false > >
[01:02:12]                              ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:12]   ^
[01:02:12]   ^
[01:02:12]   LLVM_FALLTHROUGH; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:12]   ^
[01:02:12]   break; 
[01:02:12]   break; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:12]   ^
[01:02:12]   ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<std::__1::divides<lldb_private::Scalar>, false>' requested here
[01:02:12]         ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1888:19: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1888:19: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12]   DEFINE('/', 11, BINOP('/', std::divides))                     \
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1834:30: note: expanded from macro 'BINOP'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1834:30: note: expanded from macro 'BINOP'
[01:02:12]   RustBinaryExpression< Tag, BinaryOperation< What<Scalar>, false > >
[01:02:12]                              ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:12]   ^
[01:02:12]   ^
[01:02:12]   LLVM_FALLTHROUGH; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:12]   ^
[01:02:12]   break; 
[01:02:12]   break; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:12]   ^
[01:02:12]   ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1959:9: note: in instantiation of function template specialization 'lldb_private::rust::BinaryOperation<std::__1::modulus<lldb_private::Scalar>, false>' requested here
[01:02:12]         ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1889:19: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1889:19: note: expanded from macro 'BINARY_OPERATORS'
[01:02:12]   DEFINE('%', 11, BINOP('%', std::modulus))
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1834:30: note: expanded from macro 'BINOP'
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:1834:30: note: expanded from macro 'BINOP'
[01:02:12]   RustBinaryExpression< Tag, BinaryOperation< What<Scalar>, false > >
[01:02:12]                              ^
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:02:12]   ^
[01:02:12]   ^
[01:02:12]   LLVM_FALLTHROUGH; 
[01:02:12] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ExpressionParser/Rust/RustParse.cpp:296:3: note: insert 'break;' to avoid fall-through
[01:02:12]   ^
[01:02:12]   break; 
[01:02:12] 20 warnings generated.
[01:02:15] [ 70%] Building CXX object tools/lldb/source/Interpreter/CMakeFiles/lldbInterpreter.dir/OptionValue.cpp.o
---
[01:02:27] [ 71%] Building CXX object tools/lldb/source/Plugins/Instruction/ARM/CMakeFiles/lldbPluginInstructionARM.dir/EmulateInstructionARM.cpp.o
[01:02:30] [ 71%] Building CXX object tools/lldb/source/Interpreter/CMakeFiles/lldbInterpreter.dir/OptionValueBoolean.cpp.o
[01:02:30] [ 71%] Building CXX object tools/clang/lib/StaticAnalyzer/Checkers/CMakeFiles/clangStaticAnalyzerCheckers.dir/NoReturnFunctionChecker.cpp.o
[01:02:32] [ 71%] Building CXX object tools/lldb/source/Interpreter/CMakeFiles/lldbInterpreter.dir/OptionValueChar.cpp.o
[01:02:33] In file included from /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/Instruction/ARM/EmulateInstructionARM.cpp:12:
[01:02:33] In file included from /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/Instruction/ARM/EmulateInstructionARM.h:13:
[01:02:33] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:02:33]   case COND_EQ:
[01:02:33]   ^
[01:02:33] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: note: insert '[[clang::fallthrough]];' to silence this warning
[01:02:33]   case COND_EQ:
[01:02:33]   ^
[01:02:33]   [[clang::fallthrough]]; 
[01:02:33] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: note: insert 'break;' to avoid fall-through
[01:02:33]   case COND_EQ:
[01:02:33]   break; 
[01:02:33] 1 warning generated.
[01:02:33] [ 71%] Building CXX object tools/lldb/source/Plugins/Instruction/ARM/CMakeFiles/lldbPluginInstructionARM.dir/EmulationStateARM.cpp.o
[01:02:34] [ 71%] Building CXX object tools/lldb/source/Interpreter/CMakeFiles/lldbInterpreter.dir/OptionValueDictionary.cpp.o
---
[01:05:16] [ 72%] Built target lldbPluginAppleObjCRuntime
[01:05:16] [ 72%] Building CXX object tools/clang/lib/StaticAnalyzer/Checkers/CMakeFiles/clangStaticAnalyzerCheckers.dir/PointerArithChecker.cpp.o
[01:05:20] [ 72%] Building CXX object tools/lldb/source/Plugins/Language/CPlusPlus/CMakeFiles/lldbPluginCPlusPlusLanguage.dir/LibStdcppUniquePointer.cpp.o
[01:05:21] [ 72%] Building CXX object tools/lldb/source/Plugins/Language/ObjC/CMakeFiles/lldbPluginObjCLanguage.dir/NSString.cpp.o
[01:05:24] [ 72%] Building CXX object tools/lldb/source/Plugins/Language/CPlusPlus/CMakeFiles/lldbPluginCPlusPlusLanguage.dir/MSVCUndecoratedNameParser.cpp.o
[01:05:25] [ 72%] Building CXX object tools/lldb/source/Plugins/LanguageRuntime/Rust/CMakeFiles/lldbPluginLanguageRuntimeRust.dir/RustLanguageRuntime.cpp.o
[01:05:25] [ 72%] Linking CXX static library ../../../../../../lib/liblldbPluginCPlusPlusLanguage.a
[01:05:25] [ 72%] Built target lldbPluginCPlusPlusLanguage
[01:05:25] Scanning dependencies of target lldbPluginObjectContainerBSDArchive
---
[01:06:07]       case llvm::Triple::UnknownArch:
[01:06:07]            ^~~~~~~~~~~~~~~~~~~~~~~~~
[01:06:07] 1 warning generated.
[01:06:07] [ 73%] Building CXX object tools/clang/lib/StaticAnalyzer/Checkers/CMakeFiles/clangStaticAnalyzerCheckers.dir/RunLoopAutoreleaseLeakChecker.cpp.o
[01:06:08] [ 73%] Building CXX object tools/lldb/source/Plugins/Platform/MacOSX/CMakeFiles/lldbPluginPlatformMacOSX.dir/PlatformRemoteAppleBridge.cpp.o
[01:06:12] [ 73%] Building CXX object tools/lldb/source/Plugins/Process/gdb-remote/CMakeFiles/lldbPluginProcessGDBRemote.dir/GDBRemoteCommunicationServer.cpp.o
[01:06:13] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/Platform/MacOSX/PlatformRemoteAppleBridge.cpp:101:12: warning: comparison of two values with different enumeration types in switch statement ('llvm::Triple::VendorType' and 'llvm::Triple::ArchType') [-Wenum-compare-switch]
[01:06:13]            ^~~~~~~~~~~~~~~~~~~~~~~~~
[01:06:13] 1 warning generated.
[01:06:13] [ 73%] Building CXX object tools/lldb/source/Plugins/Platform/MacOSX/CMakeFiles/lldbPluginPlatformMacOSX.dir/PlatformAppleSimulator.cpp.o
[01:06:14] Scanning dependencies of target lldbPluginProcessUtility
---
[01:07:13] [ 75%] Building CXX object tools/clang/lib/StaticAnalyzer/Checkers/CMakeFiles/clangStaticAnalyzerCheckers.dir/TestAfterDivZeroChecker.cpp.o
[01:07:14] [ 75%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextLLDB.cpp.o
[01:07:15] [ 75%] Linking CXX static library ../../../../../../lib/liblldbPluginProcessGDBRemote.a
[01:07:15] [ 75%] Built target lldbPluginProcessGDBRemote
[01:07:15] Scanning dependencies of target lldbPluginSymbolFileNativePDB
[01:07:15] [ 75%] Building CXX object tools/lldb/source/Plugins/SymbolFile/NativePDB/CMakeFiles/lldbPluginSymbolFileNativePDB.dir/CompileUnitIndex.cpp.o
[01:07:18] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/SymbolFile/DWARF/DWARFASTParserRust.cpp:266:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:07:18]   case DW_TAG_pointer_type:
[01:07:18]   ^
[01:07:18] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/SymbolFile/DWARF/DWARFASTParserRust.cpp:266:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:07:18]   case DW_TAG_pointer_type:
[01:07:18]   ^
[01:07:18]   LLVM_FALLTHROUGH; 
[01:07:18] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/SymbolFile/DWARF/DWARFASTParserRust.cpp:266:3: note: insert 'break;' to avoid fall-through
[01:07:18]   case DW_TAG_pointer_type:
[01:07:18]   break; 
[01:07:18] 1 warning generated.
[01:07:18] [ 75%] Building CXX object tools/lldb/source/Plugins/SymbolFile/DWARF/CMakeFiles/lldbPluginSymbolFileDWARF.dir/DWARFAttribute.cpp.o
[01:07:18] [ 75%] Building CXX object tools/lldb/source/Plugins/SymbolFile/DWARF/CMakeFiles/lldbPluginSymbolFileDWARF.dir/DWARFAttribute.cpp.o
[01:07:20] [ 75%] Building CXX object tools/lldb/source/Plugins/SymbolFile/NativePDB/CMakeFiles/lldbPluginSymbolFileNativePDB.dir/PdbIndex.cpp.o
[01:07:22] [ 75%] Building CXX object tools/lldb/source/Plugins/SymbolFile/DWARF/CMakeFiles/lldbPluginSymbolFileDWARF.dir/DWARFBaseDIE.cpp.o
[01:07:24] [ 75%] Building CXX object tools/clang/lib/StaticAnalyzer/Checkers/CMakeFiles/clangStaticAnalyzerCheckers.dir/TraversalChecker.cpp.o
[01:07:25] [ 75%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextMach_arm.cpp.o
[01:07:26] [ 75%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextMach_i386.cpp.o
[01:07:26] [ 75%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextMach_i386.cpp.o
[01:07:26] [ 76%] Building CXX object tools/lldb/source/Plugins/SymbolFile/NativePDB/CMakeFiles/lldbPluginSymbolFileNativePDB.dir/PdbUtil.cpp.o
[01:07:28] [ 76%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextMach_x86_64.cpp.o
[01:07:29] [ 76%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextMemory.cpp.o
[01:07:29] [ 76%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextMemory.cpp.o
[01:07:30] [ 76%] Building CXX object tools/lldb/source/Plugins/SymbolFile/NativePDB/CMakeFiles/lldbPluginSymbolFileNativePDB.dir/SymbolFileNativePDB.cpp.o
[01:07:32] [ 76%] Building CXX object tools/lldb/source/Plugins/SymbolFile/DWARF/CMakeFiles/lldbPluginSymbolFileDWARF.dir/DWARFDebugAbbrev.cpp.o
[01:07:33] [ 76%] Building CXX object tools/clang/lib/StaticAnalyzer/Checkers/CMakeFiles/clangStaticAnalyzerCheckers.dir/TrustNonnullChecker.cpp.o
[01:07:34] [ 76%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextNetBSD_x86_64.cpp.o
[01:07:36] [ 76%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextOpenBSD_i386.cpp.o
[01:07:36] [ 76%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextOpenBSD_i386.cpp.o
[01:07:36] [ 76%] Building CXX object tools/lldb/source/Plugins/SymbolFile/DWARF/CMakeFiles/lldbPluginSymbolFileDWARF.dir/DWARFDebugAranges.cpp.o
[01:07:38] [ 76%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextOpenBSD_x86_64.cpp.o
[01:07:40] [ 76%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextPOSIX_arm.cpp.o
[01:07:40] [ 76%] Building CXX object tools/lldb/source/Plugins/SymbolFile/DWARF/CMakeFiles/lldbPluginSymbolFileDWARF.dir/DWARFDebugArangeSet.cpp.o
[01:07:44] [ 76%] Building CXX object tools/clang/lib/StaticAnalyzer/Checkers/CMakeFiles/clangStaticAnalyzerCheckers.dir/UndefBranchChecker.cpp.o
[01:07:44] [ 76%] Building CXX object tools/lldb/source/Plugins/SymbolFile/DWARF/CMakeFiles/lldbPluginSymbolFileDWARF.dir/DWARFDebugInfo.cpp.o
[01:07:45] [ 76%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextPOSIX_arm64.cpp.o
[01:07:48] [ 76%] Building CXX object tools/lldb/source/Plugins/SymbolFile/NativePDB/CMakeFiles/lldbPluginSymbolFileNativePDB.dir/UdtRecordCompleter.cpp.o
[01:07:50] [ 76%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextPOSIX_mips64.cpp.o
[01:07:53] [ 76%] Building CXX object tools/clang/lib/StaticAnalyzer/Checkers/CMakeFiles/clangStaticAnalyzerCheckers.dir/UndefCapturedBlockVarChecker.cpp.o
[01:07:54] [ 76%] Building CXX object tools/lldb/source/Plugins/SymbolFile/DWARF/CMakeFiles/lldbPluginSymbolFileDWARF.dir/DWARFDebugLine.cpp.o
[01:07:56] [ 76%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextPOSIX_powerpc.cpp.o
---
[01:09:47] [ 77%] Building CXX object tools/lldb/source/Utility/CMakeFiles/lldbUtility.dir/Baton.cpp.o
[01:09:48] [ 77%] Building CXX object tools/lldb/source/Utility/CMakeFiles/lldbUtility.dir/Connection.cpp.o
[01:09:48] [ 77%] Building CXX object tools/lldb/source/Target/CMakeFiles/lldbTarget.dir/ModuleCache.cpp.o
[01:09:49] [ 77%] Building CXX object tools/lldb/source/Utility/CMakeFiles/lldbUtility.dir/ConstString.cpp.o
[01:09:50] In file included from /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ABI/SysV-arm/ABISysV_arm.cpp:35:
[01:09:50] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:09:50]   case COND_EQ:
[01:09:50]   ^
[01:09:50] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:09:50]   case COND_EQ:
[01:09:50]   ^
[01:09:50]   LLVM_FALLTHROUGH; 
[01:09:50] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: note: insert 'break;' to avoid fall-through
[01:09:50]   case COND_EQ:
[01:09:50]   break; 
[01:09:50] 1 warning generated.
[01:09:50] [ 77%] Linking CXX static library ../../../../../../lib/liblldbPluginABISysV_arm.a
[01:09:50] [ 77%] Built target lldbPluginABISysV_arm
---
[01:11:05] [ 80%] Building CXX object tools/lldb/source/Target/CMakeFiles/lldbTarget.dir/StackFrame.cpp.o
[01:11:07] [ 80%] Building CXX object tools/lldb/source/Symbol/CMakeFiles/lldbSymbol.dir/SymbolVendor.cpp.o
[01:11:07] Scanning dependencies of target lldbPluginABIMacOSX_arm64
[01:11:07] [ 80%] Building CXX object tools/lldb/source/Plugins/ABI/MacOSX-arm64/CMakeFiles/lldbPluginABIMacOSX_arm64.dir/ABIMacOSX_arm64.cpp.o
[01:11:10] In file included from /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/ABI/MacOSX-arm/ABIMacOSX_arm.cpp:35:
[01:11:10] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:11:10]   case COND_EQ:
[01:11:10]   ^
[01:11:10] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:11:10]   case COND_EQ:
[01:11:10]   ^
[01:11:10]   LLVM_FALLTHROUGH; 
[01:11:10] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: note: insert 'break;' to avoid fall-through
[01:11:10]   case COND_EQ:
[01:11:10]   break; 
[01:11:10] 1 warning generated.
[01:11:10] [ 80%] Linking CXX static library ../../../../../../lib/liblldbPluginABIMacOSX_arm.a
[01:11:10] [ 80%] Built target lldbPluginABIMacOSX_arm
[01:11:10] [ 80%] Built target lldbPluginABIMacOSX_arm
[01:11:10] [ 80%] Building CXX object tools/lldb/source/Target/CMakeFiles/lldbTarget.dir/StackFrameList.cpp.o
[01:11:11] [ 80%] Building CXX object tools/lldb/source/Symbol/CMakeFiles/lldbSymbol.dir/Symtab.cpp.o
[01:11:12] Scanning dependencies of target lldbPluginArchitectureArm
[01:11:12] [ 80%] Building CXX object tools/lldb/source/Plugins/Architecture/Arm/CMakeFiles/lldbPluginArchitectureArm.dir/ArchitectureArm.cpp.o
[01:11:13] [ 80%] Linking CXX static library ../../../../../../lib/liblldbPluginABIMacOSX_arm64.a
[01:11:13] [ 80%] Built target lldbPluginABIMacOSX_arm64
[01:11:13] Scanning dependencies of target lldbPluginArchitectureMips
[01:11:13] [ 80%] Building CXX object tools/lldb/source/Plugins/Architecture/Mips/CMakeFiles/lldbPluginArchitectureMips.dir/ArchitectureMips.cpp.o
[01:11:16] In file included from /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/Architecture/Arm/ArchitectureArm.cpp:11:
[01:11:16] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:11:16]   case COND_EQ:
[01:11:16]   ^
[01:11:16] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:11:16]   case COND_EQ:
[01:11:16]   ^
[01:11:16]   LLVM_FALLTHROUGH; 
[01:11:16] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: note: insert 'break;' to avoid fall-through
[01:11:16]   case COND_EQ:
[01:11:16]   break; 
[01:11:16] 1 warning generated.
[01:11:16] [ 80%] Linking CXX static library ../../../../../../lib/liblldbPluginArchitectureArm.a
[01:11:16] [ 80%] Built target lldbPluginArchitectureArm
---
[01:12:01] Scanning dependencies of target lldbPluginInstructionPPC64
[01:12:01] [ 82%] Building CXX object tools/lldb/source/Plugins/Instruction/PPC64/CMakeFiles/lldbPluginInstructionPPC64.dir/EmulateInstructionPPC64.cpp.o
[01:12:02] Scanning dependencies of target lldbPluginInstrumentationRuntimeASan
[01:12:02] [ 82%] Building CXX object tools/lldb/source/Plugins/InstrumentationRuntime/ASan/CMakeFiles/lldbPluginInstrumentationRuntimeASan.dir/ASanRuntime.cpp.o
[01:12:04] In file included from /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/Instruction/ARM64/EmulateInstructionARM64.cpp:10:
[01:12:04] In file included from /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/Instruction/ARM64/EmulateInstructionARM64.h:17:
[01:12:04] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:12:04]   case COND_EQ:
[01:12:04]   ^
[01:12:04] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: note: insert '[[clang::fallthrough]];' to silence this warning
[01:12:04]   case COND_EQ:
[01:12:04]   ^
[01:12:04]   [[clang::fallthrough]]; 
[01:12:04] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: note: insert 'break;' to avoid fall-through
[01:12:04]   case COND_EQ:
[01:12:04]   break; 
[01:12:04] 1 warning generated.
[01:12:04] [ 82%] Linking CXX static library ../../../../../../lib/liblldbPluginInstructionARM64.a
[01:12:04] [ 82%] Built target lldbPluginInstructionARM64
---
[01:15:29] [ 90%] Building CXX object tools/clang/tools/libclang/CMakeFiles/libclang.dir/CIndexInclusionStack.cpp.o
[01:15:33] [ 90%] Building CXX object tools/lldb/source/API/CMakeFiles/liblldb.dir/SBBroadcaster.cpp.o
[01:15:35] [ 90%] Building CXX object tools/lldb/source/API/CMakeFiles/liblldb.dir/SBCommandInterpreter.cpp.o
[01:15:36] In file included from /Users/travis/build/rust-lang/rust/src/tools/lldb/tools/lldb-test/SystemInitializerTest.cpp:40:
[01:15:36] In file included from /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/Instruction/ARM64/EmulateInstructionARM64.h:17:
[01:15:36] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/Process/Utility/ARMDefines.h:74:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:15:36]   case COND_EQ:
[01:15:36]   ^
[01:15:36] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/Process/Utility/ARMDefines.h:74:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:15:36]   case COND_EQ:
[01:15:36]   ^
[01:15:36]   LLVM_FALLTHROUGH; 
[01:15:36] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/Plugins/Process/Utility/ARMDefines.h:74:3: note: insert 'break;' to avoid fall-through
[01:15:36]   case COND_EQ:
[01:15:36]   break; 
[01:15:36] 1 warning generated.
[01:15:36] Scanning dependencies of target dsymutil
[01:15:36] [ 90%] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/dsymutil.cpp.o
---
[01:17:27] [ 98%] Building CXX object tools/lldb/source/API/CMakeFiles/liblldb.dir/SBWatchpoint.cpp.o
[01:17:27] [ 98%] Building CXX object tools/lldb/source/API/CMakeFiles/liblldb.dir/SBUnixSignals.cpp.o
[01:17:27] [ 98%] Building CXX object tools/lldb/source/API/CMakeFiles/liblldb.dir/SystemInitializerFull.cpp.o
[01:17:31] [ 98%] Building CXX object tools/lldb/source/API/CMakeFiles/liblldb.dir/__/__/scripts/LLDBWrapPython.cpp.o
[01:17:39] In file included from /Users/travis/build/rust-lang/rust/src/tools/lldb/source/API/SystemInitializerFull.cpp:51:
[01:17:39] In file included from /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Instruction/ARM64/EmulateInstructionARM64.h:17:
[01:17:39] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: warning: unannotated fall-through between switch labels [-Wimplicit-fallthrough]
[01:17:39]   case COND_EQ:
[01:17:39]   ^
[01:17:39] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: note: insert 'LLVM_FALLTHROUGH;' to silence this warning
[01:17:39]   case COND_EQ:
[01:17:39]   ^
[01:17:39]   LLVM_FALLTHROUGH; 
[01:17:39] /Users/travis/build/rust-lang/rust/src/tools/lldb/source/./Plugins/Process/Utility/ARMDefines.h:74:3: note: insert 'break;' to avoid fall-through
[01:17:39]   case COND_EQ:
[01:17:39]   break; 
[01:17:39] 1 warning generated.
[01:17:58] [ 99%] Linking CXX shared library ../../../../lib/liblldb.dylib
[01:18:00] [ 99%] Built target liblldb
---
[02:48:25] travis_time:end:stage2-cargo-clippy:start=1543376752620730000,finish=1543376753134638000,duration=513908000

[02:48:25] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-apple-darwin" }, target: "x86_64-apple-darwin", tool: "cargo-clippy", path: "src/tools/clippy", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: [] } -- 0.540
[02:48:27] [TIMING] Clippy { stage: 2, target: "x86_64-apple-darwin" } -- 2.282
The job exceeded the maximum time limit for jobs, and has been terminated.
