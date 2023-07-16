plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:114f52e8
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
Cloning into 'rust-lang/rust'...
travis_time:end:114f52e8:start=1530235919998108762,finish=1530235925383248501,duration=5385139739
$ cd rust-lang/rust
$ git checkout -qf fbf3a9f0e182e3e202e674cc355ad3598956bb90
fatal: reference is not a tree: fbf3a9f0e182e3e202e674cc355ad3598956bb90
The command "git checkout -qf fbf3a9f0e182e3e202e674cc355ad3598956bb90" failed and exited with 128 during .
Your build has been stopped.
