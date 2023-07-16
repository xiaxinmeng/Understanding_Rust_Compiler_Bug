
[01:06:36] failures:
[01:06:36] 
[01:06:36] ---- io/mod.rs - io (line 157) stdout ----
[01:06:36] 	error[E0433]: failed to resolve. Use of undeclared type or module `io`
[01:06:36]  --> <anon>:6:1
[01:06:36]   |
[01:06:36] 6 | io::stdin().read_line(&mut input).unwrap();
[01:06:36]   | ^^^^^^^^^ Use of undeclared type or module `io`
[01:06:36] 
[01:06:36] error: aborting due to previous error(s)
[01:06:36] 
[01:06:36] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:203
[01:06:36] note: Run with `RUST_BACKTRACE=1` for a backtrace.
