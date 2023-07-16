
[llvm]
[build]
target = ["x86_64-unknown-haiku"]
[install]
[rust]
use-jemalloc = false
debug-jemalloc = false
[target.x86_64-unknown-haiku]
cc = "/home/kallisti5/Code/haiku/generated.x86_64/cross-tools-x86_64/bin/x86_64-unknown-haiku-gcc"
cxx = "/home/kallisti5/Code/haiku/generated.x86_64/cross-tools-x86_64/bin/x86_64-unknown-haiku-g++"
[dist]
