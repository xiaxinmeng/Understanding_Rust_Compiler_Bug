
[01:26:48] ---- [ui] rustdoc-ui/treat-err-as-bug.rs stdout ----
[01:26:48] diff of stderr:
[01:26:48] 
[01:26:48] 15	
[01:26:48] 16	note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:26:48] 17	
[01:26:48] -	note: rustc 1.30.0-dev running on x86_64-apple-darwin
[01:26:48] +	note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[01:26:48] 19	
[01:26:48] 20	note: compiler flags: -Z ui-testing -Z unstable-options -Z treat-err-as-bug
[01:26:48] 21	
[01:26:48] 
[01:26:48] 
[01:26:48] The actual stderr differed from the expected stderr.
[01:26:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/treat-err-as-bug/treat-err-as-bug.stderr
[01:26:48] To update references, rerun the tests and pass the `--bless` flag
[01:26:48] To only update this specific test, also pass `--test-args treat-err-as-bug.rs`
[01:26:48] 
[01:26:48] error: 1 errors occurred comparing output.
[01:26:48] status: exit code: 1
