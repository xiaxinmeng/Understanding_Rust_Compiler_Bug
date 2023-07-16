
[00:47:59] ---- [ui] ui/issue-51116.rs stdout ----
[00:47:59] diff of stderr:
[00:47:59] 
[00:47:59] 5	   |                     --- the element type for this iterator is not specified
[00:47:59] 6	LL |             //~^ NOTE the element type for this iterator is not specified
[00:47:59] 7	LL |             *tile = 0;
[00:47:59] -	   |             ^^^^^ cannot infer type for `_`
[00:47:59] +	   |             ^^^^^ cannot infer type
[00:47:59] 9	   |
[00:47:59] 10	   = note: type must be known at this point
[00:47:59] 11	
