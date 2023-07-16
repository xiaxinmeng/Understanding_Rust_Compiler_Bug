 bash
$ make clean
$ ./configure
$ time make RUST_BACKTRACE=1
...
I had to stop it because it was taking too long, apparently compiling llvm for PowerPC at the time
real    67m4.734s
user    61m36.373s
sys 4m50.800s

$ make clean
$ ./configure --enable-ccache --enable-dist-host-only
...
configure: configuring LLVM with:
configure: --enable-targets=x86,x86_64,arm,aarch64,mips,powerpc --enable-optimized --disable-assertions --disable-docs --enable-bindings=none --disable-terminfo --disable-zlib --disable-libffi --build=x86_64-unknown-linux-gnu                         --host=x86_64-unknown-linux-gnu --target=x86_64-unknown-linux-gnu --with-python=/usr/bin/python2.7
...
//great, now I see that it's going to be compiling powerpc anyway, can't I have only x86_64 ?!
...
$ time make RUST_BACKTRACE=1
...
make[4]: Entering directory '/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/lib/Target/PowerPC'
llvm[4]: Building PPC.td register info implementation with tblgen
...
//unacceptable! C-c ed
real    68m2.131s
user    61m41.616s
sys 5m36.360s
...
