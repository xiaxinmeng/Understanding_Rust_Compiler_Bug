
---- [ui] rustdoc-ui/intra-links-private.rs#private stdout ----
diff of stderr:

-	warning: public documentation for `DocMe` links to private item `DontDocMe`
-	  --> $DIR/intra-links-private.rs:5:11
-	   |
-	LL | /// docs [DontDocMe]
-	   |           ^^^^^^^^^ this item is private
-	   |
-	   = note: `#[warn(private_intra_doc_links)]` on by default
-	   = note: this link resolves only because you passed `--document-private-items`, but will break without
-	
-	warning: 1 warning emitted
-	
-	
