 toml
# Xargo.toml
[target.x86_64-unknown-linux-gnu.dependencies.std]
features = ["panic-unwind"]

[target.x86_64-unknown-linux-gnu.dependencies.rustc_lsan]
path = "/path/to/rustc_lsan"  # this could be `git` as well
