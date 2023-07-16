rust
         let move_site_vec = self.get_moved_indexes(context, mpi);
         debug!(
-            "report_use_of_moved_or_uninitialized: mois={:?}",
+            "report_use_of_moved_or_uninitialized: vec<mois,back_edge>={:?}",
             move_site_vec
         );

-        let mois = move_site_vec.clone().into_iter().map(|x| x.moi).collect();
+        let mois = move_site_vec
+            .clone()
+            .into_iter()
+            .map(|x| {
+                let location = self.move_data.moves[x.moi].source;
+                let span = self.mir.source_info(location).span;
+                debug!("moi {:?}: {:?}", x.moi, span);
+                x.moi
+            })
+            .collect();
         if move_site_vec.is_empty() {
             let root_place = self.prefixes(&place, PrefixSet::All).last().unwrap();

@@ -137,12 +146,19 @@ impl<'cx, 'gcx, 'tcx> MirBorrowckCtxt<'cx, 'gcx, 'tcx> {
                     ""
                 };

-                if move_site.traversed_back_edge {
+                // if move_site.traversed_back_edge {
+                //     is_loop_move = true;
+                //     err.span_label(
+                //         span,
+                //         format!("value moved{} here in previous iteration of loop", move_msg),
+                //     );
+                // }
+                if span == move_span {
+                    is_loop_move = true;
                     err.span_label(
                         span,
                         format!("value moved{} here in previous iteration of loop", move_msg),
                     );
-                    is_loop_move = true;
                 } else {
                     err.span_label(move_span, format!("value moved{} here", move_msg));
                     move_spans.var_span_label(&mut err, "variable moved due to use in closure");
