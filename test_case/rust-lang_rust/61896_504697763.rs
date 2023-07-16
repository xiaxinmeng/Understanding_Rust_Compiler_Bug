diff
@@ -5255,12 +5209,8 @@ impl<'a, 'gcx, 'tcx> FnCtxt<'a, 'gcx, 'tcx> {
 
         let tcx = self.tcx;
 
-        let res = match self.rewrite_self_ctor(res, span) {
-            Ok(res) => res,
-            Err(ErrorReported) => return (tcx.types.err, res),
-        };
         let path_segs = match res {
-            Res::Local(_) | Res::Upvar(..) => Vec::new(),
+            Res::Local(_) | Res::Upvar(..) | Res::SelfCtor(_) => vec![],
             Res::Def(kind, def_id) =>
                 AstConv::def_ids_for_value_path_segments(self, segments, self_ty, kind, def_id),
             _ => bug!("instantiate_value_path on {:?}", res),

