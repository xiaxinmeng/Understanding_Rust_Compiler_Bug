
[llvm]
link-shared = true
[build]
[install]
[rust]
[target.x86_64-unknown-linux-gnu]
cc = "clang"
cxx = "clang++"
ar = "/usr/lib/llvm/11/bin/llvm-ar"
ranlib = "/usr/lib/llvm/11/bin/llvm-ranlib"
linker = "clang"
llvm-config = "/usr/lib/llvm/11/bin/llvm-config"
[dist]
