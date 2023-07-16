diff
@@ -150,6 +150,10 @@ pub mod util;
 #[cfg(windows)]
 mod job;

+/// During development of Yorick, we override dependencies to local paths, and this confuses the
+/// bootstrapper into believing that our code is in-tree. It's not!
+const NOT_IN_TREE: [&str; 1] = ["ykpack"];
 #[cfg(all(unix, not(target_os = "haiku")))]
 mod job {
     pub unsafe fn setup(build: &mut crate::Build) {
@@ -1105,6 +1109,9 @@ impl Build {
         let mut list = vec![INTERNER.intern_str(root)];
         let mut visited = HashSet::new();
         while let Some(krate) = list.pop() {
+            if NOT_IN_TREE.contains(&&*krate) {
+                continue;
+            }
             let krate = &self.crates[&krate];
             ret.push(krate);
             for dep in &krate.deps {
