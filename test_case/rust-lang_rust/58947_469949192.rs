
[llvm]
optimize = true
release-debuginfo = false
assertions = false
targets = "X86;Mips;NVPTX;BPF;AArch64"
link-shared = false
[build]
build = "x86_64-unknown-linux-gnu"
host = ["x86_64-unknown-linux-gnu"]
target = ["x86_64-unknown-linux-gnu","wasm32-unknown-unknown"]
docs = true
submodules = false
python = "python2.7"
locked-deps = true
vendor = false
verbose = 2
sanitizers = true
extended = true
tools = ["miri","rustfmt","rls","analysis","src","clippy","cargo",]
[install]
prefix = "/usr"
libdir = "lib64/rust-9999"
docdir = "share/doc/rust-9999"
mandir = "share/rust-9999/man"
[rust]
optimize = true
debuginfo = false
debug-assertions = false
jemalloc = true
default-linker = "x86_64-pc-linux-gnu-gcc"
rpath = false
ignore-git = false
lld = true
llvm-tools = true
[target.x86_64-unknown-linux-gnu]
cc = "x86_64-pc-linux-gnu-gcc"
cxx = "x86_64-pc-linux-gnu-g++"
linker = "x86_64-pc-linux-gnu-gcc"
ar = "x86_64-pc-linux-gnu-ar"
[target.wasm32-unknown-unknown]
linker = "rust-lld"
