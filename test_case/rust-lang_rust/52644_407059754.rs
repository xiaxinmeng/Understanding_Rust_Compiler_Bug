plain
[00:19:44]    Compiling rustc_apfloat v0.0.0 (file:///checkout/src/librustc_apfloat)
[00:19:46]    Compiling crossbeam-deque v0.2.0
[00:19:46]    Compiling parking_lot v0.5.5
[00:19:47]    Compiling rls-data v0.16.0
[00:19:48] error: the feature `fs_read_write` has been stable since 1.26.0 and no longer requires an attribute to enable
[00:19:48]   --> librustc_target/lib.rs:32:12
[00:19:48]    |
[00:19:48] 32 | #![feature(fs_read_write)]
[00:19:48]    |
[00:19:48]    = note: `-D stable-features` implied by `-D warnings`
[00:19:48] 
[00:19:48] 
[00:19:48] error: the feature `inclusive_range` has been stable since 1.26.0 and no longer requires an attribute to enable
[00:19:48]   --> librustc_target/lib.rs:33:12
[00:19:48] 33 | #![feature(inclusive_range)]
[00:19:48]    |            ^^^^^^^^^^^^^^^
[00:19:48] 
[00:19:48]    Compiling backtrace v0.3.9
