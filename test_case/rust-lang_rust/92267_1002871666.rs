
diff --git a/compiler/rustc_builtin_macros/src/format_foreign.rs b/compiler/rustc_builtin_macros/src/format_foreign.rs
index bfddd7073ff..3b9e9f76f45 100644
--- a/compiler/rustc_builtin_macros/src/format_foreign.rs
+++ b/compiler/rustc_builtin_macros/src/format_foreign.rs
@@ -289,8 +289,8 @@ fn translate(&self, s: &mut String) -> std::fmt::Result {
     }
 
     /// Returns an iterator over all substitutions in a given string.
-    pub fn iter_subs(s: &str, start_pos: usize) -> Substitutions<'_> {
-        Substitutions { s, pos: start_pos }
+    pub fn iter_subs(s: &str, _start_pos: usize) -> Substitutions<'_> {
+        Substitutions { s, pos: 0 }
     }
 
     /// Iterator over substitutions in a string.
@@ -303,15 +303,16 @@ impl<'a> Iterator for Substitutions<'a> {
         type Item = Substitution<'a>;
         fn next(&mut self) -> Option<Self::Item> {
             let (mut sub, tail) = parse_next_substitution(self.s)?;
+            let pos_diff = self.s.len() - tail.len();
             self.s = tail;
             match sub {
                 Substitution::Format(_) => {
                     if let Some(inner_span) = sub.position() {
                         sub.set_position(inner_span.start + self.pos, inner_span.end + self.pos);
-                        self.pos += inner_span.end;
+                        self.pos += pos_diff;
                     }
                 }
-                Substitution::Escape => self.pos += 2,
+                Substitution::Escape => self.pos += pos_diff,
             }
             Some(sub)
         }
