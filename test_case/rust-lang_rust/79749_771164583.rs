
+    fn push_open_tag(s: &mut String, open_tag: &str, close_tag: &str) {
+        s.push_str(open_tag);
+        unclosed_tags.push(close_tag);
+    }
