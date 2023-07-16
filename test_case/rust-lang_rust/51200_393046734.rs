plain
[00:41:45] .....................................................................i..............................
[00:41:49] ....................................................................................................
[00:41:54] ....................................................................................................
[00:42:00] ..................................................................................................i.
[00:42:03] ................iiiiiiiii...................................................
[00:42:03] 
[00:42:03] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:42:50] .....................................................................i..............................
[00:42:54] ....................................................................................................
[00:42:58] ....................................................................................................
[00:43:04] ..................................................................................................i.
[00:43:07] ................iiiiiiiii...................................................
[00:43:07] 
[00:43:07]  finished in 63.865
[00:43:07] travis_fold:end:test_ui_nll

---
[01:03:37] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:37]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[01:03:45] error[E0277]: the trait bound `std::iter::RepeatWith<[closure@libcore/../libcore/tests/iter.rs:1728:32: 1728:69 curr:_]>: std::iter::DoubleEndedIterator` is not satisfied
[01:03:45]     --> libcore/../libcore/tests/iter.rs:1729:22
[01:03:45]      |
[01:03:45] 1729 |                     .rev().take(4);
[01:03:45]      |                      ^^^ the trait `std::iter::DoubleEndedIterator` is not implemented for `std::iter::RepeatWith<[closure@libcore/../libcore/tests/iter.rs:1728:32: 1728:69 curr:_]>`
[01:03:45] 
[01:03:45] error[E0599]: no method named `take` found for type `std::iter::Rev<std::iter::RepeatWith<[closure@libcore/../libcore/tests/iter.rs:1728:32: 1728:69 curr:_]>>` in the current scope
[01:03:45]     --> libcore/../libcore/tests/iter.rs:1729:28
[01:03:45]      |
[01:03:45] 1729 |                     .rev().take(4);
[01:03:45]      |
[01:03:45]      |
[01:03:45]      = note: the method `take` exists but the following trait bounds were not satisfied:
[01:03:45]              `std::iter::Rev<std::iter::RepeatWith<[closure@libcore/../libcore/tests/iter.rs:1728:32: 1728:69 curr:_]>> : std::iter::Iterator`
[01:03:45]              `&mut std::iter::Rev<std::iter::RepeatWith<[closure@libcore/../libcore/tests/iter.rs:1728:32: 1728:69 curr:_]>> : std::iter::Iterator`
[01:03:51] error: aborting due to 2 previous errors
[01:03:51] 
[01:03:51] Some errors occurred: E0277, E0599.
[01:03:51] For more information about an error, try `rustc --explain E0277`.
