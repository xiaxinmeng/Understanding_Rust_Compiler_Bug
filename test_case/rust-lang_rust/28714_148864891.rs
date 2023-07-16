
$ ./configure --prefix=/home/zazdxscf/build/1nonpkgs/rust/usr/local --disable-rpath --enable-manage-submodules --enable-clang --enable-ccache --enable-dist-host-only --disable-valgrind --disable-helgrind --disable-valgrind-rpass --python=/usr/bin/python2 --enable-optimize --enable-optimize-cxx --enable-optimize-llvm --enable-debug --disable-debuginfo --enable-debug-assertions --enable-debuginfo-tests --enable-llvm-assertions --enable-debug-jemalloc --disable-local-rust --disable-llvm-version-check --llvm-root=/usr
...
$ make -j1 -- all NO_REBUILD=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args'
cfg: version 1.5.0-dev (747d951e8 2015-10-16)
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: enabling debug assertions (CFG_ENABLE_DEBUG_ASSERTIONS)
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: good valgrind for x86_64-unknown-linux-gnu is 1
cfg: using CC=ccache clang (CFG_CC)
cfg: using CXX=clang++ (CFG_CXX)
cfg: disabling valgrind run-pass tests
compile: x86_64-unknown-linux-gnu/rustllvm/ExecutionEngineWrapper.o
x86_64-pc-linux-gnu-clang-3.8: error: unknown argument: '-fvar-tracking-assignments'
x86_64-pc-linux-gnu-clang-3.8: error: unknown argument: '-ftrack-macro-expansion=2'
/home/zazdxscf/build/1nonpkgs/rust/rust/mk/rustllvm.mk:60: recipe for target 'x86_64-unknown-linux-gnu/rustllvm/ExecutionEngineWrapper.o' failed
make: *** [x86_64-unknown-linux-gnu/rustllvm/ExecutionEngineWrapper.o] Error 1

real    0m1.065s
user    0m0.397s
sys 0m0.447s
