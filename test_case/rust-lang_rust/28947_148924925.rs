
...
compile: x86_64-unknown-linux-gnu/rustllvm/RustWrapper.o
/home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/RustWrapper.cpp: In function 'LLVMOpaqueValue* LLVMRustBuildLandingPad(LLVMBuilderRef, LLVMTypeRef, LLVMValueRef, unsigned int, const char*, LLVMValueRef)':
/home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/RustWrapper.cpp:954:69: error: invalid conversion from 'LLVMValueRef {aka LLVMOpaqueValue*}' to 'unsigned int' [-fpermissive]
     return LLVMBuildLandingPad(Builder, Ty, PersFn, NumClauses, Name);
                                                                     ^
/home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/RustWrapper.cpp:954:69: error: invalid conversion from 'unsigned int' to 'const char*' [-fpermissive]
/home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/RustWrapper.cpp:954:69: error: too many arguments to function 'LLVMOpaqueValue* LLVMBuildLandingPad(LLVMBuilderRef, LLVMTypeRef, unsigned int, const char*)'
In file included from /usr/include/llvm/IR/Value.h:17:0,
                 from /usr/include/llvm/IR/User.h:24,
                 from /usr/include/llvm/IR/Instruction.h:22,
                 from /usr/include/llvm/IR/BasicBlock.h:19,
                 from /usr/include/llvm/IR/IRBuilder.h:21,
                 from /home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/rustllvm.h:11,
                 from /home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/RustWrapper.cpp:11:
/usr/include/llvm-c/Core.h:2677:14: note: declared here
 LLVMValueRef LLVMBuildLandingPad(LLVMBuilderRef B, LLVMTypeRef Ty,
              ^
/home/zazdxscf/build/1nonpkgs/rust/rust/mk/rustllvm.mk:60: recipe for target 'x86_64-unknown-linux-gnu/rustllvm/RustWrapper.o' failed
make: *** [x86_64-unknown-linux-gnu/rustllvm/RustWrapper.o] Error 1
