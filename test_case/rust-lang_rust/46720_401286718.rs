
---- [ui] ui\suggestions\issue-46302.rs stdout ----
diff of stderr:
2	  --> $DIR/issue-46302.rs:13:27
3	   |
4	LL |   let u: &str = if true { s[..2] } else { s };
-	   |                           ^^^^^^ expected &str, found str
+	   |                           ^^^^^^
+	   |                           |
+	   |                           expected &str, found str
+	   |                           help: consider borrowing here: `&s[..2]`
6	   |
7	   = note: expected type `&str`
8	              found type `str`
