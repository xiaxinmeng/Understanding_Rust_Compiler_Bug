
[00:55:05] ---- [ui] ui/nll/trait-associated-constant.rs stdout ----
[00:55:05] 	diff of stderr:
[00:55:05] 
[00:55:05] 9	note: the lifetime 'c as defined on the impl at 30:1...
[00:55:05] 10	  --> $DIR/trait-associated-constant.rs:30:1
[00:55:05] 11	   |
[00:55:05] -	30 | / impl<'a: 'b, 'b, 'c> Anything<'a, 'b> for FailStruct1 {
[00:55:05] -	31 | |     const AC: Option<&'c str> = None;
[00:55:05] -	32 | |     //~^ ERROR: mismatched types
[00:55:05] -	33 | | }
[00:55:05] -	   | |_^
[00:55:05] +	30 | impl<'a: 'b, 'b, 'c> Anything<'a, 'b> for FailStruct1 {
[00:55:05] +	   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:55:05] 17	note: ...does not necessarily outlive the lifetime 'b as defined on the impl at 30:1
[00:55:05] 18	  --> $DIR/trait-associated-constant.rs:30:1
[00:55:05] 19	   |
[00:55:05] 
[00:55:05] -	30 | / impl<'a: 'b, 'b, 'c> Anything<'a, 'b> for FailStruct1 {
[00:55:05] -	31 | |     const AC: Option<&'c str> = None;
[00:55:05] -	32 | |     //~^ ERROR: mismatched types
[00:55:05] -	33 | | }
[00:55:05] -	   | |_^
[00:55:05] +	30 | impl<'a: 'b, 'b, 'c> Anything<'a, 'b> for FailStruct1 {
[00:55:05] +	   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:55:05] 25	
[00:55:05] 26	error[E0308]: mismatched types
[00:55:05] 27	  --> $DIR/trait-associated-constant.rs:38:5
[00:55:05] 
[00:55:05] 34	note: the lifetime 'a as defined on the impl at 37:1...
[00:55:05] 35	  --> $DIR/trait-associated-constant.rs:37:1
[00:55:05] 36	   |
[00:55:05] -	37 | / impl<'a: 'b, 'b> Anything<'a, 'b> for FailStruct2 {
[00:55:05] -	38 | |     const AC: Option<&'a str> = None;
[00:55:05] -	39 | |     //~^ ERROR: mismatched types
[00:55:05] -	40 | | }
[00:55:05] -	   | |_^
[00:55:05] +	37 | impl<'a: 'b, 'b> Anything<'a, 'b> for FailStruct2 {
[00:55:05] +	   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:55:05] 42	note: ...does not necessarily outlive the lifetime 'b as defined on the impl at 37:1
[00:55:05] 43	  --> $DIR/trait-associated-constant.rs:37:1
[00:55:05] 44	   |
[00:55:05] 
[00:55:05] -	37 | / impl<'a: 'b, 'b> Anything<'a, 'b> for FailStruct2 {
[00:55:05] -	38 | |     const AC: Option<&'a str> = None;
[00:55:05] -	39 | |     //~^ ERROR: mismatched types
[00:55:05] -	40 | | }
[00:55:05] -	   | |_^
[00:55:05] +	37 | impl<'a: 'b, 'b> Anything<'a, 'b> for FailStruct2 {
[00:55:05] +	   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:55:05] 50	
[00:55:05] 51	error: aborting due to 2 previous errors
[00:55:05] 52	
