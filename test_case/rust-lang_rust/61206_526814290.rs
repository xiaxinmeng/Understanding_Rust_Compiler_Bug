toml
[llvm]
optimize = true
release-debuginfo = false
assertions = false
targets = "AMDGPU;PowerPC"
experimental-targets = ""
link-shared = false
[build]
build = "powerpc64le-unknown-linux-gnu"
host = ["powerpc64le-unknown-linux-gnu"]
target = ["powerpc64le-unknown-linux-gnu"]
cargo = "/var/tmp/portage/dev-lang/rust-1.37.0/work/rust-stage0/bin/cargo"
rustc = "/var/tmp/portage/dev-lang/rust-1.37.0/work/rust-stage0/bin/rustc"
docs = false
submodules = false
python = "python3.6"
locked-deps = true
vendor = true
extended = true
tools = ["rustfmt","rls","analysis","src","clippy","cargo",]
verbose = 2
[install]
prefix = "/usr"
libdir = "lib64/rust-1.37.0"
docdir = "share/doc/rust-1.37.0"
mandir = "share/rust-1.37.0/man"
[rust]
optimize = true
debug = false
debug-assertions = false
default-linker = "powerpc64le-unknown-linux-gnu-gcc"
channel = "stable"
rpath = false
lld = false
[target.powerpc64le-unknown-linux-gnu]
cc = "powerpc64le-unknown-linux-gnu-gcc"
cxx = "powerpc64le-unknown-linux-gnu-g++"
linker = "powerpc64le-unknown-linux-gnu-gcc"
ar = "powerpc64le-unknown-linux-gnu-ar"
