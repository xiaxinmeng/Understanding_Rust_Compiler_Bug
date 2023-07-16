
[build]
build = "x86_64-unknown-linux-gnu"
host = ["x86_64-unknown-linux-gnu"]
target = ["x86_64-unknown-linux-gnu","aarch64-unknown-linux-gnu"]
cargo = "/var/tmp/portage/dev-lang/rust-1.32.0/work/rust-stage0/bin/cargo"
rustc = "/var/tmp/portage/dev-lang/rust-1.32.0/work/rust-stage0/bin/rustc"
docs = false
submodules = false
python = "python3.6"
locked-deps = true
vendor = true
extended = true
tools = ["cargo",]
[install]
prefix = "/usr"
libdir = "lib64/rust-1.32.0"
docdir = "share/doc/rust-1.32.0"
mandir = "share/rust-1.32.0/man"
[rust]
optimize = true
debuginfo = false
debug-assertions = false
default-linker = "x86_64-pc-linux-gnu-gcc"
channel = "stable"
rpath = false
lld = false
[target.x86_64-unknown-linux-gnu]
cc = "x86_64-pc-linux-gnu-gcc"
cxx = "x86_64-pc-linux-gnu-g++"
linker = "x86_64-pc-linux-gnu-gcc"
ar = "x86_64-pc-linux-gnu-ar"
llvm-config = "/usr/lib/llvm/7/bin/llvm-config"
[target.aarch64-unknown-linux-gnu]
cc = "aarch64-unknown-linux-gnu-gcc"
cxx = "aarch64-unknown-linux-gnu-g++"
linker = "aarch64-unknown-linux-gnu-gcc"
ar = "aarch64-unknown-linux-gnu-ar"
