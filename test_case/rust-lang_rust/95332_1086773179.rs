diff
diff --git a/collector/benchmarks/ctfe-stress-4/src/lib.rs b/collector/benchmarks/ctfe-stress-4/src/lib.rs
index 074a18c5..1119b3b1 100644
--- a/collector/benchmarks/ctfe-stress-4/src/lib.rs
+++ b/collector/benchmarks/ctfe-stress-4/src/lib.rs
@@ -11,10 +11,10 @@ use std::mem::MaybeUninit;
 macro_rules! const_repeat {
     // Base case: Use 16 at the end to avoid function calls at the leafs as much as possible.
     ([16] $e: expr, $T: ty) => {{
-        $e; $e; $e; $e;
-        $e; $e; $e; $e;
-        $e; $e; $e; $e;
-        $e; $e; $e; $e
+        unsafe{$e}; unsafe{$e}; unsafe{$e}; unsafe{$e};
+        unsafe{$e}; unsafe{$e}; unsafe{$e}; unsafe{$e};
+        unsafe{$e}; unsafe{$e}; unsafe{$e}; unsafe{$e};
+        unsafe{$e}; unsafe{$e}; unsafe{$e}; unsafe{$e}
     }};
     ([1] $e: expr, $T: ty) => {{
         $e
