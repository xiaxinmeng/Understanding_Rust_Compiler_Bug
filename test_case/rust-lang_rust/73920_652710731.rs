
$ rustc -vV
rustc 1.46.0-nightly (16957bd4d 2020-06-30)
binary: rustc
commit-hash: 16957bd4d3a5377263f76ed74c572aad8e4b7e59
commit-date: 2020-06-30
host: x86_64-pc-windows-gnu
release: 1.46.0-nightly
LLVM version: 10.0

$ rustc powi2.rs && ./powi2
5e-324
5e-324
5e-324

$ rustc powi2.rs -O && ./powi2
5e-324
5e-324
5e-324

$ rustc powi2.rs --target x86_64-pc-windows-msvc && ./powi2
5e-324
5e-324
5e-324

$ rustc powi2.rs --target x86_64-pc-windows-msvc -O && ./powi2
5e-324
5e-324
5e-324
