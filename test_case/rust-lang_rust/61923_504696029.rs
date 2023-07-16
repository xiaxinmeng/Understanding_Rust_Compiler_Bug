plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0d3a6aa8
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
Cloning into 'rust-lang/rust'...
travis_time:end:0d3a6aa8:start=1561235299045578076,finish=1561235304535555566,duration=5489977490
$ cd rust-lang/rust
$ git checkout -qf 739ec977abef6de9ed58fcc2e0cc822138c7ceab
fatal: reference is not a tree: 739ec977abef6de9ed58fcc2e0cc822138c7ceab
The command "git checkout -qf 739ec977abef6de9ed58fcc2e0cc822138c7ceab" failed and exited with 128 during .
Your build has been stopped.
