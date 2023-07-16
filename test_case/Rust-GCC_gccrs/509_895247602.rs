
diff --git a/gcc/rust/resolve/rust-ast-resolve-item.h b/gcc/rust/resolve/rust-ast-resolve-item.h
index 923faf8b416..bc9aa4a9418 100644
--- a/gcc/rust/resolve/rust-ast-resolve-item.h
+++ b/gcc/rust/resolve/rust-ast-resolve-item.h
@@ -226,13 +226,14 @@ public:
     resolver->push_new_type_rib (resolver->get_type_scope ().peek ());
     resolver->push_new_label_rib (resolver->get_type_scope ().peek ());
 
-    auto path = CanonicalPath::new_seg (module.get_node_id(), module.get_name ());
+    auto path
+      = CanonicalPath::new_seg (module.get_node_id (), module.get_name ());
 
     for (auto &item : module.get_items ())
-      ResolveTopLevel::go (item.get (), path);
+      ResolveTopLevel::go (item.get ());
 
     for (auto &item : module.get_items ())
-      ResolveItem::go(item.get());
+      ResolveItem::go (item.get ());
 
     resolver->get_name_scope ().pop ();
     resolver->get_type_scope ().pop ();
diff --git a/gcc/rust/resolve/rust-ast-resolve-toplevel.h b/gcc/rust/resolve/rust-ast-resolve-toplevel.h
index c28a0820e52..d6ca42018e8 100644
--- a/gcc/rust/resolve/rust-ast-resolve-toplevel.h
+++ b/gcc/rust/resolve/rust-ast-resolve-toplevel.h
@@ -42,8 +42,8 @@ public:
 
   void visit (AST::Module &module) override
   {
-    auto path = prefix.append (CanonicalPath::new_seg (module.get_node_id(),
-                                                       module.get_name()));
+    auto path = prefix.append (
+      CanonicalPath::new_seg (module.get_node_id (), module.get_name ()));
     resolver->get_name_scope ().insert (
       path, module.get_node_id (), module.get_locus (), false,
       [&] (const CanonicalPath &, NodeId, Location locus) -> void {
@@ -53,15 +53,13 @@ public:
       });
 
     resolver->insert_new_definition (module.get_node_id (),
-                                     Definition{
-                                       module.get_node_id (),
-                                       module.get_node_id ()
-                                     });
+                                    Definition{module.get_node_id (),
+                                               module.get_node_id ()});
 
     // This crashes the compiler. Resolving top level is OK, but looks like it's
     // missing parts.
-    // for (auto &item : module.get_items())
-    //   ResolveTopLevel::go (item.get(), path);
+    for (auto &item : module.get_items ())
+      ResolveTopLevel::go (item.get (), path);
   }
 
   void visit (AST::TypeAlias &alias) override
