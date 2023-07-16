
[01:02:34] failures:
[01:02:34] 
[01:02:34] ---- net/tcp.rs - net::tcp::TcpListener (line 75) stdout ----
[01:02:34] 	error[E0434]: can't capture dynamic environment in a fn item; use the || { ... } closure form instead
[01:02:34]   --> <anon>:15:15
[01:02:34]    |
[01:02:34] 15 | for stream in listener.incoming() {
[01:02:34]    |               ^^^^^^^^
[01:02:34] 
[01:02:34] error: aborting due to previous error(s)
[01:02:34] 
[01:02:34] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:216
[01:02:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:02:34] 
[01:02:34] 
[01:02:34] failures:
[01:02:34]     net/tcp.rs - net::tcp::TcpListener (line 75)
[01:02:34] 
[01:02:34] test result: FAILED. 786 passed; 1 failed; 22 ignored; 0 measured
[01:02:34] 
[01:02:34] error: test failed
