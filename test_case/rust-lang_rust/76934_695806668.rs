
 failures:

---- [ui] rustdoc-ui/malformed-generics.rs stdout ----
diff of stderr:

34	LL | //! [Vec<T>>>]
35	   |      ^^^^^^^^ unbalanced angle brackets
36	
-	error: aborting due to 5 previous errors
+	error: unresolved link to `<Vec`
+	  --> $DIR/malformed-generics.rs:8:6
+	   |
+	LL | //! [<Vec]
+	   |      ^^^^ unbalanced angle brackets
+	
+	error: unresolved link to `Vec::<`
+	  --> $DIR/malformed-generics.rs:9:6
+	   |
+	LL | //! [Vec::<]
+	   |      ^^^^^^ unbalanced angle brackets
+	
+	error: aborting due to 7 previous errors
