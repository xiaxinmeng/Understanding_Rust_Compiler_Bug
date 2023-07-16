
[00:58:56] failures:
[00:58:56] 
[00:58:56] ---- path.rs - path::Path::is_dir (line 2278) stdout ----
[00:58:56] 	error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `Also`
[00:58:56]  --> <anon>:7:5
[00:58:56]   |
[00:58:56] 7 | See Also
[00:58:56]   |    -^^^^ unexpected token
[00:58:56]   |    |
[00:58:56]   |    expected one of 8 possible tokens here
[00:58:56] 
[00:58:56] error[E0425]: cannot find value `See` in this scope
[00:58:56]  --> <anon>:7:1
[00:58:56]   |
[00:58:56] 7 | See Also
[00:58:56]   | ^^^ not found in this scope
[00:58:56] 
[00:58:56] thread 'rustc' panicked at 'couldn't compile the test', /checkout/src/librustdoc/test.rs:273:12
[00:58:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:56] 
[00:58:56] 
[00:58:56] failures:
[00:58:56]     path.rs - path::Path::is_dir (line 2278)
[00:58:56] 
[00:58:56] test result: FAILED. 836 passed; 1 failed; 10 ignored; 0 measured; 0 filtered out
