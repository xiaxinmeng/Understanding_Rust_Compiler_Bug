patch
diff --git a/src/libcore/char/methods.rs b/src/libcore/char/methods.rs
index e843303380..6709b9c75b 100644
--- a/src/libcore/char/methods.rs
+++ b/src/libcore/char/methods.rs
@@ -553,9 +553,10 @@ impl char {
     /// 'XID_Start' is a Unicode Derived Property specified in
     /// [UAX #31](http://unicode.org/reports/tr31/#NFKC_Modifications),
     /// mostly similar to `ID_Start` but modified for closure under `NFKx`.
-    #[unstable(feature = "rustc_private",
-               reason = "mainly needed for compiler internals",
-               issue = "27812")]
+    #[cfg_attr(bootstrap,
+               unstable(feature = "rustc_private",
+                        reason = "mainly needed for compiler internals",
+                        issue = "27812"))]
     #[inline]
     pub fn is_xid_start(self) -> bool {
         derived_property::XID_Start(self)
@@ -567,9 +568,10 @@ impl char {
     /// 'XID_Continue' is a Unicode Derived Property specified in
     /// [UAX #31](http://unicode.org/reports/tr31/#NFKC_Modifications),
     /// mostly similar to 'ID_Continue' but modified for closure under NFKx.
-    #[unstable(feature = "rustc_private",
-               reason = "mainly needed for compiler internals",
-               issue = "27812")]
+    #[cfg_attr(bootstrap,
+               unstable(feature = "rustc_private",
+                        reason = "mainly needed for compiler internals",
+                        issue = "27812"))]
     #[inline]
     pub fn is_xid_continue(self) -> bool {
         derived_property::XID_Continue(self)
