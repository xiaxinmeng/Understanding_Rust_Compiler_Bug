patch
diff --git a/compiler/rustc_middle/src/ty/layout.rs b/compiler/rustc_middle/src/ty/layout.rs
index dde55dd96554..833edd228050 100644
--- a/compiler/rustc_middle/src/ty/layout.rs
+++ b/compiler/rustc_middle/src/ty/layout.rs
@@ -1231,11 +1231,15 @@ fn layout_of_uncached(&self, ty: Ty<'tcx>) -> Result<Layout<'tcx>, LayoutError<'
                                 .collect::<Result<IndexVec<VariantIdx, _>, _>>()?;
 
                             let offset = st[i].fields().offset(field_index) + niche.offset;
-                            let size = st[i].size();
+
+                            // Align the total size to the largest alignment.
+                            let size = st[i].size().align_to(align.abi);
 
                             let abi = if st.iter().all(|v| v.abi().is_uninhabited()) {
                                 Abi::Uninhabited
-                            } else {
+                            } else if align == st[i].align() && size == st[i].size() {
+                                // When the total alignment and size match, we can use the
+                                // same ABI as the scalar variant with the reserved niche.
                                 match st[i].abi() {
                                     Abi::Scalar(_) => Abi::Scalar(niche_scalar),
                                     Abi::ScalarPair(first, second) => {
@@ -1249,6 +1253,8 @@ fn layout_of_uncached(&self, ty: Ty<'tcx>) -> Result<Layout<'tcx>, LayoutError<'
                                     }
                                     _ => Abi::Aggregate { sized: true },
                                 }
+                            } else {
+                                Abi::Aggregate { sized: true }
                             };
 
                             let largest_niche = Niche::from_scalar(dl, offset, niche_scalar);
