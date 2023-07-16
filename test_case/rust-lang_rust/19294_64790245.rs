 diff
--- a/src/librustc_trans/trans/intrinsic.rs
+++ b/src/librustc_trans/trans/intrinsic.rs
@@ -192,11 +192,8 @@ pub fn trans_intrinsic_call<'a, 'blk, 'tcx>(mut bcx: Block<'blk, 'tcx>,
                     (nonpointer_nonaggregate(in_kind) && nonpointer_nonaggregate(ret_kind)) || {
                         in_kind == TypeKind::Pointer && ret_kind == TypeKind::Pointer
                     };
-                let primitive =
-                    !ty::type_needs_drop(ccx.tcx(), in_type) &&
-                    !ty::type_needs_drop(ccx.tcx(), ret_ty.unwrap());

-                let dest = if bitcast_compatible && primitive {
+                let dest = if bitcast_compatible {
                     // if we're here, the type is scalar-like (a primitive or a
                     // SIMD type), and so can be handled as a by-value ValueRef
                     // and can also be directly bitcast to the target type.
@@ -205,7 +202,11 @@ pub fn trans_intrinsic_call<'a, 'blk, 'tcx>(mut bcx: Block<'blk, 'tcx>,
                     // are done efficiently implicitly in C with the `__m128i`
                     // type and so this means Rust doesn't lose out there).
                     let datum = unpack_datum!(bcx, expr::trans(bcx, &*arg_exprs[0]));
-                    let val = datum.to_llscalarish(bcx);
+                    let val = if datum.kind.is_by_ref() {
+                        load_ty(bcx, datum.val, datum.ty)
+                    } else {
+                        datum.val
+                    };
                     let cast_val = BitCast(bcx, val, llret_ty);

                     match dest {
