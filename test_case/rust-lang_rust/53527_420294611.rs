
[00:55:31] error[E0308]: mismatched types
[00:55:31]    --> C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\compiletest_rs-0.3.13\src\lib.rs:100:22
[00:55:31]     |
[00:55:31] 100 |         run_ignored: config.run_ignored,
[00:55:31]     |                      ^^^^^^^^^^^^^^^^^^ expected enum `test::RunIgnored`, found bool
[00:55:31]     |
[00:55:31]     = note: expected type `test::RunIgnored`
[00:55:31]                found type `bool`
[00:55:31] 
[00:55:32] error: aborting due to previous error
