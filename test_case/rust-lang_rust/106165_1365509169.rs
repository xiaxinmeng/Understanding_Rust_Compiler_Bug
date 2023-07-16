diff
diff --git a/src/bootstrap/lib.rs b/src/bootstrap/lib.rs
index f84fcd21cfc..5aff33a2fb8 100644
--- a/src/bootstrap/lib.rs
+++ b/src/bootstrap/lib.rs
@@ -1400,7 +1396,7 @@ fn in_tree_crates(&self, root: &str, target: Option<TargetSelection>) -> Vec<&Cr
         let mut list = vec![INTERNER.intern_str(root)];
         let mut visited = HashSet::new();
         while let Some(krate) = list.pop() {
-            let krate = &self.crates[&krate];
+            let krate = self.crates.get(&krate).unwrap_or_else(|| panic!("metadata missing for {krate}: {:?}", self.crates));
             ret.push(krate);
             for dep in &krate.deps {
                 if !self.crates.contains_key(dep) {
