plain
travis_time:end:09ed7d20:start=1547001746166781524,finish=1547001748289823824,duration=2123042300
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:49]    Compiling jobserver v0.1.12
[00:05:49]    Compiling memmap v0.6.2
[00:05:50]    Compiling rand v0.6.1
[00:05:50]    Compiling parking_lot_core v0.3.0
[00:05:50] error[E0658]: use of unstable library feature 'renamed_spin_loop' (see issue #55002)
[00:05:50]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/parking_lot_core-0.3.0/src/spinwait.rs:10:5
[00:05:50] 10 | use std::sync::atomic::spin_loop_hint;
[00:05:50]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:05:50]    |
[00:05:50]    = help: add #![feature(renamed_spin_loop)] to the crate attributes to enable
[00:05:50]    = help: add #![feature(renamed_spin_loop)] to the crate attributes to enable
[00:05:50] 
[00:05:50] error[E0658]: use of unstable library feature 'renamed_spin_loop' (see issue #55002)
[00:05:50]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/parking_lot_core-0.3.0/src/spinwait.rs:61:9
[00:05:50] 61 |         spin_loop_hint()
[00:05:50]    |         ^^^^^^^^^^^^^^
[00:05:50]    |
[00:05:50]    = help: add #![feature(renamed_spin_loop)] to the crate attributes to enable
