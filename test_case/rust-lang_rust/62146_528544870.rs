diff
diff --git a/lexical-core/src/atof/algorithm/bigcomp.rs b/lexical-core/src/atof/algorithm/bigcomp.rs
index 388cec3..db19256 100644
--- a/lexical-core/src/atof/algorithm/bigcomp.rs
+++ b/lexical-core/src/atof/algorithm/bigcomp.rs
@@ -48,7 +48,8 @@ if #[cfg(limb_width_64)] {
 }}   // cfg_if
 
 /// Storage for a big integer type.
-#[derive(Debug, Clone, Default, PartialEq, Eq)]
+//Debug,
+#[derive(Clone, Default, PartialEq, Eq)]
 pub struct Bigint {
     /// Internal storage for the Bigint, in little-endian order.
     ///
diff --git a/lexical-core/src/atof/algorithm/bigint.rs b/lexical-core/src/atof/algorithm/bigint.rs
index d3cbb05..47733da 100644
--- a/lexical-core/src/atof/algorithm/bigint.rs
+++ b/lexical-core/src/atof/algorithm/bigint.rs
@@ -35,7 +35,8 @@ if #[cfg(feature = "radix")] {
 // BIGINT
 
 /// Storage for a big integer type.
-#[derive(Debug, Clone, PartialEq, Eq)]
+//Debug,
+#[derive(Clone, PartialEq, Eq)]
 pub(super) struct Bigint {
     /// Internal storage for the Bigint, in little-endian order.
     data: DataType,
diff --git a/lexical-core/src/atof/algorithm/correct.rs b/lexical-core/src/atof/algorithm/correct.rs
index e6b7802..1b0ae55 100644
--- a/lexical-core/src/atof/algorithm/correct.rs
+++ b/lexical-core/src/atof/algorithm/correct.rs
@@ -38,7 +38,7 @@ macro_rules! rtrim_0 {
 // FLOAT SLICE
 
 /// Substrings and information from parsing the float.
-#[derive(Debug)]
+//#[derive(Debug)]
 pub(super) struct FloatSlice<'a> {
     /// Substring for the integer component of the mantissa.
     integer: &'a [u8],
