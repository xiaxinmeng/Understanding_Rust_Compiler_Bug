diff
diff --git a/src/librustdoc/passes/mod.rs b/src/librustdoc/passes/mod.rs
index 2591650e3f9..ec1e2ae36e6 100644
--- a/src/librustdoc/passes/mod.rs
+++ b/src/librustdoc/passes/mod.rs
@@ -179,7 +179,9 @@ crate fn source_span_for_markdown_range(
         return None;
     }
 
+    trace!("markdown is {}", markdown);
     let snippet = cx.sess().source_map().span_to_snippet(span_of_attrs(attrs)?).ok()?;
+    trace!("snippet is {}", snippet);
