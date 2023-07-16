diff
7	   = help: items from traits can only be used if the trait is in scope
8	help: the following trait is implemented but not in scope; perhaps add a `use` for it:
9	   |
-	LL | use std::os::unix::process::CommandExt;
+	LL | use rustc_ast::std::os::unix::process::CommandExt;
