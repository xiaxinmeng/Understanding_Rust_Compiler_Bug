shell
> cargo clone ordered-float
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading ordered-float v0.5.0
> cd .\ordered-float\
> cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading num-traits v0.1.42
   Compiling num-traits v0.1.42
   Compiling void v1.0.2
   Compiling unreachable v0.1.1
   Compiling ordered-float v0.5.0 (file:///C:/Users/steve/tmp/ordered-float)
    Finished dev [unoptimized + debuginfo] target(s) in 4.53 secs
> rustc -vV
rustc 1.25.0-nightly (b5392f545 2018-01-08)
binary: rustc
commit-hash: b5392f54503fdaf04df4b9578510b2baa944f4af
commit-date: 2018-01-08
host: x86_64-pc-windows-msvc
release: 1.25.0-nightly
LLVM version: 4.0
> cargo +stable build
   Compiling num-traits v0.1.42
   Compiling void v1.0.2
   Compiling unreachable v0.1.1
   Compiling ordered-float v0.5.0 (file:///C:/Users/steve/tmp/ordered-float)
    Finished dev [unoptimized + debuginfo] target(s) in 3.99 secs
> rustc +stable -vV
rustc 1.23.0 (766bd11c8 2018-01-01)
binary: rustc
commit-hash: 766bd11c8a3c019ca53febdcd77b2215379dd67d
commit-date: 2018-01-01
host: x86_64-pc-windows-msvc
release: 1.23.0
LLVM version: 4.0
