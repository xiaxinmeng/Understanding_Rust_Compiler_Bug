diff
diff --git src/main.rs src/main.rs
index 6a6d2a3..69be593 100644
--- src/main.rs
+++ src/main.rs
@@ -8,7 +8,7 @@ fn main() {
 }
 
 fn gen_adj(alpha:&String) -> String {
-    let list_of_adj = [
+    let list_of_adj: &[&str] = &[
         "a-ok",
         "a-okay",
         "a-one",
@@ -21268,7 +21268,7 @@ fn gen_adj(alpha:&String) -> String {
 }
 
 fn gen_noun(alpha:&String) -> String {
-    let list_of_noun= [
+    let list_of_noun: &[&str] = &[
         "a'man",
         "a-bomb",
         "a-horizon",
