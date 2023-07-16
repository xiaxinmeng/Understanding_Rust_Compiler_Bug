
In file included from /usr/include/c++/8/cmath:45,
                 from /opt/rust/src/llvm-project/llvm/include/llvm-c/DataTypes.h:28,
                 from /opt/rust/src/llvm-project/llvm/include/llvm/Support/DataTypes.h:16,
                 from /opt/rust/src/llvm-project/llvm/include/llvm/ADT/Hashing.h:47,
                 from /opt/rust/src/llvm-project/llvm/include/llvm/ADT/ArrayRef.h:12,
                 from /opt/rust/src/llvm-project/llvm/include/llvm/Transforms/Utils/NoAliasUtils.h:16,
                 from /opt/rust/src/llvm-project/llvm/lib/Transforms/Utils/NoAliasUtils.cpp:13:
/opt/rust/src/llvm-project/llvm/lib/Transforms/Utils/NoAliasUtils.cpp: In function ‘void llvm::cloneNoAliasScopes(llvm::ArrayRef<llvm::MetadataAsValue*>, llvm::DenseMap<llvm::MDN
ode*, llvm::MDNode*>&, llvm::DenseMap<llvm::MetadataAsValue*, llvm::MetadataAsValue*>&, llvm::StringRef, llvm::LLVMContext&)’:
/opt/rust/src/llvm-project/llvm/lib/Transforms/Utils/NoAliasUtils.cpp:174:30: error: no matching function for call to ‘llvm::AliasScopeNode::AliasScopeNode(double)’
         llvm::AliasScopeNode SNAN(MD);
                              ^~~~
In file included from /opt/rust/src/llvm-project/llvm/include/llvm/IR/TrackingMDRef.h:16,
                 from /opt/rust/src/llvm-project/llvm/include/llvm/IR/DebugLoc.h:17,
                 from /opt/rust/src/llvm-project/llvm/include/llvm/IR/Instruction.h:21,
                 from /opt/rust/src/llvm-project/llvm/include/llvm/IR/BasicBlock.h:22,
                 from /opt/rust/src/llvm-project/llvm/include/llvm/IR/Instructions.h:27,
                 from /opt/rust/src/llvm-project/llvm/include/llvm/Transforms/Utils/NoAliasUtils.h:22,
                 from /opt/rust/src/llvm-project/llvm/lib/Transforms/Utils/NoAliasUtils.cpp:13:
/opt/rust/src/llvm-project/llvm/include/llvm/IR/Metadata.h:1446:12: note: candidate: ‘llvm::AliasScopeNode::AliasScopeNode(const llvm::MDNode*)’
   explicit AliasScopeNode(const MDNode *N) : Node(N) {}
            ^~~~~~~~~~~~~~
/opt/rust/src/llvm-project/llvm/include/llvm/IR/Metadata.h:1446:12: note:   no known conversion for argument 1 from ‘double’ to ‘const llvm::MDNode*’
/opt/rust/src/llvm-project/llvm/include/llvm/IR/Metadata.h:1445:3: note: candidate: ‘constexpr llvm::AliasScopeNode::AliasScopeNode()’
   AliasScopeNode() = default;
   ^~~~~~~~~~~~~~
/opt/rust/src/llvm-project/llvm/include/llvm/IR/Metadata.h:1445:3: note:   candidate expects 0 arguments, 1 provided
/opt/rust/src/llvm-project/llvm/include/llvm/IR/Metadata.h:1441:7: note: candidate: ‘constexpr llvm::AliasScopeNode::AliasScopeNode(const llvm::AliasScopeNode&)’
 class AliasScopeNode {
       ^~~~~~~~~~~~~~
/opt/rust/src/llvm-project/llvm/include/llvm/IR/Metadata.h:1441:7: note:   no known conversion for argument 1 from ‘double’ to ‘const llvm::AliasScopeNode&’
/opt/rust/src/llvm-project/llvm/include/llvm/IR/Metadata.h:1441:7: note: candidate: ‘constexpr llvm::AliasScopeNode::AliasScopeNode(llvm::AliasScopeNode&&)’
/opt/rust/src/llvm-project/llvm/include/llvm/IR/Metadata.h:1441:7: note:   no known conversion for argument 1 from ‘double’ to ‘llvm::AliasScopeNode&&’
/opt/rust/src/llvm-project/llvm/lib/Transforms/Utils/NoAliasUtils.cpp:177:31: error: request for member ‘getName’ in ‘__builtin_nans(((const char*)""))’, which is of non-class ty
pe ‘double’
         auto ScopeName = SNAN.getName();
                               ^~~~~~~
/opt/rust/src/llvm-project/llvm/lib/Transforms/Utils/NoAliasUtils.cpp:187:39: error: request for member ‘getDomain’ in ‘__builtin_nans(((const char*)""))’, which is of non-class
type ‘double’
             const_cast<MDNode *>(SNAN.getDomain()), Name);
                                       ^~~~~~~~~
[ 75%] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/RDFCopy.cpp.o
make[2]: *** [lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/build.make:635: lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/NoAliasUtils.cpp.o] Error 1
make[2]: *** Waiting for unfinished jobs....
