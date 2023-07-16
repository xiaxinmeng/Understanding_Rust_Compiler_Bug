
[llvm]
targets = "X86"
link-shared = true

[build]
docs = false
extended = true
verbose = 2

[install]
prefix = "/home/bsferraz/tools"
sysconfdir = "etc"
docdir = "share/doc/rustc-1.64.0"

[rust]
channel = "stable"
rpath = false
codegen-tests = false

[target.x86_64-unknown-linux-gnu]
llvm-config = "/home/bsferraz/tools/bin/llvm-config"
