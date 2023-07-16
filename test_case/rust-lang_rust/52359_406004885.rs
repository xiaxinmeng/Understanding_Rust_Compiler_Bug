
---- [ui (nll)] ui\issue-30355.rs stdout ----
diff of stderr:
-	error[E0508]: cannot move out of type `[u8]`, a non-copy slice
-	  --> $DIR/issue-30355.rs:15:8
-	   |
-	LL |     &X(*Y)
-	   |        ^^ cannot move out of here
-	
7	error[E0161]: cannot move a value of type X: the size of X cannot be statically determined
8	  --> $DIR/issue-30355.rs:15:6
9	   |
15	   |
16	LL |     &X(*Y)
17	   |        ^^
+	
+	error[E0508]: cannot move out of type `[u8]`, a non-copy slice
+	  --> $DIR/issue-30355.rs:15:8
+	   |
+	LL |     &X(*Y)
+	   |        ^^ cannot move out of here
18	
19	error: aborting due to 3 previous errors
20	
