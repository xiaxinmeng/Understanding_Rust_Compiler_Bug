
[00:51:26] ---- [ui] ui/range/issue-54505-no-std.rs stdout ----
[00:51:26] diff of stderr:
[00:51:26] 1	error: `#[panic_handler]` function required, but not found
[00:51:26] 2	
[00:51:26] -	error: language item required, but not found: `eh_personality`
[00:51:26] -	
[00:51:26] 5	error[E0308]: mismatched types
[00:51:26] 6	  --> $DIR/issue-54505-no-std.rs:21:16
[00:51:26] 7	   |
[00:51:26] 
[00:51:26] 74	   = note: expected type `&_`
[00:51:26] 75	              found type `core::ops::RangeToInclusive<{integer}>`
[00:51:26] 76	
[00:51:26] -	error: aborting due to 8 previous errors
[00:51:26] +	error: aborting due to 7 previous errors
[00:51:26] 78	
[00:51:26] 79	For more information about this error, try `rustc --explain E0308`.
