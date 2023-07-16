plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:340bfcc0
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
Resolving deltas: 100% (7654/7654), done.
travis_time:end:340bfcc0:start=1540239404934436557,finish=1540239412039311187,duration=7104874630
$ cd rust-lang/rust
$ git checkout -qf 621ca4e381532a956260abc1675160714542804f
fatal: reference is not a tree: 621ca4e381532a956260abc1675160714542804f
The command "git checkout -qf 621ca4e381532a956260abc1675160714542804f" failed and exited with 128 during .
Your build has been stopped.
