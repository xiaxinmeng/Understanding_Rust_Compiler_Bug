
modified   src/client/layout.rs
@@ -176,6 +176,8 @@ pub struct Layout {
     pub split_size: Option<SplitSize>,
     #[serde(skip_serializing_if = "Option::is_none")]
     pub plugin: Option<PathBuf>,
+    #[serde(skip_serializing_if = "Option::is_none")]
+    pub run: Option<String>,
 }
