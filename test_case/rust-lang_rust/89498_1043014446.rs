diff
diff --git a/compiler/rustc_middle/src/ty/layout.rs b/compiler/rustc_middle/src/ty/layout.rs
index 6d4178c3e75..1fd875c1b7a 100644
--- a/compiler/rustc_middle/src/ty/layout.rs
+++ b/compiler/rustc_middle/src/ty/layout.rs
@@ -3223,6 +3223,15 @@ fn fn_abi_adjust_for_abi(
                         return;
                     }
 
+
+                    Abi::ScalarPair { .. }
+                        if self.tcx.sess.target.arch == "m68k" =>
+                    {
+                        arg.make_indirect();
+                        return;
+                    }
+
+
                     _ => return,
                 }
 
