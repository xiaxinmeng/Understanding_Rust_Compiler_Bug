
[01:04:37] failures:
[01:04:37] 
[01:04:37] ---- convert.rs - convert::Into (line 191) stdout ----
[01:04:37] 	error[E0053]: method `into` has an incompatible type for trait
[01:04:37]  --> <anon>:6:5
[01:04:37]   |
[01:04:37] 6 | /     fn into(self) -> Wrapper<T> {
[01:04:37] 7 | |         self.0
[01:04:37] 8 | |     }
[01:04:37]   | |_____^ expected struct `std::vec::Vec`, found struct `main::Wrapper`
[01:04:37]   |
[01:04:37]   = note: expected type `fn(main::Wrapper<T>) -> std::vec::Vec<T>`
[01:04:37]              found type `fn(main::Wrapper<T>) -> main::Wrapper<T>`
[01:04:37] 
[01:04:37] error: aborting due to previous error(s)
[01:04:37] 
[01:04:37] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:214
[01:04:37] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:37] 
[01:04:37] 
[01:04:37] failures:
[01:04:37]     convert.rs - convert::Into (line 191)
[01:04:37] 
[01:04:37] test result: FAILED. 1203 passed; 1 failed; 13 ignored; 0 measured; 0 filtered out
