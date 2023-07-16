 bash
$ llc --version
LLVM (http://llvm.org/):
  LLVM version 3.7.0
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
$ wget https://patch-diff.githubusercontent.com/raw/rust-lang/rust/pull/29272.patch
$ cd rust/
$ git clean -dfx
...
$ git reset --hard
$ git pull
$ patch -Np1 -i ../llvm_only_x86_64.patch
$ patch -Np1 -i ../29272.patch
$ ./configure --prefix=/home/zazdxscf/build/1nonpkgs/rust/usr/local --enable-rpath --enable-manage-submodules --disable-clang --enable-ccache --enable-dist-host-only --disable-valgrind --disable-helgrind --disable-valgrind-rpass --python=/usr/bin/python2 --enable-optimize --enable-optimize-cxx --enable-optimize-llvm --enable-debug --enable-debuginfo --enable-debug-assertions --enable-debuginfo-tests --enable-llvm-assertions --enable-debug-jemalloc --disable-local-rust --llvm-root=/usr --release-channel=dev --host=x86_64-unknown-linux-gnu --target=x86_64-unknown-linux-gnu --build=x86_64-unknown-linux-gnu
...
