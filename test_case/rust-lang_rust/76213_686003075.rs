`toml
[llvm]
thin-lto = true
ninja = true
targets = "X86"
link-jobs = 2
[build]
extended = true
tools = ["cargo", "rls", "clippy", "rustfmt", "analysis", "src", "miri", "rust-analyzer"]
sanitizers = true
profiler = true
low-priority = true
[install]
[rust]
codegen-units = 0
backtrace-on-ice = true
[target.x86_64-unknown-linux-gnu]
ar = "llvm-ar"
ranlib = "llvm-ranlib"
[dist]
