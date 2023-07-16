
$ cargo --version                          
cargo 1.66.1 (ad779e08b 2023-01-10)

$ cargo +nightly rustc -- --version
balaba.....
rustc 1.68.0-nightly (3984bc583 2023-01-17)


$ cargo +nightly build --release                                          
   Compiling github_issue_test v0.1.0 (/home/gngshn/develop/rust/github_issue_test)
error: linking with `cc` failed: exit status: 1
  |
  = note: balabalbala............
  = note: /home/gngshn/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-cb19371b39fc63d8.rlib(alloc-cb19371b39fc63d8.alloc.ea0e1c85-cgu.0.rcgu.o):(.data.DW.ref.rust_eh_personality[DW.ref.rust_eh_personality]+0x0): undefined reference to `rust_eh_personality'
          collect2: error: ld returned 1 exit status
          
  = note: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
  = note: use the `-l` flag to specify native libraries to link
  = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)


$ cargo +nightly build -Zbuild-std=core,alloc --release --target x86_64-unknown-linux-gnu
   Compiling core v0.0.0 (/home/gngshn/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   Compiling compiler_builtins v0.1.85
   Compiling rustc-std-workspace-core v1.99.0 (/home/gngshn/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling alloc v0.0.0 (/home/gngshn/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc)
   Compiling github_issue_test v0.1.0 (/home/gngshn/develop/rust/github_issue_test)
    Finished release [optimized] target(s) in 9.77s


$ cargo +nightly-2022-12-29 build --release                       
   Compiling github_issue_test v0.1.0 (/home/gngshn/develop/rust/github_issue_test)
    Finished release [optimized] target(s) in 0.29s
