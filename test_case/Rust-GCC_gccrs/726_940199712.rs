
--- gcc/rust/hir/tree/rust-hir-item.h	(original)
+++ gcc/rust/hir/tree/rust-hir-item.h	(reformatted)
@@ -1492,7 +1492,7 @@
 
   void accept_vis (HIRVisitor &vis) override;
 
-  std::vector<StructField>& get_fields() { return fields; }
+  std::vector<StructField> &get_fields () { return fields; }
...

--- gcc/rust/hir/tree/rust-hir-expr.h	(original)
+++ gcc/rust/hir/tree/rust-hir-expr.h	(reformatted)
 @@ -796,7 +796,7 @@
 
   size_t get_num_elements () const { return values.size (); }
 
-  std::vector<std::unique_ptr<Expr>>& get_values () { return values; }
+  std::vector<std::unique_ptr<Expr>> &get_values () { return values; }
 
 protected:
   ArrayElemsValues *clone_array_elems_impl () const override
