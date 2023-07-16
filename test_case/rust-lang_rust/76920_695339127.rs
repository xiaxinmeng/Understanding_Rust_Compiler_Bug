
failures:

---- [ui] ui/proc-macro/invalid-punct-ident-2.rs stdout ----
diff of stderr:

-	query stack during panic:
-	we're just showing a limited slice of the query stack
-	end of query stack
4	error: proc macro panicked
5	  --> $DIR/invalid-punct-ident-2.rs:16:1
6	   |


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-2/invalid-punct-ident-2.stderr
 ---- [ui] ui/proc-macro/invalid-punct-ident-3.rs stdout ----
diff of stderr:

-	query stack during panic:
-	we're just showing a limited slice of the query stack
-	end of query stack
4	error: proc macro panicked
5	  --> $DIR/invalid-punct-ident-3.rs:16:1
6	   |


The actual stderr differed from the expected stderr.
 ---- [ui] ui/proc-macro/load-panic-backtrace.rs stdout ----
diff of stderr:

1	at 'panic-derive', $DIR/auxiliary/test-macros.rs:43:5
+	query stack during panic:
+	we're just showing a limited slice of the query stack
+	end of query stack
2	error: proc-macro derive panicked
3	  --> $DIR/load-panic-backtrace.rs:17:10
4	   |
