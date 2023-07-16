toml
[llvm]
optimize = true
release-debuginfo = false 
assertions = false 
ccache = "/usr/bin/ccache" 
static-libstdcpp = false
ninja = true
targets = "X86"
experimental-targets = ""
link-jobs = 0
[build]
build = "x86_64-unknown-linux-gnu"
host = ["x86_64-unknown-linux-gnu"]
target = ["x86_64-unknown-linux-gnu"] 
docs = true
compiler-docs = false 
submodules = true 
fast-submodules = true
locked-deps = false
full-bootstrap = false
extended = true 
tools = [ "src", "clippy", "rustfmt", "rls" ]
verbose = 2
sanitizers = false
profiler = false
low-priority = true
local-rebuild = false
print-step-timings = true
[install]
prefix = "/home/user/build/2nonpkgs/rust.stuff/rust/rust.installed.dir"
bindir = "bin"
[rust]
optimize = true
debug = false 
codegen-units = 1 
codegen-units-std = 1
debug-assertions = false 
debuginfo-level = 2
debuginfo-level-rustc = 2
debuginfo-level-std = 2
debuginfo-level-tools = 2
backtrace = true
incremental = true 
parallel-compiler = true 
channel = "dev"
rpath = true
verbose-tests = false
optimize-tests = true
codegen-tests = false 
ignore-git = false 
lld = false
deny-warnings = true 
backtrace-on-ice = true
llvm-libunwind = false 
[target.x86_64-unknown-linux-gnu]
[dist]
src-tarball = false
missing-tools = false
