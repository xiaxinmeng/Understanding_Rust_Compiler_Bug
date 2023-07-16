 diff
@@ -1221,11 +1221,16 @@ fn compile_submatch_continue<'a, 'p, 'blk, 'tcx>
             }
             _ => {}
         }
         Some(field_vals)
     } else if any_uniq_pat(m, col) || any_region_pat(m, col) {
-        Some(vec!(Load(bcx, val.val)))
+        let ptr = if type_is_fat_ptr(bcx.tcx(), left_ty) {
+            val.val
+        } else {
+            Load(bcx, val.val)
+        };
+        Some(vec!(ptr))
     } else {
         match left_ty.sty {
             ty::TyArray(_, n) => {
                 let args = extract_vec_elems(bcx, left_ty, n, 0, val);
                 Some(args.vals)
