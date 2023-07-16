diff
@@ -336,13 +336,14 @@ pub struct Enum {
 #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
 pub struct Variant {
     /// Whether the variant is plain, a tuple-like, or struct-like. Contains the fields.
-    pub kind: VariantKind,
+    #[serde(flatten)]
+    pub variant_kind: VariantKind,
     /// The discriminant, if explicitly specified.
     pub discriminant: Option<Discriminant>,
 }

 #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
-#[serde(rename_all = "snake_case")]
+#[serde(tag = "variant_kind", content = "variant_inner", rename_all = "snake_case")]
 pub enum VariantKind {
     /// A variant with no parentheses
     ///
