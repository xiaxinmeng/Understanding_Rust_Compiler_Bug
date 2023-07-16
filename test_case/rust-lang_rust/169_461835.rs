
@@ -368,9 +368,9 @@ let trans_crate
     in
     let base_llty = trans_ty ty in
       match slot.Ast.slot_mode with
-        | Ast.MODE_alias _ ->
+        | Ast.MODE_alias  ->
             Llvm.pointer_type base_llty
-        | Ast.MODE_local _ -> base_llty
+        | Ast.MODE_local  -> base_llty
   in

   let get_element_ptr
