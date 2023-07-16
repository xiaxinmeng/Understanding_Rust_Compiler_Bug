toml
changelog-seen = 2
[llvm]
download-ci-llvm = false
optimize = true
release-debuginfo = false
assertions = false
ninja = true
targets = "X86;Mips;NVPTX;RISCV;BPF;AArch64;WebAssembly"
experimental-targets = ""
link-shared = false

[build]
build-stage = 2
test-stage = 2
doc-stage = 2
build = "x86_64-unknown-linux-gnu"
host = ["x86_64-unknown-linux-gnu"]
target = ["i686-unknown-linux-gnu","x86_64-unknown-linux-gnu","wasm32-unknown-unknown"]
docs = true
compiler-docs = false
submodules = false
python = "python3.10"
locked-deps = true
vendor = false
extended = true
tools = ["analysis","src","rust-analyzer","rustfmt","rls","miri","clippy","cargo",]
verbose = 2
sanitizers = true
profiler = false
cargo-native-static = false
[install]
prefix = "/usr/lib/rust/9999"
sysconfdir = "etc"
docdir = "share/doc/rust"
bindir = "bin"
libdir = "lib"
mandir = "share/man"
[rust]
# https://github.com/rust-lang/rust/issues/54872
codegen-units-std = 1
optimize = true
debug = false
debug-assertions = false
debug-assertions-std = false
debuginfo-level = 0
debuginfo-level-rustc = 0
debuginfo-level-std = 0
debuginfo-level-tools = 0
debuginfo-level-tests = 0
backtrace = true
incremental = false
default-linker = "x86_64-pc-linux-gnu-gcc"
parallel-compiler = false
description = "gentoo"
rpath = false
verbose-tests = true
optimize-tests = true
codegen-tests = true
dist-src = false
remap-debuginfo = true
ignore-git = false
lld = true
# only deny warnings if doc+wasm are NOT requested, documenting stage0 wasm std fails without it
# https://github.com/rust-lang/rust/issues/74976
# https://github.com/rust-lang/rust/issues/76526
deny-warnings = false
backtrace-on-ice = true
jemalloc = false
[dist]
src-tarball = false
compression-formats = ["xz"]
[target.i686-unknown-linux-gnu]
ar = "x86_64-pc-linux-gnu-ar"
cc = "x86_64-pc-linux-gnu-gcc"
cxx = "x86_64-pc-linux-gnu-g++"
linker = "x86_64-pc-linux-gnu-gcc"
ranlib = "x86_64-pc-linux-gnu-ranlib"
[target.x86_64-unknown-linux-gnu]
ar = "x86_64-pc-linux-gnu-ar"
cc = "x86_64-pc-linux-gnu-gcc"
cxx = "x86_64-pc-linux-gnu-g++"
linker = "x86_64-pc-linux-gnu-gcc"
ranlib = "x86_64-pc-linux-gnu-ranlib"
[target.wasm32-unknown-unknown]
linker = "rust-lld"
