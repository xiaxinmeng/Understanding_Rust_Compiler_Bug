
failures:
[01:02:27] 
[01:02:27] ---- thread/mod.rs - thread::Thread::id (line 791) stdout ----
[01:02:27] 	error: no method named `unwrap` found for type `std::thread::JoinHandle<std::thread::ThreadId>` in the current scope
[01:02:27]   --> <anon>:10:4
[01:02:27]    |
[01:02:27] 10 | }).unwrap();
[01:02:27]    |    ^^^^^^
[01:02:27] 
[01:02:27] error: aborting due to previous error(s)
[01:02:27] 
[01:02:27] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:203
[01:02:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:02:27] 
[01:02:27] ---- thread/mod.rs - thread::ThreadId (line 660) stdout ----
[01:02:27] 	error: no method named `unwrap` found for type `std::thread::JoinHandle<std::thread::ThreadId>` in the current scope
[01:02:27]   --> <anon>:10:4
[01:02:27]    |
[01:02:27] 10 | }).unwrap();
[01:02:27]    |    ^^^^^^
[01:02:27] 
[01:02:27] error: aborting due to previous error(s)
[01:02:27] 
[01:02:27] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:203
[01:02:27] 
[01:02:27] 
[01:02:27] failures:
[01:02:27]     thread/mod.rs - thread::Thread::id (line 791)
[01:02:27]     thread/mod.rs - thread::ThreadId (line 660)
[01:02:27] 
[01:02:27] test result: FAILED. 776 passed; 2 failed; 22 ignored; 0 measured
[01:02:27] 
[01:02:27] error: test failed
