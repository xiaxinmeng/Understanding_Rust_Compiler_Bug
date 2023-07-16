
[00:58:58] failures:
[00:58:58] 
[00:58:58] ---- macros.rs - eprint (line 196) stdout ----
[00:58:58] 	error[E0423]: expected function, found macro `eprint`
[00:58:58]  --> macros.rs:4:1
[00:58:58]   |
[00:58:58] 4 | eprint("Error: Could not complete task");
[00:58:58]   | ^^^^^^ did you mean `eprint!(...)`?
[00:58:58] 
[00:58:58] thread 'rustc' panicked at 'couldn't compile the test', /checkout/src/librustdoc/test.rs:280:12
[00:58:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:58] 
[00:58:58] ---- macros.rs - eprintln (line 224) stdout ----
[00:58:58] 	error[E0423]: expected function, found macro `eprint`
[00:58:58]  --> macros.rs:4:1
[00:58:58]   |
[00:58:58] 4 | eprint("Error: Could not complete task");
[00:58:58]   | ^^^^^^ did you mean `eprint!(...)`?
[00:58:58] 
[00:58:58] thread 'rustc' panicked at 'couldn't compile the test', /checkout/src/librustdoc/test.rs:280:12
[00:58:58] 
[00:58:58] 
[00:58:58] failures:
[00:58:58]     macros.rs - eprint (line 196)
[00:58:58]     macros.rs - eprintln (line 224)
[00:58:58] 
[00:58:58] test result: FAILED. 842 passed; 2 failed; 10 ignored; 0 measured; 0 filtered out
