diff
diff --git a/gcc/rust/util/rust-hir-map.cc b/gcc/rust/util/rust-hir-map.cc
index 9190bd92a43..5b0417b4549 100644
--- a/gcc/rust/util/rust-hir-map.cc
+++ b/gcc/rust/util/rust-hir-map.cc
@@ -750,6 +750,7 @@ Mappings::insert_macro_def (AST::MacroRulesDefinition *macro)
     builtin_macros = {
       {"assert", MacroBuiltin::assert},
       {"file", MacroBuiltin::file},
+      {"column", MacroBuiltin::column},
     };
 
   auto builtin = builtin_macros.find (macro->get_rule_name ());
