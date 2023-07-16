 bash
configure: looking at LLVM
configure:
configure: configuring LLVM for i686-unknown-linux-gnu
configure: configuring LLVM with:
configure: --enable-targets=x86_64 --enable-optimized --enable-assertions --disable-docs --enable-bindings=none --disable-terminfo --disable-zlib --disable-libffi --build=i686-unknown-linux-gnu                         --host=i686-unknown-linux-gnu --target=i686-unknown-linux-gnu --with-python=/usr/bin/python2.7
/rust/rust-0.8/src/llvm/configure: line 1995: -m32: command not found
/rust/rust-0.8/src/llvm/configure: line 1996: -m32: command not found
checking for i686-unknown-linux-gnu-clang... gcc -m32
checking for C compiler default output file name... configure: error: C compiler cannot create executables
See `config.log' for more details.
