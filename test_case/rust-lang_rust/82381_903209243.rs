diff
diff --git a/src/librustdoc/html/render/context.rs b/src/librustdoc/html/render/context.rs
index 733bedfdde9..3d29c6021c1 100644
--- a/src/librustdoc/html/render/context.rs
+++ b/src/librustdoc/html/render/context.rs
@@ -61,7 +61,7 @@
     /// Issue for improving the situation: [#82381][]
     ///
     /// [#82381]: https://github.com/rust-lang/rust/issues/82381
-    crate shared: Rc<SharedContext<'tcx>>,
+    crate shared: Box<SharedContext<'tcx>>,
     /// This flag indicates whether `[src]` links should be generated or not. If
     /// the source files are present in the html rendering, then this will be
     /// `true`.
@@ -73,6 +73,7 @@
 rustc_data_structures::static_assert_size!(Context<'_>, 104);
 
 /// Shared mutable state used in [`Context`] and elsewhere.
+// #[derive(Clone)] // TODO: bad
 crate struct SharedContext<'tcx> {
     crate tcx: TyCtxt<'tcx>,
     /// The path to the crate root source minus the file name.
@@ -503,7 +504,7 @@ fn init(
             dst,
             render_redirect_pages: false,
             id_map: RefCell::new(id_map),
-            shared: Rc::new(scx),
+            shared: Box::new(scx),
             include_sources,
         };
 
@@ -512,12 +513,12 @@ fn init(
         }
 
         // Build our search index
-        let index = build_index(&krate, &mut Rc::get_mut(&mut cx.shared).unwrap().cache, tcx);
+        let index = build_index(&krate, &mut cx.shared.cache, tcx);
 
         // Write shared runs within a flock; disable thread dispatching of IO temporarily.
-        Rc::get_mut(&mut cx.shared).unwrap().fs.set_sync_only(true);
+        cx.shared.fs.set_sync_only(true);
         write_shared(&cx, &krate, index, &md_opts)?;
-        Rc::get_mut(&mut cx.shared).unwrap().fs.set_sync_only(false);
+        cx.shared.fs.set_sync_only(false);
         Ok((cx, krate))
     }
 
@@ -527,7 +528,9 @@ fn make_child_renderer(&self) -> Self {
             dst: self.dst.clone(),
             render_redirect_pages: self.render_redirect_pages,
             id_map: RefCell::new(IdMap::new()),
-            shared: Rc::clone(&self.shared),
+            // TODO: this clone is stupid expensive
+            shared: todo!(),
+            // shared: self.shared.clone(),
             include_sources: self.include_sources,
         }
     }
@@ -608,7 +611,7 @@ fn after_krate(&mut self) -> Result<(), Error> {
         }
 
         // Flush pending errors.
-        Rc::get_mut(&mut self.shared).unwrap().fs.close();
+        self.shared.fs.close();
         let nb_errors =
             self.shared.errors.iter().map(|err| self.tcx().sess.struct_err(&err).emit()).count();
         if nb_errors > 0 {
