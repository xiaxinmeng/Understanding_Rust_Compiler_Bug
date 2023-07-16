
[build]
rustc = "/home/semarie/repos/openbsd/ports/pobj/rust-1.18.0/rustc-bootstrap-amd64-1.18.0-20170611/bin/rustc"
cargo = "/home/semarie/repos/openbsd/ports/pobj/rust-1.18.0/cargo-bootstrap-amd64-0.19.0-20170611/cargo"
prefix = "/usr/local"
vendor = true
[rust]
channel = "stable"
codegen-tests = false
[dist]
src-tarball = false
[target.x86_64-unknown-openbsd]
llvm-config = "/usr/local/bin/llvm-config"
