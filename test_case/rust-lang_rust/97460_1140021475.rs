
$ cat Cargo.toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

[profile.dev]
opt-level = 1

$ strace -e execve -f -- cargo +1.59 test
...
[pid 12014] execve("/home/wesley/.rustup/toolchains/1.59-x86_64-unknown-linux-gnu/bin/rustc", ["/home/wesley/.rustup/toolchains/"..., "--crate-name", "hello_world", "--edition=2021", "src/main.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,"..., "--emit=dep-info,link", "-C", "opt-level=1", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "debug-assertions=on", "--test", "-C", "metadata=79aa5460ab0a171e", "-C", "extra-filename=-79aa5460ab0a171e", "--out-dir", "/tmp/hello_world/target/debug/de"..., "-C", "incremental=/tmp/hello_world/tar"..., "-L", "dependency=/tmp/hello_world/targ"...], 0x56026762fa60 /* 52 vars */) = 0
...
