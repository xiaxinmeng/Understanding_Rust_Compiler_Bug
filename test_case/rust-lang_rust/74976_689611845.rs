toml
[llvm]
optimize = true
release-debuginfo = false
assertions = false
ninja = true
targets = "X86;Mips;NVPTX;RISCV;BPF;AArch64;WebAssembly"
experimental-targets = ""
link-shared = true
[build]
build = "x86_64-unknown-linux-gnu"
host = ["x86_64-unknown-linux-gnu"]
target = ["i686-unknown-linux-gnu","x86_64-unknown-linux-gnu","wasm32-unknown-unknown"]
docs = true
compiler-docs = true
submodules = false
python = "python3.7"
locked-deps = true
vendor = false
extended = true
tools = ["rust-analyzer","rustfmt","rls","miri","clippy","cargo",]
verbose = 2
sanitizers = false
profiler = false
cargo-native-static = false
[install]
prefix = "/usr"
libdir = "lib64/rust-9999"
docdir = "share/doc/rust-9999"
mandir = "share/rust-9999/man"
[rust]
optimize = true
debug = false
debug-assertions = false
debuginfo-level-rustc = 0
backtrace = true
incremental = false
default-linker = "x86_64-pc-linux-gnu-gcc"
parallel-compiler = false
rpath = false
verbose-tests = true
optimize-tests = true
codegen-tests = true
dist-src = false
ignore-git = false
lld = false
backtrace-on-ice = true
jemalloc = false
[dist]
src-tarball = false
[target.i686-unknown-linux-gnu]
cc = "x86_64-pc-linux-gnu-gcc"
cxx = "x86_64-pc-linux-gnu-g++"
linker = "x86_64-pc-linux-gnu-gcc"
ar = "x86_64-pc-linux-gnu-ar"
llvm-config = "/usr/lib/llvm/10/bin/llvm-config"
[target.x86_64-unknown-linux-gnu]
cc = "x86_64-pc-linux-gnu-gcc"
cxx = "x86_64-pc-linux-gnu-g++"
linker = "x86_64-pc-linux-gnu-gcc"
ar = "x86_64-pc-linux-gnu-ar"
llvm-config = "/usr/lib/llvm/10/bin/llvm-config"
[target.wasm32-unknown-unknown]
linker = "lld"
