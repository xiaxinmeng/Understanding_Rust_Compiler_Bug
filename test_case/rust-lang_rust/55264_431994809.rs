plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:00e7ba4e
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
Resolving deltas: 100% (7937/7937), done.
travis_time:end:00e7ba4e:start=1540241159474675330,finish=1540241165516488837,duration=6041813507
$ cd rust-lang/rust
$ git checkout -qf 0c6e1283b0a9fe23e54018a5dcdd3ec6d84d1896
fatal: reference is not a tree: 0c6e1283b0a9fe23e54018a5dcdd3ec6d84d1896
The command "git checkout -qf 0c6e1283b0a9fe23e54018a5dcdd3ec6d84d1896" failed and exited with 128 during .
Your build has been stopped.
