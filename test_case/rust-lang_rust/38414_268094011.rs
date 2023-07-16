diff
diff --git a/src/librustdoc/clean/mod.rs b/src/librustdoc/clean/mod.rs
index 28ca92f..a99d4dd 100644
--- a/src/librustdoc/clean/mod.rs
+++ b/src/librustdoc/clean/mod.rs
@@ -2205,8 +2205,8 @@ impl Path {
         }
     }
 
-    pub fn last_name(&self) -> String {
-        self.segments.last().unwrap().name.clone()
+    pub fn last_name(&self) -> &str {
+        &self.segments.last().unwrap().name
     }
 }
 
diff --git a/src/librustdoc/html/render.rs b/src/librustdoc/html/render.rs
index 846c072..5839c46 100644
--- a/src/librustdoc/html/render.rs
+++ b/src/librustdoc/html/render.rs
@@ -2110,23 +2110,22 @@ fn item_trait(w: &mut fmt::Formatter, cx: &Context, it: &clean::Item,
         <h2 id='implementors'>Implementors</h2>
         <ul class='item-list' id='implementors-list'>
     ")?;
-    let mut implementor_count: FxHashMap<String, usize> = FxHashMap();
-    for (_, implementors) in cache.implementors.iter() {
+    if let Some(implementors) = cache.implementors.get(&it.def_id) {
+        let mut implementor_count: FxHashMap<&str, usize> = FxHashMap();
         for implementor in implementors {
             if let clean::Type::ResolvedPath {ref path, ..} = implementor.impl_.for_ {
                 *implementor_count.entry(path.last_name()).or_insert(0) += 1;
             }
         }
-    }
-    if let Some(implementors) = cache.implementors.get(&it.def_id) {
-        for implementor in implementors.iter() {
+
+        for implementor in implementors {
             write!(w, "<li><code>")?;
             // If there's already another implementor that has the same abbridged name, use the
             // full path, for example in `std::iter::ExactSizeIterator`
             let dissambiguate = if let clean::Type::ResolvedPath {
                 ref path, ..
             } = implementor.impl_.for_ {
-                *implementor_count.get(&path.last_name()).unwrap_or(&0) > 1
+                implementor_count[path.last_name()] > 1
             } else {
                 false
             };

