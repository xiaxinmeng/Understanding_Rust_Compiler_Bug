
---- [ui] ui\issues\issue-39175.rs stdout ----
diff of stderr:
3	   |
4	LL |     Command::new("echo").arg("hello").exec();
5	   |                                       ^^^^
-	   |
-	   = help: items from traits can only be used if the trait is in scope
-	help: the following trait is implemented but not in scope, perhaps add a `use` for it:
-	   |
-	LL | use std::os::unix::process::CommandExt;
-	   |
12	
13	error: aborting due to previous error
14	
