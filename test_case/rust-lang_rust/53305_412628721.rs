
[00:51:17] thread '[ui (nll)] ui/borrowck/mut-borrow-in-loop.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:51:17] 
[00:51:17] ---- [ui (nll)] ui/codemap_tests/one_line.rs stdout ----
[00:51:17] diff of stderr:
[00:51:17] 
[00:51:17] 7	   |     |      second mutable borrow occurs here
[00:51:17] 8	   |     first mutable borrow occurs here
[00:51:17] 9	   |     borrow later used here
[00:51:17] +	   |
[00:51:17] +	note: consider extracting this into a `let`-binding
[00:51:17] +	  --> $DIR/one_line.rs:13:12
[00:51:17] +	   |
[00:51:17] +	LL |     v.push(v.pop().unwrap()); //~ ERROR cannot borrow
[00:51:17] +	   |            ^^^^^^^
[00:51:17] 10	
[00:51:17] 11	error: aborting due to previous error
