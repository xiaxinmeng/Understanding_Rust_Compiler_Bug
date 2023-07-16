toml
[llvm]
optimize = true
release-debuginfo = false
assertions = false
ccache = "/usr/bin/ccache"
static-libstdcpp = false
ninja = true
targets = "X86"
link-jobs = 4
[build]
build = "x86_64-unknown-linux-gnu"
host = ["x86_64-unknown-linux-gnu"]
target = ["x86_64-unknown-linux-gnu"]
cargo = "/home/xftroxgpx/.cargo/bin/cargo"
rustc = "/home/xftroxgpx/.cargo/bin/rustc"
docs = true
compiler-docs = true
submodules = true
locked-deps = false
full-bootstrap = false
extended = false
verbose = 0
sanitizers = false
profiler = false
openssl-static = false
low-priority = true
[install]
[rust]
optimize = true
codegen-units = 1 #We currently have the capability to do multiple codegen units in parallel. Unfortunately, one drawback of using this functionality is that using multiple codegen units loses optimization opportunities, like inlining, between the units. src: https://internals.rust-lang.org/t/towards-a-second-edition-of-the-compiler/5582
debug-assertions = false
debuginfo = true
debuginfo-lines = true
debuginfo-only-std = false
debug-jemalloc = false
backtrace = true
channel = "dev"
rpath = true
optimize-tests = false
debuginfo-tests = false
codegen-tests = false
ignore-git = false
[target.x86_64-unknown-linux-gnu]
[dist]
src-tarball = false
