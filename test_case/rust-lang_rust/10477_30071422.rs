 diff
diff --git a/src/etc/extract-tests.py b/src/etc/extract-tests.py
index 5904e10..ab2556f 100644
--- a/src/etc/extract-tests.py
+++ b/src/etc/extract-tests.py
@@ -64,6 +64,7 @@ while cur < len(lines):
 #[ allow(dead_assignment) ];\n
 #[ allow(unused_mut) ];\n
 #[ allow(attribute_usage) ];\n
+#[ allow(dead_code) ];\n
 #[ feature(macro_rules, globs, struct_variant, managed_boxes) ];\n
 """ + block
             if xfail:
diff --git a/src/librustc/driver/driver.rs b/src/librustc/driver/driver.rs
index f07887c..e08726a 100644
--- a/src/librustc/driver/driver.rs
+++ b/src/librustc/driver/driver.rs
@@ -312,7 +312,7 @@ pub fn phase_3_run_analysis_passes(sess: Session,

     time(time_passes, "death checking", (), |_|
          middle::dead::check_crate(ty_cx, method_map,
-                                   reachable_map, crate));
+                                   &exported_items, reachable_map, crate));

     time(time_passes, "lint checking", (), |_|
          lint::check_crate(ty_cx, &exported_items, crate));
diff --git a/src/librustc/middle/dead.rs b/src/librustc/middle/dead.rs
index 88b0a35..332b706 100644
--- a/src/librustc/middle/dead.rs
+++ b/src/librustc/middle/dead.rs
@@ -196,31 +196,46 @@ impl Visitor<()> for TraitMethodSeeder {
 }

 fn create_and_seed_worklist(tcx: ty::ctxt,
+                            exported_items: &privacy::ExportedItems,
                             reachable_symbols: &HashSet<ast::NodeId>,
                             crate: &ast::Crate) -> ~[ast::NodeId] {
     let mut worklist = ~[];

+    // Preferably, we would only need to seed the worklist with reachable
+    // symbols. However, since the set of reachable symbols differs
+    // depending on whether a crate is built as bin or lib, and we want
+    // the warning to be consistent, we also seed the worklist with
+    // exported symbols.
+    for &id in exported_items.iter() {
+        worklist.push(id);
+    }
     for &id in reachable_symbols.iter() {
         worklist.push(id);
     }
+
+    // Seed entry point
     match *tcx.sess.entry_fn {
         Some((id, _)) => worklist.push(id),
         None => ()
     }
-    let mut trait_method_visitor = TraitMethodSeeder {
+
+    // Seed implemeneted trait methods
+    let mut trait_method_seeder = TraitMethodSeeder {
         worklist: worklist
     };
-    visit::walk_crate(&mut trait_method_visitor, crate, ());
+    visit::walk_crate(&mut trait_method_seeder, crate, ());

-    return trait_method_visitor.worklist;
+    return trait_method_seeder.worklist;
 }

 fn find_live(tcx: ty::ctxt,
              method_map: typeck::method_map,
+             exported_items: &privacy::ExportedItems,
              reachable_symbols: &HashSet<ast::NodeId>,
              crate: &ast::Crate)
              -> ~HashSet<ast::NodeId> {
-    let worklist = create_and_seed_worklist(tcx, reachable_symbols, crate);
+    let worklist = create_and_seed_worklist(tcx, exported_items,
+                                            reachable_symbols, crate);
     let mut symbol_visitor = MarkSymbolVisitor::new(tcx, method_map, worklist);
     symbol_visitor.mark_live_symbols();
     symbol_visitor.live_symbols
@@ -315,13 +330,19 @@ impl Visitor<()> for DeadVisitor {
         }
         visit::walk_block(self, block, ());
     }
+
+    fn visit_trait_method(&mut self, _ :&ast::trait_method, _: ()) {
+        // Don't warn on trait method
+    }
 }

 pub fn check_crate(tcx: ty::ctxt,
                    method_map: typeck::method_map,
                    exported_items: &privacy::ExportedItems,
+                   reachable_symbols: &HashSet<ast::NodeId>,
                    crate: &ast::Crate) {
-    let live_symbols = find_live(tcx, method_map, exported_items, crate);
+    let live_symbols = find_live(tcx, method_map, exported_items,
+                                 reachable_symbols, crate);
     let mut visitor = DeadVisitor { tcx: tcx, live_symbols: live_symbols };
     visit::walk_crate(&mut visitor, crate, ());
 }
