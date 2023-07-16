 patch
diff --git a/gdb/rust-lang.c b/gdb/rust-lang.c
index 9d13353..9569584 100644
--- a/gdb/rust-lang.c
+++ b/gdb/rust-lang.c
@@ -1736,7 +1736,9 @@ tuple structs, and tuple-like enum variants"));
        variant_type = TYPE_FIELD_TYPE (type, disr.field_no);

        if (variant_type == NULL
-       || rust_tuple_variant_type_p (variant_type))
+           || (disr.is_encoded
+               ? rust_tuple_struct_type_p (variant_type)
+               : rust_tuple_variant_type_p (variant_type)))
          error(_("Attempting to access named field %s of tuple variant %s, \
 which has only anonymous fields"),
            field_name, disr.name);
