
[llvm]
cflags = "-march=native -pipe -O2"
cxxflags = "-march=native -pipe -O2"
ldflags = "-Wl,-O1 -Wl,--as-needed"
[build]
build = "x86_64-unknown-linux-gnu"
host = ["x86_64-unknown-linux-gnu"]
target = ["i686-unknown-linux-gnu","x86_64-unknown-linux-gnu"]
[rust]
backtrace = true
default-linker = "x86_64-pc-linux-gnu-cc"
codegen-backends = ["llvm"]
llvm-libunwind = true
[target.i686-unknown-linux-gnu]
cc = "i686-pc-linux-gnu-cc"
cxx = "i686-pc-linux-gnu-c++"
ar = "i686-pc-linux-gnu-ar"
ranlib = "i686-pc-linux-gnu-ranlib"
linker = "i686-pc-linux-gnu-cc"
[target.x86_64-unknown-linux-gnu]
cc = "x86_64-pc-linux-gnu-cc"
cxx = "x86_64-pc-linux-gnu-c++"
ar = "x86_64-pc-linux-gnu-ar"
ranlib = "x86_64-pc-linux-gnu-ranlib"
linker = "x86_64-pc-linux-gnu-cc"
