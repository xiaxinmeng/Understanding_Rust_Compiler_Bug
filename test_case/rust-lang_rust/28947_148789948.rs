 bash
$ ./configure --prefix=/home/zazdxscf/build/1nonpkgs/rust/usr/local --disable-rpath --enable-manage-submodules --disable-clang --enable-ccache --enable-dist-host-only --disable-valgrind --disable-helgrind --disable-valgrind-rpass --python=/usr/bin/python2 --enable-optimize --enable-optimize-cxx --enable-optimize-llvm --enable-debug --disable-debuginfo --enable-debug-assertions --enable-debuginfo-tests --enable-llvm-assertions --enable-debug-jemalloc --disable-local-rust --disable-llvm-version-check --llvm-root=/usr
...
//this is a rerun after the error happened already:
$ make -j1 -- all NO_REBUILD=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args'
cfg: version 1.5.0-dev (24228fee1 2015-10-16)
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: enabling debug assertions (CFG_ENABLE_DEBUG_ASSERTIONS)
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: good valgrind for x86_64-unknown-linux-gnu is 1
cfg: using CC=ccache gcc (CFG_CC)
cfg: disabling valgrind run-pass tests
compile: x86_64-unknown-linux-gnu/rustllvm/RustWrapper.o
/home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/RustWrapper.cpp: In function 'LLVMOpaqueMetadata* LLVMDIBuilderCreateSubroutineType(DIBuilderRef, LLVMMetadataRef, LLVMMetadataRef)':
/home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/RustWrapper.cpp:325:56: error: no matching function for call to 'llvm::DIBuilder::createSubroutineType(llvm::DIFile*, llvm::DITypeRefArray)'
         DITypeRefArray(unwrap<MDTuple>(ParameterTypes))));
                                                        ^
In file included from /home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/rustllvm.h:56:0,
                 from /home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/RustWrapper.cpp:11:
/usr/include/llvm/IR/DIBuilder.h:382:23: note: candidate: llvm::DISubroutineType* llvm::DIBuilder::createSubroutineType(llvm::DITypeRefArray, unsigned int)
     DISubroutineType *createSubroutineType(DITypeRefArray ParameterTypes,
                       ^
/usr/include/llvm/IR/DIBuilder.h:382:23: note:   no known conversion for argument 1 from 'llvm::DIFile*' to 'llvm::DITypeRefArray'
/home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/RustWrapper.cpp: In function 'LLVMOpaqueMetadata* LLVMDIBuilderCreateVariable(DIBuilderRef, unsigned int, LLVMMetadataRef, const char*, LLVMMetadataRef, unsigned int, LLVMMetadataRef, bool, unsigned int, int64_t*, unsigned int, unsigned int)':
/home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/RustWrapper.cpp:505:26: error: 'class llvm::DIBuilder' has no member named 'createLocalVariable'
     return wrap(Builder->createLocalVariable(Tag,
                          ^
/home/zazdxscf/build/1nonpkgs/rust/rust/mk/rustllvm.mk:60: recipe for target 'x86_64-unknown-linux-gnu/rustllvm/RustWrapper.o' failed
make: *** [x86_64-unknown-linux-gnu/rustllvm/RustWrapper.o] Error 1

real    0m8.069s
user    0m6.697s
sys 0m1.237s
