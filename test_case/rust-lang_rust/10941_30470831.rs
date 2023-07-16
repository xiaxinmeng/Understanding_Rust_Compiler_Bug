
llvm[4]: Compiling Local.cpp for Release+Asserts build
llvm[4]: Compiling SpecialCaseList.cpp for Release+Asserts build
llvm[4]: Linking Release+Asserts unit test Utils (without symbols)
llvm[4]: ======= Finished Linking Release+Asserts Unit test Utils (without symbols)
llvm[5]: Compiling sample.c for Release+Asserts build
llvm[5]: Building Release+Asserts Archive Library libsample.a
llvm[5]: Compiling main.c for Release+Asserts build
llvm[5]: Linking Release+Asserts executable Sample (without symbols)
llvm[5]: ======= Finished Linking Release+Asserts Executable Sample (without symbols)
make[2]: Nothing to be done for `all'.
llvm[1]: ***** Completed Release+Asserts Build
compile: x86_64-apple-darwin/rt/stage0/arch/x86_64/_context.o
compile: x86_64-apple-darwin/rt/stage0/arch/x86_64/record_sp.o
link: x86_64-apple-darwin/rt/stage0/librustrt.a
/usr/bin/ranlib: file: x86_64-apple-darwin/rt/stage0/librustrt.a(rust_android_dummy.o) has no symbols
/usr/bin/ranlib: file: x86_64-apple-darwin/rt/stage0/librustrt.a(record_sp.o) has no symbols
cp: x86_64-apple-darwin/stage0/lib/rustc/x86_64-apple-darwin/lib/librustrt.a
compile: x86_64-apple-darwin/rt/stage0/arch/x86_64/morestack.o
link: x86_64-apple-darwin/rt/stage0/arch/x86_64/libmorestack.a
cp: x86_64-apple-darwin/stage0/lib/rustc/x86_64-apple-darwin/lib/libmorestack.a
compile_and_link: x86_64-apple-darwin/stage0/lib/rustc/x86_64-apple-darwin/lib/libstd.dylib
/Users/carter/Desktop/repoScratcher/rust/src/libstd/lib.rs:46:0: 46:23 warning: unknown crate attribute, #[warn(attribute_usage)] on by default
/Users/carter/Desktop/repoScratcher/rust/src/libstd/lib.rs:46 #[pkgid="std#0.9-pre"];
                                                              ^~~~~~~~~~~~~~~~~~~~~~~
compile_and_link: x86_64-apple-darwin/stage0/lib/rustc/x86_64-apple-darwin/lib/libextra.dylib
/Users/carter/Desktop/repoScratcher/rust/src/libextra/lib.rs:23:0: 23:25 warning: unknown crate attribute, #[warn(attribute_usage)] on by default
/Users/carter/Desktop/repoScratcher/rust/src/libextra/lib.rs:23 #[pkgid="extra#0.9-pre"];
                                                                ^~~~~~~~~~~~~~~~~~~~~~~~~
  CC(target) obj.target/libuv/src/fs-poll.o
  CC(target) obj.target/libuv/src/inet.o
  CC(target) obj.target/libuv/src/uv-common.o
  CC(target) obj.target/libuv/src/version.o
  CC(target) obj.target/libuv/src/unix/async.o
  CC(target) obj.target/libuv/src/unix/core.o
  CC(target) obj.target/libuv/src/unix/dl.o
  CC(target) obj.target/libuv/src/unix/fs.o
  CC(target) obj.target/libuv/src/unix/getaddrinfo.o
  CC(target) obj.target/libuv/src/unix/loop.o
  CC(target) obj.target/libuv/src/unix/loop-watcher.o
  CC(target) obj.target/libuv/src/unix/pipe.o
  CC(target) obj.target/libuv/src/unix/poll.o
  CC(target) obj.target/libuv/src/unix/process.o
  CC(target) obj.target/libuv/src/unix/signal.o
  CC(target) obj.target/libuv/src/unix/stream.o
  CC(target) obj.target/libuv/src/unix/tcp.o
  CC(target) obj.target/libuv/src/unix/thread.o
  CC(target) obj.target/libuv/src/unix/threadpool.o
  CC(target) obj.target/libuv/src/unix/timer.o
  CC(target) obj.target/libuv/src/unix/tty.o
  CC(target) obj.target/libuv/src/unix/udp.o
  CC(target) obj.target/libuv/src/unix/proctitle.o
  CC(target) obj.target/libuv/src/unix/darwin.o
  CC(target) obj.target/libuv/src/unix/fsevents.o
  CC(target) obj.target/libuv/src/unix/darwin-proctitle.o
  CC(target) obj.target/libuv/src/unix/kqueue.o
  LIBTOOL-STATIC libuv.a
compile: x86_64-apple-darwin/rt/uv_support/rust_uv.o
link: x86_64-apple-darwin/rt/uv_support/libuv_support.a
compile_and_link: x86_64-apple-darwin/stage0/lib/rustc/x86_64-apple-darwin/lib/librustuv.dylib
/Users/carter/Desktop/repoScratcher/rust/src/librustuv/lib.rs:37:0: 37:26 warning: unknown crate attribute, #[warn(attribute_usage)] on by default
/Users/carter/Desktop/repoScratcher/rust/src/librustuv/lib.rs:37 #[pkgid="rustuv#0.9-pre"];
                                                                 ^~~~~~~~~~~~~~~~~~~~~~~~~~
compile_and_link: x86_64-apple-darwin/stage0/lib/rustc/x86_64-apple-darwin/lib/libsyntax.dylib
/Users/carter/Desktop/repoScratcher/rust/src/libsyntax/lib.rs:16:0: 16:26 warning: unknown crate attribute, #[warn(attribute_usage)] on by default
/Users/carter/Desktop/repoScratcher/rust/src/libsyntax/lib.rs:16 #[pkgid="syntax#0.9-pre"];
                                                                 ^~~~~~~~~~~~~~~~~~~~~~~~~~
compile: x86_64-apple-darwin/rustllvm/RustWrapper.o
compile: x86_64-apple-darwin/rustllvm/PassWrapper.o
link: x86_64-apple-darwin/rustllvm/librustllvm.a
cp: x86_64-apple-darwin/stage0/lib/rustc/x86_64-apple-darwin/lib/librustllvm.a
compile_and_link: x86_64-apple-darwin/stage0/lib/rustc/x86_64-apple-darwin/lib/librustc.dylib
/Users/carter/Desktop/repoScratcher/rust/src/librustc/lib.rs:11:0: 11:25 warning: unknown crate attribute, #[warn(attribute_usage)] on by default
/Users/carter/Desktop/repoScratcher/rust/src/librustc/lib.rs:11 #[pkgid="rustc#0.9-pre"];
                                                                ^~~~~~~~~~~~~~~~~~~~~~~~~
error: linking with `cc` failed: exit code: 1
note: cc arguments: '-m64' '-L/Users/carter/Desktop/repoScratcher/rust/x86_64-apple-darwin/stage0/lib/rustc/x86_64-apple-darwin/lib' '-o' 'x86_64-apple-darwin/stage0/lib/rustc/x86_64-apple-darwin/lib/librustc-8581899a03b03e-0.9-pre.dylib' 'x86_64-apple-darwin/stage0/lib/rustc/x86_64-apple-darwin/lib/rustc.o' '-L/Users/carter/Desktop/repoScratcher/rust/x86_64-apple-darwin/llvm/Release+Asserts/lib' '-L/Users/carter/Desktop/repoScratcher/rust/.rust' '-L/Users/carter/Desktop/repoScratcher/rust' '-lrustllvm' '-lstdc++' '-lLLVMInstrumentation' '-lLLVMInterpreter' '-lLLVMMCJIT' '-lLLVMJIT' '-lLLVMRuntimeDyld' '-lLLVMExecutionEngine' '-lLLVMAsmParser' '-lLLVMLinker' '-lLLVMBitWriter' '-lLLVMBitReader' '-lLLVMipo' '-lLLVMVectorize' '-lLLVMMipsDisassembler' '-lLLVMMipsCodeGen' '-lLLVMMipsAsmParser' '-lLLVMMipsDesc' '-lLLVMMipsInfo' '-lLLVMMipsAsmPrinter' '-lLLVMARMDisassembler' '-lLLVMARMCodeGen' '-lLLVMARMAsmParser' '-lLLVMARMDesc' '-lLLVMARMInfo' '-lLLVMARMAsmPrinter' '-lLLVMX86Disassembler' '-lLLVMX86AsmParser' '-lLLVMX86CodeGen' '-lLLVMSelectionDAG' '-lLLVMAsmPrinter' '-lLLVMMCParser' '-lLLVMCodeGen' '-lLLVMObjCARCOpts' '-lLLVMScalarOpts' '-lLLVMInstCombine' '-lLLVMTransformUtils' '-lLLVMipa' '-lLLVMAnalysis' '-lLLVMX86Desc' '-lLLVMX86Info' '-lLLVMTarget' '-lLLVMX86AsmPrinter' '-lLLVMMC' '-lLLVMObject' '-lLLVMX86Utils' '-lLLVMCore' '-lLLVMSupport' '-L/Users/carter/Desktop/repoScratcher/rust/x86_64-apple-darwin/stage0/lib/rustc/x86_64-apple-darwin/lib' '-lstd-6425b930ca146ae9-0.9-pre' '-L/Users/carter/Desktop/repoScratcher/rust/x86_64-apple-darwin/stage0/lib/rustc/x86_64-apple-darwin/lib' '-lextra-aaa96aab146eb38e-0.9-pre' '-L/Users/carter/Desktop/repoScratcher/rust/x86_64-apple-darwin/stage0/lib/rustc/x86_64-apple-darwin/lib' '-lsyntax-2bb2d559d93ae8f0-0.9-pre' '-lpthread' '-lstdc++' '-dynamiclib' '-Wl,-dylib' '-Wl,-install_name,@rpath/librustc-8581899a03b03e-0.9-pre.dylib' '-lmorestack' '-Wl,-rpath,@loader_path/.' '-Wl,-rpath,/Users/carter/Desktop/repoScratcher/rust/x86_64-apple-darwin/stage0/lib/rustc/x86_64-apple-darwin/lib' '-Wl,-rpath,/usr/local/lib/rustc/x86_64-apple-darwin/lib'
note: ld: warning: directory not found for option '-L/Users/carter/Desktop/repoScratcher/rust/.rust'
Undefined symbols for architecture x86_64:
  "std::__detail::_List_node_base::_M_hook(std::__detail::_List_node_base*)", referenced from:
      (anonymous namespace)::MemsetRanges::addRange(long long, long long, llvm::Value*, unsigned int, llvm::Instruction*) in libLLVMScalarOpts.a(MemCpyOptimizer.o)
      (anonymous namespace)::RegToMem::runOnFunction(llvm::Function&) in libLLVMScalarOpts.a(Reg2Mem.o)
      (anonymous namespace)::MipsELFObjectWriter::sortRelocs(llvm::MCAssembler const&, std::vector<llvm::ELFRelocationEntry, std::allocator<llvm::ELFRelocationEntry> >&) in libLLVMMipsDesc.a(MipsELFObjectWriter.o)
  "std::__detail::_List_node_base::_M_unhook()", referenced from:
      (anonymous namespace)::MemsetRanges::addRange(long long, long long, llvm::Value*, unsigned int, llvm::Instruction*) in libLLVMScalarOpts.a(MemCpyOptimizer.o)
      (anonymous namespace)::MipsELFObjectWriter::sortRelocs(llvm::MCAssembler const&, std::vector<llvm::ELFRelocationEntry, std::allocator<llvm::ELFRelocationEntry> >&) in libLLVMMipsDesc.a(MipsELFObjectWriter.o)
ld: symbol(s) not found for architecture x86_64
clang: error: linker command failed with exit code 1 (use -v to see invocation)

error: aborting due to previous error
task 'rustc' failed at 'explicit failure', /Users/rustbuild/src/rust-buildbot/slave/snap3-mac/build/src/libsyntax/diagnostic.rs:102
task '<main>' failed at 'explicit failure', /Users/rustbuild/src/rust-buildbot/slave/snap3-mac/build/src/librustc/lib.rs:393
make: *** [x86_64-apple-darwin/stage0/lib/rustc/x86_64-apple-darwin/lib/librustc.dylib] Error 101


