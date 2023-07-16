
pnkfelix@videolan-ubuntu-be1:~/Mozilla/issue-50960$ cat Cargo.toml
[package]
name = "issue-50960"
version = "0.1.0"
authors = ["pnkfelix"]

[dependencies]
rand = "0.4.2"

pnkfelix@videolan-ubuntu-be1:~/Mozilla/issue-50960$ rustup default nightly-2018-02-24 && cargo clean && cargo build
info: using existing install for 'nightly-2018-02-24-powerpc-unknown-linux-gnu'
info: default toolchain set to 'nightly-2018-02-24-powerpc-unknown-linux-gnu'

  nightly-2018-02-24-powerpc-unknown-linux-gnu unchanged - rustc 1.26.0-nightly (063deba92 2018-02-23)

   Compiling libc v0.2.41
   Compiling rand v0.4.2
   Compiling issue-50960 v0.1.0 (file:///home/pnkfelix/Mozilla/issue-50960)
    Finished dev [unoptimized + debuginfo] target(s) in 6.6 secs
pnkfelix@videolan-ubuntu-be1:~/Mozilla/issue-50960$ rustup default nightly-2018-02-25 && cargo clean && cargo build
info: using existing install for 'nightly-2018-02-25-powerpc-unknown-linux-gnu'
info: default toolchain set to 'nightly-2018-02-25-powerpc-unknown-linux-gnu'

  nightly-2018-02-25-powerpc-unknown-linux-gnu unchanged - rustc 1.26.0-nightly (28a1e4ffe 2018-02-24)

   Compiling libc v0.2.41
   Compiling rand v0.4.2
Expected no forward declarations!
!189 = <temporary!> !{}
scope points into the type hierarchy
!191 = !DILocation(line: 1, scope: !186)
scope points into the type hierarchy
!193 = !DILocation(line: 464, scope: !186)
scope points into the type hierarchy
!194 = !DILocation(line: 465, scope: !186)
LLVM ERROR: Broken function found, compilation aborted!
error: Could not compile `rand`.

To learn more, run the command again with --verbose.
