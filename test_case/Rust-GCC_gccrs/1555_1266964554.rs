
diff --git a/gcc/rust/typecheck/rust-hir-trait-ref.h b/gcc/rust/typecheck/rust-hir-trait-ref.h
index 6eec461e8a5..0f4883d6ba3 100644
--- a/gcc/rust/typecheck/rust-hir-trait-ref.h
+++ b/gcc/rust/typecheck/rust-hir-trait-ref.h
@@ -336,6 +336,15 @@ public:
            return true;
          }
       }
+
+    // lookup super traits
+    for (const auto &super_trait : super_traits)
+      {
+       bool found = super_trait->lookup_trait_item (ident, ref);
+       if (found)
+         return true;
+      }
+
     return false;
   }
 
@@ -351,6 +360,16 @@ public:
        if (ident.compare (item.get_identifier ()) == 0)
          return &item;
       }
+
+    // lookup super traits
+    for (const auto &super_trait : super_traits)
+      {
+       const TraitItemReference *res
+         = super_trait->lookup_trait_item (ident, type);
+       if (!res->is_error ())
+         return res;
+      }
+
     return &TraitItemReference::error_node ();
   }
 

