
$ rustc +nightly --version
rustc 1.57.0-nightly (ac2d9fc50 2021-09-21)

$ RUSTFLAGS="-C split-debuginfo=packed -Zunstable-options -C save-temps" cargo +nightly build --release

   Compiling xtask v0.1.0 (/home/philipc/code/rust/rust-analyzer/xtask)
error: linking dwarf objects with `rust-llvm-dwp` failed: exit status: 1
  |
  = note: "rust-llvm-dwp" "-e" "/home/philipc/code/rust/rust-analyzer/target/release/deps/xtask-be398824d5b61cec" "-o" "/home/philipc/code/rust/rust-analyzer/target/release/deps/xtask-be398824d5b61cec.dwp"
  = note: 
  = note: error: duplicate DWO ID (C6ED43DCB82FE226) in 'xtask/src/main.rs' and 'xtask/src/main.rs'
