 patch
diff --git a/gdb/rust-lang.c b/gdb/rust-lang.c
index 9569584..5376efc 100644
--- a/gdb/rust-lang.c
+++ b/gdb/rust-lang.c
@@ -93,6 +93,28 @@ struct disr_info

 #define RUST_ENCODED_ENUM_HIDDEN 1

+/* Whether or not a TYPE_CODE_UNION value is an untagged union
+   as opposed to being a regular Rust enum.  */
+static bool
+rust_union_is_untagged(struct type *type) {
+  /* Unions must have at least one field.  */
+  if (TYPE_NFIELDS (type) == 0)
+    return false;
+  /* If the first field is named, but the name has the rust enum prefix,
+      it is an enum.  */
+  if (strncmp (TYPE_FIELD_NAME (type, 0), RUST_ENUM_PREFIX,
+         strlen (RUST_ENUM_PREFIX)) == 0)
+    return false;
+  /* Unions only have named fields.  */
+  for (int i = 0; i < TYPE_NFIELDS (type); ++i)
+    {
+      if (strlen (TYPE_FIELD_NAME (type, i)) == 0)
+        return false;
+    }
+    return true;
+
+}
+
 /* Utility function to get discriminant info for a given value.  */

 static struct disr_info
@@ -566,6 +588,14 @@ rust_val_print (struct type *type, const gdb_byte *valaddr, int embedded_offset,
    struct value_print_options opts;
    struct cleanup *cleanup;

+  /* Untagged unions are printed as if they are structs.
+     Since the field bit positions overlap in the debuginfo,
+     the code for printing a union is same as that for a struct,
+     the only difference is that the input type will have overlapping
+     fields.  */
+  if (rust_union_is_untagged (type))
+      goto struct_val;
+
    opts = *options;
    opts.deref_ref = 0;

@@ -638,6 +668,7 @@ rust_val_print (struct type *type, const gdb_byte *valaddr, int embedded_offset,
       break;

     case TYPE_CODE_STRUCT:
+    struct_val:
       {
    int i;
    int first_field;
@@ -809,6 +840,7 @@ rust_print_type (struct type *type, const char *varstring,
       break;

     case TYPE_CODE_STRUCT:
+    struct_printer:
       {
    int is_tuple_struct;

@@ -823,7 +855,12 @@ rust_print_type (struct type *type, const char *varstring,
    if (TYPE_N_BASECLASSES (type) > 0)
      goto c_printer;

-   fputs_filtered ("struct ", stream);
+  /* This code path is also used by unions.  */
+  if (TYPE_CODE (type) == TYPE_CODE_STRUCT)
+     fputs_filtered ("struct ", stream);
+  else
+    fputs_filtered ("union ", stream);
+
    if (TYPE_TAG_NAME (type) != NULL)
      fputs_filtered (TYPE_TAG_NAME (type), stream);

@@ -899,6 +936,13 @@ rust_print_type (struct type *type, const char *varstring,
    /* Skip the discriminant field.  */
    int skip_to = 1;

+  /* Unions and structs have the same syntax in Rust,
+     the only difference is that structs are declared with `struct`
+     and union with `union`. This difference is handled in the struct
+     printer.  */
+  if (rust_union_is_untagged (type))
+    goto struct_printer;
+
    fputs_filtered ("enum ", stream);
    if (TYPE_TAG_NAME (type) != NULL)
      {
@@ -1637,7 +1681,10 @@ rust_evaluate_subexp (struct type *expect_type, struct expression *exp,
         lhs = evaluate_subexp (NULL_TYPE, exp, pos, noside);

         type = value_type (lhs);
-        if (TYPE_CODE (type) == TYPE_CODE_UNION)
+        /* Untagged unions can't have anonymous field access since
+           they can only have named fields.  */
+        if (TYPE_CODE (type) == TYPE_CODE_UNION
+            && !rust_union_is_untagged (type))
      {
        struct cleanup *cleanup;

@@ -1712,8 +1759,8 @@ tuple structs, and tuple-like enum variants"));
         lhs = evaluate_subexp (NULL_TYPE, exp, pos, noside);

         type = value_type (lhs);
-
-        if (TYPE_CODE (type) == TYPE_CODE_UNION)
+        if (TYPE_CODE (type) == TYPE_CODE_UNION
+            && !rust_union_is_untagged (type))
      {
        int i, start;
        struct disr_info disr;
@@ -1762,6 +1809,7 @@ which has only anonymous fields"),
      }
    else
      {
+      /* Field access in structs and untagged unions works like C.  */
        *pos = pc;
        result = evaluate_subexp_standard (expect_type, exp, pos, noside);
      }
