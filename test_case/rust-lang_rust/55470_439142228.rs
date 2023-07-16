
01:17:12] failures:
[01:17:12] 
[01:17:12] ---- boxed.rs - boxed::Box<[T]>::from (line 479) stdout ----
[01:17:12] error: expected expression, found `:`
[01:17:12]  --> boxed.rs:484:11
[01:17:12]   |
[01:17:12] 8 | println!({:?}, boxed_slice);
[01:17:12]   |           ^ expected expression
[01:17:12] 
[01:17:12] error: format argument must be a string literal
[01:17:12]  --> boxed.rs:484:10
[01:17:12]   |
[01:17:12] 8 | println!({:?}, boxed_slice);
[01:17:12]   |          ^^^^
[01:17:12] help: you might be missing a string literal to format with
[01:17:12]   |
[01:17:12] 8 | println!("{} {}", {:?}, boxed_slice);
[01:17:12]   |          ^^^^^^^^
[01:17:12] 
[01:17:12] thread 'boxed.rs - boxed::Box<[T]>::from (line 479)' panicked at 'couldn't compile the test', librustdoc/test.rs:332:13
[01:17:12] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:17:12] 
[01:17:12] 
[01:17:12] failures:
[01:17:12]     boxed.rs - boxed::Box<[T]>::from (line 479)
[01:17:12] 
