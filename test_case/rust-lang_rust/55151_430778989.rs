
[00:54:08] ---- [ui] ui/nll/polonius-smoke-test.rs stdout ----
[00:54:08] diff of stderr:
[00:54:08] 
[00:54:08] 28	LL |     y
[00:54:08] 29	   |     - borrow later used here
[00:54:08] 30	
[00:54:08] -	error[E0505]: cannot move out of `s` because it is borrowed
[00:54:08] -	  --> $DIR/polonius-smoke-test.rs:43:5
[00:54:08] -	   |
[00:54:08] -	LL |     let r = &mut *s;
[00:54:08] -	   |             ------- borrow of `*s` occurs here
[00:54:08] -	LL |     let tmp = foo(&r);
[00:54:08] -	LL |     s; //~ ERROR
[00:54:08] -	   |     ^ move out of `s` occurs here
[00:54:08] -	LL |     tmp;
[00:54:08] -	   |     --- borrow later used here
[00:54:08] -	
[00:54:08] -	error: aborting due to 4 previous errors
[00:54:08] +	error: aborting due to 3 previous errors
[00:54:08] 43	
[00:54:08] 44	Some errors occurred: E0503, E0505, E0597.
[00:54:08] 45	For more information about an error, try `rustc --explain E0503`.
