console
# Fails on latest nightly
$ rustc +nightly -vV
rustc 1.71.0-nightly (74c482104 2023-05-04)
binary: rustc
commit-hash: 74c4821045c68d42bb8b8a7c998bdb5c2a72bd0d
commit-date: 2023-05-04
host: aarch64-apple-darwin
release: 1.71.0-nightly
LLVM version: 16.0.2
$ rustc +nightly -O -Zverify-llvm-ir issue-105439.rs
Invalid bitcast
  %16 = bitcast <4 x i32> %15 to [4 x i32]
LLVM ERROR: Broken module found, compilation aborted!

# Passes with PR
$ rustc +stage1 -O -Zverify-llvm-ir issue-105439.rs
$ ./issue-105439 && echo $?
0
