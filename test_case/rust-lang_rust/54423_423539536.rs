console
$ cargo +nightly doc --all-features --target x86_64-pc-windows-gnu
    Updating crates.io index
   Compiling winapi-x86_64-pc-windows-gnu v0.4.0
   Compiling winapi v0.3.6 (/home/misdreavus/clones/winapi)
 Documenting winapi-x86_64-pc-windows-gnu v0.4.0
 Documenting winapi v0.3.6 (/home/misdreavus/clones/winapi)
    Finished dev [unoptimized + debuginfo] target(s) in 1m 16s
$ rustc +nightly -vV
rustc 1.30.0-nightly (3bc2ca7e4 2018-09-20)
binary: rustc
commit-hash: 3bc2ca7e4f8507f82a4c357ee19300166bcd8099
commit-date: 2018-09-20
host: x86_64-unknown-linux-gnu
release: 1.30.0-nightly
LLVM version: 8.0
