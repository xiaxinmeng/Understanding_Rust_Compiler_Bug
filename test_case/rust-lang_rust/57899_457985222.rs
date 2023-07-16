

failures:
---- [ui] ui\lint\issue-47390-unused-variable-in-struct-pattern.rs stdout ----
diff of stderr:
29	LL |     let (mut var, unused_var) = (1, 2);
30	   |                   ^^^^^^^^^^ help: consider prefixing with an underscore: `_unused_var`
31	
-	warning: unused variable: `corridors_of_light`
-	  --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:37:26
-	   |
-	LL |     if let SoulHistory { corridors_of_light,
-	   |                          ^^^^^^^^^^^^^^^^^^ help: try ignoring the field: `corridors_of_light: _`
-	
38	warning: variable `hours_are_suns` is assigned to, but never used
39	  --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:38:30
40	   |
42	   |                              ^^^^^^^^^^^^^^
43	   |
44	   = note: consider using `_hours_are_suns` instead
+	
+	warning: unused variable: `corridors_of_light`
+	  --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:37:26
+	   |
+	LL |     if let SoulHistory { corridors_of_light,
+	   |                          ^^^^^^^^^^^^^^^^^^ help: try ignoring the field: `corridors_of_light: _`
45	
46	warning: value assigned to `hours_are_suns` is never read
47	  --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:40:9
The actual stderr differed from the expected stderr.
