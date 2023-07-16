 diff
@@ -740,11 +740,18 @@ fn bind_subslice_pat
     let unit_ty = vec_ty_contents.sequence_element_type(bcx.tcx());
     let vec_datum = match_datum(val, vec_ty);
     let (base, len) = vec_datum.get_vec_base_and_len(bcx);

     let slice_begin = InBoundsGEP(bcx, base, &[C_uint(bcx.ccx(), offset_left)]);
-    let slice_len_offset = C_uint(bcx.ccx(), offset_left + offset_right);
+    let diff = offset_left + offset_right;
+    if let ty::TyArray(ty, n) = vec_ty_contents.sty {
+        let array_ty = bcx.tcx().mk_array(ty, n - diff);
+        let llty_array = type_of::type_of(bcx.ccx(), array_ty);
+        return PointerCast(bcx, slice_begin, llty_array.ptr_to());
+    }
+
+    let slice_len_offset = C_uint(bcx.ccx(), diff);
     let slice_len = Sub(bcx, len, slice_len_offset, DebugLoc::None);
     let slice_ty = bcx.tcx().mk_imm_ref(bcx.tcx().mk_region(ty::ReStatic),
                                          bcx.tcx().mk_slice(unit_ty));
     let scratch = rvalue_scratch_datum(bcx, slice_ty, "");
     Store(bcx, slice_begin, expr::get_dataptr(bcx, scratch.val));
