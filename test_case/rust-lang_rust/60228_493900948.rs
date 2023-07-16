toml
[llvm]
[build]
submodules = true
extended = true
tools = ["cargo", "rls", "clippy", "rustfmt", "analysis", "src"]
sanitizers = true
profiler = true
low-priority = true

rustc = "/home/pnkfelix/Dev/Mozilla/issue60228/rust-60228-bootstrap/build/x86_64-unknown-linux-gnu/stage1/bin/rustc"

[install]
[rust]
[target.x86_64-unknown-linux-gnu]
[dist]
