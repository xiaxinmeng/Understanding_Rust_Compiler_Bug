
[01:03:01] failures:
[01:03:01] 
[01:03:01] ---- vec.rs - vec::Vec<T>::resize_default (line 1321) stdout ----
[01:03:01] 	error: use of unstable library feature 'vec_resize_default' (see issue #41758)
[01:03:01]  --> <anon>:5:5
[01:03:01]   |
[01:03:01] 5 | vec.resize_default(5);
[01:03:01]   |     ^^^^^^^^^^^^^^
[01:03:01]   |
[01:03:01]   = help: add #![feature(vec_resize_default)] to the crate attributes to enable
[01:03:01] 
[01:03:01] error: use of unstable library feature 'vec_resize_default' (see issue #41758)
[01:03:01]  --> <anon>:9:5
[01:03:01]   |
[01:03:01] 9 | vec.resize_default(2);
[01:03:01]   |     ^^^^^^^^^^^^^^
[01:03:01]   |
[01:03:01]   = help: add #![feature(vec_resize_default)] to the crate attributes to enable
[01:03:01] 
[01:03:01] error: aborting due to previous error(s)
[01:03:01] 
[01:03:01] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:216
[01:03:01] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:01] 
[01:03:01] 
[01:03:01] failures:
[01:03:01]     vec.rs - vec::Vec<T>::resize_default (line 1321)
[01:03:01] 
[01:03:01] test result: FAILED. 404 passed; 1 failed; 4 ignored; 0 measured
[01:03:01] 
[01:03:01] error: test failed, to rerun pass '--doc'
[01:03:01] 
