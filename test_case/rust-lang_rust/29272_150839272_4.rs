 bash
$ time RUST_LOG=rustc::metadata::loader make -j4 -- VERBOSE=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z print-link-args -Z print-llvm-passes -C debug-assertions=y' RUST_BACKTRACE=1
...
INFO:rustc::metadata::loader: reading "liblog-bb94g++  -O2  -Wall -Werror -g -fPIC -m64 -fno-rtti -c -o  x86_64-unknown-linux-gnu/rustllvm/RustWrapper.o  -I//usr//include -O2 -pipe -march=native -ggdb -fvar-tracking-assignments -fno-omit-frame-pointer -ftrack-macro-expansion=2 -fstack-protector-all -fPIC  -fPIC -fvisibility-inlines-hidden -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wnon-virtual-dtor -Wno-comment -Werror=date-time -std=c++11 -ffunction-sections -fdata-sections    -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS   -I /usr/include -I /home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/include /home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/RustWrapper.cpp
...
time: 0.466; rss: 128MB match checking
time: 0.021; rss: 128MB liveness checking
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
^Cmake: *** Deleting file 'x86_64-unknown-linux-gnu/rustllvm/RustWrapper.o'
/home/zazdxscf/build/1nonpkgs/rust/rust/mk/rustllvm.mk:60: recipe for target 'x86_64-unknown-linux-gnu/rustllvm/RustWrapper.o' failed
make: *** [x86_64-unknown-linux-gnu/rustllvm/RustWrapper.o] Interrupt
/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.serialize' failed
make: *** [x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.serialize] Interrupt


