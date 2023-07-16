 bash
$ git clean -dfx
$ git reset --hard
$ git pull
// latest commit now at: 83cf3ce4981e2ff1a3d7629665b7ec884adbb9de
$ patch -Np1 -i ../llvm_only_x86_64.patch
//that got this line: LLVM_TARGETS="--enable-targets=x86_64"
$ ./configure --enable-ccache --enable-dist-host-only
...
configure: configuring LLVM with:
configure: --enable-targets=x86_64 --enable-optimized --disable-assertions --disable-docs --enable-bindings=none --disable-terminfo --disable-zlib --disable-libffi --build=x86_64-unknown-linux-gnu                         --host=x86_64-unknown-linux-gnu --target=x86_64-unknown-linux-gnu --with-python=/usr/bin/python2.7
...
$ time make RUST_BACKTRACE=1
cfg: version 1.5.0-dev (83cf3ce49 2015-10-22)
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: good valgrind for x86_64-unknown-linux-gnu is 1
cfg: using CC=ccache gcc (CFG_CC)
cfg: disabling valgrind run-pass tests
make: cleaning llvm
...
