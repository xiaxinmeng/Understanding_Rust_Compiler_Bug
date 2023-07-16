
[01:01:42] failures:
[01:01:42] 
[01:01:42] ---- path.rs - path::PathBuf (line 1076) stdout ----
[01:01:42] 	error: unknown character escape: s
[01:01:42]  --> <anon>:6:39
[01:01:42]   |
[01:01:42] 6 | let path = PathBuf::from("c:\\windows\system32.dll");
[01:01:42]   |                                       ^
[01:01:42] 
[01:01:42] error: aborting due to previous error(s)
[01:01:42] 
[01:01:42] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:216
[01:01:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:42] 
[01:01:42] 
[01:01:42] failures:
[01:01:42]     path.rs - path::PathBuf (line 1076)
[01:01:42] 
[01:01:42] test result: FAILED. 783 passed; 1 failed; 22 ignored; 0 measured
[01:01:42] 
[01:01:42] error: test failed
[01:01:42] 
