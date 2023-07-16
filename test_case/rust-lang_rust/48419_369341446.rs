
[01:00:19] failures:
[01:00:19] 
[01:00:19] ---- [ui] ui/issue-48276.rs stdout ----
[01:00:19] 	diff of stderr:
[01:00:19] 
[01:00:19] 1	error[E0185]: method `from` has a `&self` declaration in the impl, but not in the trait
[01:00:19] 2	  --> $DIR/issue-48276.rs:21:5
[01:00:19] 3	   |
[01:00:19] -	15 |     fn from(a: A) -> Self;
[01:00:19] +	LL |     fn from(a: A) -> Self;
[01:00:19] 5	   |     ---------------------- trait method declared without `&self`
[01:00:19] 6	...
[01:00:19] -	21 |     fn from(self: &'a Self) -> &'b str {
[01:00:19] +	LL |     fn from(self: &'a Self) -> &'b str {
[01:00:19] 8	   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `&self` used in impl
[01:00:19] 9	
[01:00:19] 10	error[E0185]: method `from` has a `&self` declaration in the impl, but not in the trait
[01:00:19] 
[01:00:19] 11	  --> $DIR/issue-48276.rs:30:5
[01:00:19] 12	   |
[01:00:19] -	30 |     fn from(&self) -> B {
[01:00:19] +	LL |     fn from(&self) -> B {
[01:00:19] 14	   |     ^^^^^^^^^^^^^^^^^^^ `&self` used in impl
[01:00:19] 15	   |
[01:00:19] 16	   = note: `from` from trait: `fn(T) -> Self`
[01:00:19] 
[01:00:19] 18	error[E0185]: method `from` has a `&self` declaration in the impl, but not in the trait
[01:00:19] 19	  --> $DIR/issue-48276.rs:37:5
[01:00:19] 20	   |
[01:00:19] -	37 |     fn from(&self) -> &'static str {
[01:00:19] +	LL |     fn from(&self) -> &'static str {
[01:00:19] 22	   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `&self` used in impl
[01:00:19] 23	   |
[01:00:19] 24	   = note: `from` from trait: `fn(T) -> Self`
[01:00:19] 
[01:00:19] 25	
[01:00:19] 26	error: aborting due to 3 previous errors
[01:00:19] 27	
[01:00:19] +	If you want more information on this error, try using "rustc --explain E0185"
[01:00:19] 28	
[01:00:19] 
[01:00:19] 
[01:00:19] The actual stderr differed from the expected stderr.
[01:00:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-48276.stderr
[01:00:19] To update references, run this command from build directory:
[01:00:19] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'issue-48276.rs'
[01:00:19] 
[01:00:19] error: 1 errors occurred comparing output.
[01:00:19] status: exit code: 101
[01:00:19] stdout:
[01:00:19] ------------------------------------------
[01:00:19] 
[01:00:19] ------------------------------------------
[01:00:19] stderr:
[01:00:19] ------------------------------------------
[01:00:19] error[E0185]: method `from` has a `&self` declaration in the impl, but not in the trait
[01:00:19]   --> /checkout/src/test/ui/issue-48276.rs:21:5
[01:00:19]    |
[01:00:19] LL |     fn from(a: A) -> Self;
[01:00:19]    |     ---------------------- trait method declared without `&self`
[01:00:19] ...
[01:00:19] LL |     fn from(self: &'a Self) -> &'b str {
[01:00:19]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `&self` used in impl
[01:00:19] 
[01:00:19] error[E0185]: method `from` has a `&self` declaration in the impl, but not in the trait
[01:00:19]   --> /checkout/src/test/ui/issue-48276.rs:30:5
[01:00:19]    |
[01:00:19] LL |     fn from(&self) -> B {
[01:00:19]    |     ^^^^^^^^^^^^^^^^^^^ `&self` used in impl
[01:00:19]    |
[01:00:19]    = note: `from` from trait: `fn(T) -> Self`
[01:00:19] 
[01:00:19] error[E0185]: method `from` has a `&self` declaration in the impl, but not in the trait
[01:00:19]   --> /checkout/src/test/ui/issue-48276.rs:37:5
[01:00:19]    |
[01:00:19] LL |     fn from(&self) -> &'static str {
[01:00:19]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `&self` used in impl
[01:00:19]    |
[01:00:19]    = note: `from` from trait: `fn(T) -> Self`
[01:00:19] 
[01:00:19] error: aborting due to 3 previous errors
[01:00:19] 
[01:00:19] If you want more information on this error, try using "rustc --explain E0185"
[01:00:19] 
[01:00:19] ------------------------------------------
[01:00:19] 
[01:00:19] thread '[ui] ui/issue-48276.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2893:9
[01:00:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:19] 
[01:00:19] 
[01:00:19] failures:
[01:00:19]     [ui] ui/issue-48276.rs
[01:00:19] 
[01:00:19] test result: FAILED. 1243 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out
[01:00:19] 
[01:00:19] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:476:22
[01:00:19] 
