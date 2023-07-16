 bash
$ llc --version
LLVM (http://llvm.org/):
  LLVM version 3.8.0svn
  Optimized build.
  Default target: x86_64-pc-linux-gnu
  Host CPU: (unknown)

  Registered Targets:
    amdgcn - AMD GCN GPUs
    cpp    - C++ backend
    r600   - AMD GPUs HD2XXX-HD6XXX
    x86    - 32-bit X86: Pentium-Pro and above
    x86-64 - 64-bit X86: EM64T and AMD64

$ cd /home/zazdxscf/build/1nonpkgs/rust/
//already have: $ wget https://patch-diff.githubusercontent.com/raw/rust-lang/rust/pull/29272.patch
$ cd rust/
$ git clean -dfx
...
$ git reset --hard
$ git pull
$ patch -Np1 -i ../29272.patch
$ ./configure --prefix=/home/zazdxscf/build/1nonpkgs/rust/usr/local --enable-rpath --enable-manage-submodules --disable-clang --enable-ccache --enable-dist-host-only --disable-valgrind --disable-helgrind --disable-valgrind-rpass --python=/usr/bin/python2 --enable-optimize --enable-optimize-cxx --enable-optimize-llvm --enable-debug --enable-debuginfo --enable-debug-assertions --enable-debuginfo-tests --enable-llvm-assertions --enable-debug-jemalloc --disable-local-rust --llvm-root=/usr --release-channel=dev --host=x86_64-unknown-linux-gnu --target=x86_64-unknown-linux-gnu --build=x86_64-unknown-linux-gnu
...
$ time RUST_LOG=rustc::metadata::loader make -j1 -- VERBOSE=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z print-link-args -Z print-llvm-passes -C debug-assertions=y' RUST_BACKTRACE=1
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
/usr/bin/python2 /home/zazdxscf/build/1nonpkgs/rust/rust/src/etc/get-snapshot.py x86_64-unknown-linux-gnu 
determined most recent snapshot: rust-stage0-2015-08-11-1af31d4-linux-x86_64-7df8ba9dec63ec77b857066109d4b6250f3d222f.tar.bz2
...
g++  -O2  -Wall -Werror -g -fPIC -m64 -fno-rtti -c -o  x86_64-unknown-linux-gnu/rustllvm/ArchiveWrapper.o  -I//usr//include -O2 -pipe -march=native -ggdb -fvar-tracking-assignments -fno-omit-frame-pointer -ftrack-macro-expansion=2 -fstack-protector-all -fPIC  -fPIC -fvisibility-inlines-hidden -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wnon-virtual-dtor -Wno-comment -Werror=date-time -std=c++11 -ffunction-sections -fdata-sections    -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS   -I /usr/include -I /home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/include /home/zazdxscf/build/1nonpkgs/rust/rust/src/rustllvm/ArchiveWrapper.cpp
ar crus x86_64-unknown-linux-gnu/rt/librustllvm.a x86_64-unknown-linux-gnu/rustllvm/ExecutionEngineWrapper.o x86_64-unknown-linux-gnu/rustllvm/RustWrapper.o x86_64-unknown-linux-gnu/rustllvm/PassWrapper.o x86_64-unknown-linux-gnu/rustllvm/ArchiveWrapper.o
/usr/bin/python2 "/home/zazdxscf/build/1nonpkgs/rust/rust/src/etc/mklldeps.py" "/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs" "x86 arm aarch64 mips powerpc ipo bitreader bitwriter linker asmparser mcjit interpreter instrumentation" "" /usr/bin/llvm-config "stdc++"
failed to run llvm_config: args = `['/usr/bin/llvm-config', '--libs', '--system-libs', 'x86', 'arm', 'aarch64', 'mips', 'powerpc', 'ipo', 'bitreader', 'bitwriter', 'linker', 'asmparser', 'mcjit', 'interpreter', 'instrumentation']`
llvm-config: unknown component name: arm

/home/zazdxscf/build/1nonpkgs/rust/rust/mk/llvm.mk:89: recipe for target '/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs' failed
make: *** [/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs] Error 1

real    10m35.573s
user    9m29.297s
sys 0m45.763s
