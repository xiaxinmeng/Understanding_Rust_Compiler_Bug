
[00:42:54] thread '[ui] ui/issue-43925.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:42:54] 
[00:42:54] ---- [ui] ui/parser/expected-comma-found-token.rs stdout ----
[00:42:54] diff of stderr:
[00:42:54] 
[00:42:54] -	error: expected one of `)` or `,`, found `label`
[00:42:54] -	  --> $DIR/expected-comma-found-token.rs:19:5
[00:42:54] +	error[E0601]: `main` function not found in crate `expected_comma_found_token`
[00:42:54] 3	   |
[00:42:54] -	LL |     message="the message"
[00:42:54] -	   |                          - expected one of `)` or `,` here
[00:42:54] -	LL |     label="the label"
[00:42:54] -	   |     ^^^^^ unexpected token
[00:42:54] +	   = note: consider adding a `main` function to `$DIR/expected-comma-found-token.rs`
[00:42:54] 8	
[00:42:54] -	error: aborting due to previous error
[00:42:54] +	error[E0232]: `#[rustc_on_unimplemented]` requires a value
[00:42:54] +	  --> $DIR/expected-comma-found-token.rs:17:1
[00:42:54] +	   |
[00:42:54] +	LL | / #[rustc_on_unimplemented(
[00:42:54] +	LL | |     message="the message"
[00:42:54] +	LL | |     label="the label"
[00:42:54] +	LL | | )]
[00:42:54] +	   | |__^ value required here
[00:42:54] +	   |
[00:42:54] +	   = note: eg `#[rustc_on_unimplemented = "foo"]`
[00:42:54] 10	
[00:42:54] +	error: aborting due to 2 previous errors
[00:42:54] +	
[00:42:54] +	Some errors occurred: E0232, E0601.
[00:42:54] +	For more information about an error, try `rustc --explain E0232`.
[00:42:54] 11	
