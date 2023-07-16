
[01:14:05] failures:
[01:14:05] 
[01:14:05] ---- [ui] ui\error-codes\E0583.rs stdout ----
[01:14:05] 	diff of stderr:
[01:14:05] 
[01:14:05] 4	11 | mod module_that_doesnt_exist; //~ ERROR E0583
[01:14:05] 5	   |     ^^^^^^^^^^^^^^^^^^^^^^^^
[01:14:05] 6	   |
[01:14:05] -	   = help: name the file either module_that_doesnt_exist.rs or module_that_doesnt_exist/mod.rs inside the directory "$DIR"
[01:14:05] +	   = help: name the file either module_that_doesnt_exist.rs or module_that_doesnt_exist/mod.rs inside the directory "C:/projects/rust/src/test/ui/error-codes"
[01:14:05] 8	
[01:14:05] 9	error: aborting due to previous error
[01:14:05] 10	
