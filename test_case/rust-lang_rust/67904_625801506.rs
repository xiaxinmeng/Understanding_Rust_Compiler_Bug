
git diff
diff --git a/src/element/mod.rs b/src/element/mod.rs
index 9485b13..24da473 100644
--- a/src/element/mod.rs
+++ b/src/element/mod.rs
@@ -22,7 +22,7 @@ impl El {
    
    /// Creates a new element that displays its name as text and nothing else
    /// when formatted. This is useful for making innerHTML something other
-   /// than a tag, such as inside the <style> or <script> tags.
+   /// than a tag, such as inside the `<style>` or `<script>` tags.
    pub fn text<N: Into<String>>(text: N) -> Self {
       El {
          is_text: true,
