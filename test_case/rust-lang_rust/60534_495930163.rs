toml
[llvm]
optimize = true
release-debuginfo = false #faster
assertions = false #faster
ccache = "/usr/bin/ccache" #faster
static-libstdcpp = false
ninja = true
targets = "X86"
experimental-targets = ""
link-jobs = 0
[build]
build = "x86_64-unknown-linux-gnu"
host = ["x86_64-unknown-linux-gnu"]
target = ["x86_64-unknown-linux-gnu"] # defaults to just the build triple
docs = true
compiler-docs = false #faster ./x.py install XXX: undo
submodules = true #this and/or fast-submodules=true below, will overwrite changes presumably made by eg. git checkout d6525ef539 in the root rustc/ dir and thus I cannot properly roll back to that working rustc commit because submodules are in fact brought up to date automatically, so still hitting this issue: https://github.com/rust-lang/rust/issues/57596
fast-submodules = true
locked-deps = false
full-bootstrap = false
extended = false
tools = [ "src", "rls"]
verbose = 0
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
debug = false #faster
codegen-units = 1 #We currently have the capability to do multiple codegen units in parallel. Unfortunately, one drawback of using this functionality is that using multiple codegen units loses optimization opportunities, like inlining, between the units. src: https://internals.rust-lang.org/t/towards-a-second-edition-of-the-compiler/5582
codegen-units-std = 1
debug-assertions = false #WAY(?) faster
debuginfo-level = 2
debuginfo-level-rustc = 2
debuginfo-level-std = 2
debuginfo-level-tools = 2
backtrace = true
incremental = true #faster but hitting this issue after each update which makes it useless: https://github.com/rust-lang/rust/issues/58633
parallel-compiler = true #use false or else: https://github.com/rust-lang/rust/issues/61162#issuecomment-495888785
channel = "dev"
rpath = true
verbose-tests = false
optimize-tests = true
codegen-tests = false #for no good reason; just to be sure
ignore-git = false #i don't get it! what git information and why will it cause rebuilds?
lld = false
deny-warnings = false #workaround for above
backtrace-on-ice = true
llvm-libunwind = false #default
[target.x86_64-unknown-linux-gnu]
[dist]
src-tarball = false
missing-tools = false
