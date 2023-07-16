
[01:26:02] failures:
[01:26:02] 
[01:26:02] ---- /checkout/src/doc/rust-by-example/src/macros/dsl.md - Domain_Specific_Languages__DSLs_ (line 33) stdout ----
[01:26:02] 	error[E0618]: expected function, found `{integer}`
[01:26:02]  --> /checkout/src/doc/rust-by-example/src/macros/dsl.md:34:9
[01:26:02]   |
[01:26:02] 3 |   1 + 2 = 3
[01:26:02]   |  _________^
[01:26:02] 4 | | (1 + 2) * (3 / 4) = 0
[01:26:02]   | |_______^ not a function
[01:26:02] 
[01:26:02] error[E0070]: invalid left-hand side expression
[01:26:02]  --> /checkout/src/doc/rust-by-example/src/macros/dsl.md:34:9
[01:26:02]   |
[01:26:02] 3 |   1 + 2 = 3
[01:26:02]   |  _________^
[01:26:02] 4 | | (1 + 2) * (3 / 4) = 0
[01:26:02]   | |_____________________^ left-hand of expression not valid
[01:26:02] 
[01:26:02] error[E0308]: mismatched types
[01:26:02]  --> /checkout/src/doc/rust-by-example/src/macros/dsl.md:34:9
[01:26:02]   |
[01:26:02] 3 |   1 + 2 = 3
[01:26:02]   |  _________^
[01:26:02] 4 | | (1 + 2) * (3 / 4) = 0
[01:26:02]   | |_____________________^ expected integral variable, found ()
[01:26:02]   |
[01:26:02]   = note: expected type `{integer}`
[01:26:02]              found type `()`
[01:26:02] 
[01:26:02] error[E0070]: invalid left-hand side expression
[01:26:02]  --> /checkout/src/doc/rust-by-example/src/macros/dsl.md:34:1
[01:26:02]   |
[01:26:02] 3 | / 1 + 2 = 3
[01:26:02] 4 | | (1 + 2) * (3 / 4) = 0
[01:26:02]   | |_____________________^ left-hand of expression not valid
[01:26:02] 
[01:26:02] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:295:13
[01:26:02] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:26:02] 
[01:26:02] 
[01:26:02] failures:
[01:26:02]     /checkout/src/doc/rust-by-example/src/macros/dsl.md - Domain_Specific_Languages__DSLs_ (line 33)
