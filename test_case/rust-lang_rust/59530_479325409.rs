plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:35c20c74
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
Cloning into 'rust-lang/rust'...
travis_time:end:35c20c74:start=1554263240349952646,finish=1554263246309018077,duration=5959065431
$ cd rust-lang/rust
$ git checkout -qf 964bc281eed9c8f4fcafb5131845baa7ee00ddc8
fatal: reference is not a tree: 964bc281eed9c8f4fcafb5131845baa7ee00ddc8
The command "git checkout -qf 964bc281eed9c8f4fcafb5131845baa7ee00ddc8" failed and exited with 128 during .
Your build has been stopped.
