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
$ git reset --hard
$ git pull
$ patch -Np1 -i ../llvm_only_x86_64.patch
$ patch -Np1 -i ../29272.patch
$ ./configure --llvm-root=/usr --enable-ccache --host=x86_64-unknown-linux-gnu --target=x86_64-unknown-linux-gnu --build=x86_64-unknown-linux-gnu
...
$ time RUST_LOG=rustc::metadata::loader make -j1 -- VERBOSE=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z print-link-args -Z print-llvm-passes' RUST_BACKTRACE=1
...
