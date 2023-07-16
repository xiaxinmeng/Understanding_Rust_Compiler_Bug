plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0783d488
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
Cloning into 'rust-lang/rust'...
travis_time:end:0783d488:start=1558435113273512685,finish=1558435119334606568,duration=6061093883
$ cd rust-lang/rust
$ git checkout -qf ec4deee08e13106e7de91b1b630c63ae2774d6c1
fatal: reference is not a tree: ec4deee08e13106e7de91b1b630c63ae2774d6c1
The command "git checkout -qf ec4deee08e13106e7de91b1b630c63ae2774d6c1" failed and exited with 128 during .
Your build has been stopped.
