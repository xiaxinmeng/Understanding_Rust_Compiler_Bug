
[llvm]
optimize = true
release-debuginfo = false
assertions = false
targets = "X86"
[build]
build = "i686-unknown-linux-gnu"
host = ["i686-unknown-linux-gnu"]
target = ["i686-unknown-linux-gnu"]
cargo = "/var/tmp/portage/dev-lang/rust-1.32.0/work/rust-stage0/bin/cargo"
rustc = "/var/tmp/portage/dev-lang/rust-1.32.0/work/rust-stage0/bin/rustc"
docs = false
submodules = false
python = "python3.6"
locked-deps = true
vendor = true
extended = false
tools = []
[install]
prefix = "/usr"
libdir = "lib/rust-1.32.0"
docdir = "share/doc/rust-1.32.0"
mandir = "share/rust-1.32.0/man"
[rust]
optimize = true
debuginfo = false
debug-assertions = false
use-jemalloc = true
default-linker = "i686-pc-linux-gnu-gcc"
channel = "stable"
rpath = false
lld = false
[target.]
cc = "i686-pc-linux-gnu-gcc"
cxx = "i686-pc-linux-gnu-g++"
linker = "i686-pc-linux-gnu-gcc"
ar = "i686-pc-linux-gnu-ar"
