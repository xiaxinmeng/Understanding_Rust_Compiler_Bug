
$ cat build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-ebd551ef66e29972/output | grep rerun
cargo:rerun-if-changed=/home/vagrant/repos/rust/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-config
cargo:rerun-if-changed=/home/vagrant/repos/rust/config.toml
cargo:rerun-if-changed=../rustllvm/ArchiveWrapper.cpp
cargo:rerun-if-changed=../rustllvm/llvm-rebuild-trigger
cargo:rerun-if-changed=../rustllvm/RustWrapper.cpp
cargo:rerun-if-changed=../rustllvm/rustllvm.h
cargo:rerun-if-changed=../rustllvm/README
cargo:rerun-if-changed=../rustllvm/PassWrapper.cpp
