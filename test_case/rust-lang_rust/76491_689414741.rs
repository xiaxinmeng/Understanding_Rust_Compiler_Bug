
$ CARGO_TARGET_I686_PC_WINDOWS_GNU_LINKER=rust-lld cargo +nightly-i686-pc-windows-gnu build
   Compiling hello v0.1.0 (D:\tmp\hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.66s

$ rustc +nightly-i686-pc-windows-gnu -vV
rustc 1.48.0-nightly (5099914a1 2020-09-08)
binary: rustc
commit-hash: 5099914a16a215794ad243df0cc7a05d91d168e0
commit-date: 2020-09-08
host: i686-pc-windows-gnu
release: 1.48.0-nightly
LLVM version: 11.0
