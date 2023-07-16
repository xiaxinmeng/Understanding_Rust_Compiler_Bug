 bash
rustc: x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_front
compile: x86_64-unknown-linux-gnu/rustllvm/ExecutionEngineWrapper.o
compile: x86_64-unknown-linux-gnu/rustllvm/RustWrapper.o
compile: x86_64-unknown-linux-gnu/rustllvm/PassWrapper.o
compile: x86_64-unknown-linux-gnu/rustllvm/ArchiveWrapper.o
link: x86_64-unknown-linux-gnu/rt/librustllvm.a
failed to run llvm_config: args = `['/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release/bin/llvm-config', '--libs', '--system-libs', 'x86', 'arm', 'aarch64', 'mips', 'powerpc', 'ipo', 'bitreader', 'bitwriter', 'linker', 'asmparser', 'mcjit', 'interpreter', 'instrumentation']`
llvm-config: unknown component name: arm

/home/zazdxscf/build/1nonpkgs/rust/rust/mk/llvm.mk:89: recipe for target '/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs' failed
make: *** [/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs] Error 1

real    85m59.154s
user    76m58.310s
sys 7m55.930s

