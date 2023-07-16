
---- [ui] ui/check-doc-alias-attr.rs stdout ----
diff of stderr:

36	LL | | ")]
37	   | |_^
38	
-	error: ' ' character isn't allowed in `#[doc(alias = "...")]`
-	  --> $DIR/check-doc-alias-attr.rs:14:7
-	   |
-	LL | #[doc(alias = " ")]
-	   |       ^^^^^^^^^^^
-	
45	error: '\t' character isn't allowed in `#[doc(alias = "...")]`
46	  --> $DIR/check-doc-alias-attr.rs:15:7
47	   |

48	LL | #[doc(alias = "\t")]
49	   |       ^^^^^^^^^^^^
50	
-	error: aborting due to 8 previous errors
+	error: aborting due to 7 previous errors
