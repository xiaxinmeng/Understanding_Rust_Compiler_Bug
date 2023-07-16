toml
changelog-seen = 2
[llvm]
link-shared = true
[build]
sanitizers = true
profiler = true
[install]
[rust]
[target.x86_64-unknown-linux-gnu]
cc = "clang"
cxx = "clang++"
ar = "/usr/bin/llvm-ar"
ranlib = "/usr/bin/llvm-ranlib"
linker = "clang"
llvm-config = "/usr/bin/llvm-config"
[dist]
