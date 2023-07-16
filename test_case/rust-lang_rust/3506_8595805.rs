 diff
diff --git a/src/libcore/repr.rs b/src/libcore/repr.rs
index a0d6b01..610cc6f 100644
--- a/src/libcore/repr.rs
+++ b/src/libcore/repr.rs
@@ -881,13 +881,13 @@ impl ReprPrinterWrapper : TyVisitor {
             let mut enum_state = stack.last();
             match enum_state.state {
                 PreVariant => {
-                    let disr_ptr = self.printer.ptr as *int;
-                    if *disr_ptr == disr_val {
+                    //let disr_ptr = self.printer.ptr as *int;
+                    //if *disr_ptr == disr_val {
                         enum_state.state = InVariant;
                         self.printer.writer.write_str(name);
-                        self.printer.bump(sys::size_of::<int>());
+                        //self.printer.bump(sys::size_of::<int>());
                         stack.set_elt(stack.len() - 1, enum_state);
-                    }
+                    //}
                 }
                 InVariant | PostVariant => {}
             }
