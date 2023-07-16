plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:1c1a3477
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
Cloning into 'rust-lang/rust'...
travis_time:end:1c1a3477:start=1561241805895108140,finish=1561241811474866321,duration=5579758181
$ cd rust-lang/rust
$ git checkout -qf 03b8d2800f299e13f300fa658b68ddaee57d3a77
fatal: reference is not a tree: 03b8d2800f299e13f300fa658b68ddaee57d3a77
The command "git checkout -qf 03b8d2800f299e13f300fa658b68ddaee57d3a77" failed and exited with 128 during .
Your build has been stopped.
