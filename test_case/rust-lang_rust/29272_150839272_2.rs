 bash
$ time RUST_LOG=rustc::metadata::loader make -j4 -- VERBOSE=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z print-link-args -Z print-llvm-passes -C debug-assertions=y' RUST_BACKTRACE=1
cfg: version 1.5.0-dev (04e497c00 2015-10-24)
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: enabling debug assertions (CFG_ENABLE_DEBUG_ASSERTIONS)
cfg: enabling debuginfo (CFG_ENABLE_DEBUGINFO)
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: good valgrind for x86_64-unknown-linux-gnu is 1
cfg: using CC=ccache gcc (CFG_CC)
cfg: disabling valgrind run-pass tests
...
time: 0.069; rss: 112MB variance inference
/home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/RustWrapper.cpp: In function 'LLVMOpaqueValue* LLVMRustBuildLandingPad(LLVMBuilderRef, LLVMTypeRef, LLVMValueRef, unsigned int, const char*, LLVMValueRef)':
/home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/RustWrapper.cpp:972:69: error: invalid conversion from 'LLVMValueRef {aka LLVMOpaqueValue*}' to 'unsigned int' [-fpermissive]
     return LLVMBuildLandingPad(Builder, Ty, PersFn, NumClauses, Name);
                                                                     ^
/home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/RustWrapper.cpp:972:69: error: invalid conversion from 'unsigned int' to 'const char*' [-fpermissive]
/home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/RustWrapper.cpp:972:69: error: too many arguments to function 'LLVMOpaqueValue* LLVMBuildLandingPad(LLVMBuilderRef, LLVMTypeRef, unsigned int, const char*)'
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
make: *** Waiting for unfinished jobs....
...
//something error-ed, rerunning with -j1 so to ensure output isn't garbled
