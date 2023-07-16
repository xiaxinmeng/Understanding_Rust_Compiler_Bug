
diff --git a/gcc/rust/expand/rust-attribute-visitor.cc b/gcc/rust/expand/rust-attribute-visitor.cc
index 13ae28c0b5e..e9c3f5c6c38 100644
--- a/gcc/rust/expand/rust-attribute-visitor.cc
+++ b/gcc/rust/expand/rust-attribute-visitor.cc
@@ -166,19 +166,22 @@ AttrVisitor::expand_generic_args (AST::GenericArgs &args)
        }
     }
 
-  expander.pop_context ();
-
   // FIXME: Can we have macro invocations in generic type bindings?
   // expand binding args - strip sub-types only
   for (auto &binding : args.get_binding_args ())
     {
       auto &type = binding.get_type ();
+
+      rust_debug_loc (type->get_locus (), "HELLO");
       type->accept_vis (*this);
+      maybe_expand_type (type);
 
       if (type->is_marked_for_strip ())
        rust_error_at (type->get_locus (),
                       "cannot strip type in this position");
     }
+
+  expander.pop_context ();

