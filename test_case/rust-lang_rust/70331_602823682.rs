
Diff in /checkout/src/librustc_privacy/lib.rs at line 1023:
         span: Span,            // span of the field pattern, e.g., `x: 0`
         def: &'tcx ty::AdtDef, // definition of the struct or enum
         field: &'tcx ty::FieldDef,
-        in_update_syntax: bool
+        in_update_syntax: bool,
     ) {
