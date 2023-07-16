diff
--- a/compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs
+++ b/compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs
@@ -980,6 +986,15 @@ fn build_struct_type_di_node<'ll, 'tcx>(
     let struct_type_and_layout = cx.layout_of(struct_type);
     let variant_def = adt_def.non_enum_variant();

+    let tcx = cx.tcx;
+    let struct_span = tcx.def_span(adt_def.did());
+    let (file_metadata, line_number) = if !struct_span.is_dummy() {
+        let loc = cx.lookup_debug_loc(struct_span.lo());
+        (file_metadata(cx, &loc.file), loc.line)
+    } else {
+        (unknown_file_metadata(cx), UNKNOWN_LINE_NUMBER)
+    };
+
     type_map::build_type_with_children(
         cx,
         type_map::stub(
@@ -987,6 +1002,8 @@ fn build_struct_type_di_node<'ll, 'tcx>(
             Stub::Struct,
             unique_type_id,
             &compute_debuginfo_type_name(cx.tcx, struct_type, false),
+            file_metadata,
+            line_number,
             size_and_align_of(struct_type_and_layout),
             Some(containing_scope),
             DIFlags::FlagZero,
