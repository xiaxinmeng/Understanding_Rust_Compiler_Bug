
[00:44:34] ---- [ui] ui/similar-tokens.rs stdout ----
[00:44:34] diff of stderr:
[00:44:34] 
[00:44:34] -	error: expected one of `,`, `::`, or `as`, found `.`
[00:44:34] +	error: expected one of `,`, `::`, `as`, or `}`, found `.`
[00:44:34] 2	  --> $DIR/similar-tokens.rs:17:10
[00:44:34] 3	   |
[00:44:34] 4	LL | use x::{A. B}; //~ ERROR expected one of `,`, `::`, or `as`, found `.`
[00:44:34] 
[00:44:34] -	   |          ^ expected one of `,`, `::`, or `as` here
[00:44:34] +	   |          ^ expected one of `,`, `::`, `as`, or `}` here
[00:44:34] 6	
[00:44:34] 7	error: aborting due to previous error
[00:44:34] 8	
