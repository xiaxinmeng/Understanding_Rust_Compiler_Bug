
[01:13:49] failures:
[01:13:49]
[01:13:49] ---- convert.rs - convert::Into (line 178) stdout ----
[01:13:49]  thread 'rustc' panicked at 'test compiled while it wasn't supposed to', /checkout/src/librustdoc/test.rs:263
[01:13:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:13:49]
[01:13:49] ---- convert.rs - convert::Into (line 194) stdout ----
[01:13:49]  error[E0425]: cannot find value `w` in this scope
[01:13:49]  --> <anon>:7:9
[01:13:49]   |
[01:13:49] 7 |         w.0
[01:13:49]   |         ^ not found in this scope
[01:13:49]
[01:13:49] error[E0053]: method `into` has an incompatible type for trait
[01:13:49]  --> <anon>:6:5
[01:13:49]   |
[01:13:49] 6 | /     fn into(self) -> Wrapper {
[01:13:49] 7 | |         w.0
[01:13:49] 8 | |     }
[01:13:49]   | |_____^ expected struct `std::string::String`, found struct `main::Wrapper`
[01:13:49]   |
[01:13:49]   = note: expected type `fn(main::Wrapper) -> std::string::String`
[01:13:49]              found type `fn(main::Wrapper) -> main::Wrapper`
[01:13:49]
[01:13:49] error: aborting due to previous error(s)
[01:13:49]
[01:13:49] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:214
[01:13:49] failures:
[01:13:49]     convert.rs - convert::Into (line 178)
[01:13:49]     convert.rs - convert::Into (line 194)
[01:13:49]
[01:13:49] test result: FAILED. 1200 passed; 2 failed; 12 ignored; 0 measured; 0 filtered out
