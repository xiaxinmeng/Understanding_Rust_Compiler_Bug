plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:12b7d074
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
Cloning into 'rust-lang/rust'...
travis_time:end:12b7d074:start=1558433607451298324,finish=1558433613657120363,duration=6205822039
$ cd rust-lang/rust
$ git checkout -qf 0f90b7b859c08fbf9807af5984090f26297e5933
fatal: reference is not a tree: 0f90b7b859c08fbf9807af5984090f26297e5933
The command "git checkout -qf 0f90b7b859c08fbf9807af5984090f26297e5933" failed and exited with 128 during .
Your build has been stopped.
