diff
diff --git a/src/librustc_trans/debuginfo/namespace.rs b/src/librustc_trans/debuginfo/namespace.rs
index 47e2b8c461..8324077e29 100644
--- a/src/librustc_trans/debuginfo/namespace.rs
+++ b/src/librustc_trans/debuginfo/namespace.rs
@@ -58,7 +58,8 @@ pub fn item_namespace(ccx: &CrateContext, def_id: DefId) -> DIScope {
 
     let namespace_name = match def_key.disambiguated_data.data {
         DefPathData::CrateRoot => ccx.tcx().crate_name(def_id.krate).as_str(),
-        data => data.as_interned_str()
+        DefPathData::Module(module) => module,
+        _ => return parent_scope,
     };
 
     let namespace_name = CString::new(namespace_name.as_bytes()).unwrap();
