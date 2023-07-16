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
link-jobs = 4
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
extended = false
tools = [ "src" ]
verbose = 0
sanitizers = false
profiler = false
low-priority = true
local-rebuild = false
print-step-timings = false
[install]
[rust]
optimize = true
debug = false
codegen-units = 1
codegen-units-std = 1
debug-assertions = true #required!
debuginfo = false
debuginfo-lines = false
debuginfo-only-std = false
debuginfo-tools = false
backtrace = true
incremental = true
experimental-parallel-queries = true
channel = "dev"
rpath = true
verbose-tests = false
optimize-tests = false
debuginfo-tests = false
codegen-tests = false
ignore-git = false
lld = false
deny-warnings = true
backtrace-on-ice = true
[target.x86_64-unknown-linux-gnu]
[dist]
src-tarball = false
missing-tools = false
