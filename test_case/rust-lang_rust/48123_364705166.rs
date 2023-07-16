
[01:00:25] failures:
[01:00:25] 
[01:00:25] ---- [ui] ui/issue-47706-trait.rs stdout ----
[01:00:25] 	diff of stderr:
[01:00:25] 
[01:00:25] 1	error[E0601]: main function not found
[01:00:25] 2	
[01:00:25] -	error[E0593]: function is expected to take a single 0-tuple as argument, but it takes 2 distinct arguments
[01:00:25] -	  --> $DIR/issue-47706-trait.rs:13:20
[01:00:25] -	   |
[01:00:25] -	12 |     fn f(&self, _: ()) {
[01:00:25] -	   |     ------------------ takes 2 distinct arguments
[01:00:25] -	13 |         None::<()>.map(Self::f);
[01:00:25] -	   |                    ^^^ expected function that takes a single 0-tuple as argument
[01:00:25] -	
[01:00:25] -	error: aborting due to 2 previous errors
[01:00:25] -	
[01:00:25] 13	
[01:00:25] 
[01:00:25] 
[01:00:25] The actual stderr differed from the expected stderr.
[01:00:25] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-47706-trait.stderr
[01:00:25] To update references, run this command from build directory:
[01:00:25] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'issue-47706-trait.rs'
[01:00:25] 
[01:00:25] error: 1 errors occurred comparing output.
[01:00:25] status: exit code: 101
[01:00:25] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-47706-trait.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-47706-trait.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-47706-trait.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:00:25] stdout:
[01:00:25] ------------------------------------------
[01:00:25] 
[01:00:25] ------------------------------------------
[01:00:25] stderr:
[01:00:25] ------------------------------------------
[01:00:25] {"message":"main function not found","code":{"code":"E0601","explanation":"\nNo `main` function was found in a binary crate. To fix this error, add a\n`main` function. For example:\n\n