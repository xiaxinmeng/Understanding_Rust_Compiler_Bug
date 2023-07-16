
--- gcc/rust/parse/rust-parse-impl.h	(original)
+++ gcc/rust/parse/rust-parse-impl.h	(reformatted)
@@ -10693,7 +10693,7 @@
     {
       lexer.skip_token ();
       alts.push_back (parse_pattern_no_alt ());
-    } 
+    }
   while (lexer.peek_token ()->get_id () == PIPE);
 
   /* alternates */
   