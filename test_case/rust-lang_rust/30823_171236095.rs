 diff
diff --git a/src/librustc_trans/trans/tvec.rs b/src/librustc_trans/trans/tvec.rs
index e5ef3b0..ac638d6 100644
--- a/src/librustc_trans/trans/tvec.rs
+++ b/src/librustc_trans/trans/tvec.rs
@@ -219,7 +219,6 @@ fn write_content<'blk, 'tcx>(bcx: Block<'blk, 'tcx>,
                         bcx = expr::trans_into(bcx, &**element,
                                                SaveIn(lleltptr));
                         let scope = cleanup::CustomScope(temp_scope);
-                        fcx.schedule_lifetime_end(scope, lleltptr);
                         // Issue #30822: mark memory as dropped after running destructor
                         fcx.schedule_drop_and_fill_mem(scope, lleltptr, vt.unit_ty, None);
                     }
