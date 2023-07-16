
---- [ui (nll)] ui\unsized-locals\unsized-exprs2.rs stdout ----
diff of stderr:
4	LL |     udrop::<[u8]>(foo()[..]);
5	   |                   ^^^^^^^^^ cannot move out of here
6	
-	error[E0507]: cannot move out of data in a `&` reference
-	  --> $DIR/unsized-exprs2.rs:22:19
-	   |
-	LL |     udrop::<[u8]>(foo()[..]);
-	   |                   ^^^^^^^^^
-	   |                   |
-	   |                   cannot move out of data in a `&` reference
-	   |                   cannot move
+	error: aborting due to previous error
15	
-	error: aborting due to 2 previous errors
-	
-	Some errors occurred: E0507, E0508.
-	For more information about an error, try `rustc --explain E0507`.
+	For more information about this error, try `rustc --explain E0508`.
20	
